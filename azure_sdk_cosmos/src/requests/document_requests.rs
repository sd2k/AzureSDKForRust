use super::*;
use crate::responses::DocumentAdditionalHeaders;
use crate::IndexingDirective;
use std::convert::TryFrom;

pub fn request_charge_from_headers(headers: &HeaderMap) -> Result<f64, AzureError> {
    Ok(headers
        .get(HEADER_REQUEST_CHARGE)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_REQUEST_CHARGE.to_owned()))?
        .to_str()?
        .parse()?)
}

pub fn request_item_count_from_headers(headers: &HeaderMap) -> Result<u64, AzureError> {
    Ok(headers
        .get(HEADER_ITEM_COUNT)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_ITEM_COUNT.to_owned()))?
        .to_str()?
        .parse()?)
}

pub fn number_of_read_regions_from_headers(headers: &HeaderMap) -> Result<u32, AzureError> {
    Ok(headers
        .get(HEADER_NUMBER_OF_READ_REGIONS)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_NUMBER_OF_READ_REGIONS.to_owned()))?
        .to_str()?
        .parse()?)
}

pub struct CreateDocumentRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
    payload: Result<String, serde_json::Error>,
}

impl DocumentRequestExt for CreateDocumentRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

impl CreateDocumentRequest {
    pub(crate) fn new(
        hyper_client: HyperClient,
        request: RequestBuilder,
        payload: Result<String, serde_json::Error>,
    ) -> CreateDocumentRequest {
        CreateDocumentRequest {
            hyper_client,
            request,
            payload,
        }
    }

    request_option!(upsert, bool, HEADER_DOCUMENTDB_IS_UPSERT);
    request_option!(
        indexing_directive,
        IndexingDirective,
        HEADER_INDEXING_DIRECTIVE
    );
    request_bytes_ref!(partition_key, HEADER_DOCUMENTDB_PARTITIONKEY);
    request_option!(
        use_multiple_write_locations,
        bool,
        HEADER_ALLOW_MULTIPLE_WRITES
    );

    pub async fn execute(self) -> Result<DocumentAttributes, AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        let payload = self.payload?;
        let r = req.body(payload.into())?;
        let body = check_status_extract_body(hc.request(r), StatusCode::CREATED).await?;
        Ok(serde_json::from_str::<DocumentAttributes>(&body)?)
    }
}

pub struct QueryDocumentRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
    payload: Result<String, serde_json::Error>,
}

impl DocumentRequestExt for QueryDocumentRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

const QUERY_CONTENT_TYPE: &str = "application/query+json";

impl QueryDocumentRequest {
    pub(crate) fn new(
        hyper_client: HyperClient,
        mut request: RequestBuilder,
        payload: Result<String, serde_json::Error>,
    ) -> QueryDocumentRequest {
        request
            .header(HEADER_DOCUMENTDB_ISQUERY, HeaderValue::from_static("true"))
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static(QUERY_CONTENT_TYPE),
            );
        QueryDocumentRequest {
            hyper_client,
            request,
            payload,
        }
    }

    request_option!(max_item_count, u64, HEADER_MAX_ITEM_COUNT);
    request_bytes_ref!(continuation_token, HEADER_CONTINUATION);
    request_option!(
        enable_cross_partition,
        bool,
        HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION
    );
    request_option!(
        enable_parallelize_cross_partition_query,
        bool,
        HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY
    );
    request_option!(
        use_multiple_write_locations,
        bool,
        HEADER_ALLOW_MULTIPLE_WRITES
    );
    //request_option!(
    //    consistency_level,
    //    ConsistencyLevel,
    //    HEADER_CONSISTENCY_LEVEL
    //);

    pub async fn execute<T: DeserializeOwned>(
        self,
    ) -> Result<QueryDocumentResponse<T>, AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        let p = self.execute_json().await?;
        Self::convert_query_document_type(p)
    }

    pub async fn execute_json(
        self,
    ) -> Result<QueryDocumentResponse<serde_json::Value>, AzureError> {
        trace!("query_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        let payload = self.payload?;
        let r = req.body(payload.into())?;
        let (headers, body) =
            check_status_extract_headers_and_body(hc.request(r), StatusCode::OK).await?;
        Self::extract_result_json(&body, &headers)
    }

    fn extract_result_json(
        body: &[u8],
        headers: &HeaderMap,
    ) -> Result<QueryDocumentResponse<serde_json::Value>, AzureError> {
        trace!("headers == {:?}", headers);

        let additional_headers = QueryDocumentResponseAdditonalHeaders {
            // This match just tries to extract the info and convert it
            // into the correct type. It is complicated because headers
            // can be missing and also because headers.get<T> will return
            // a T reference (&T) so we need to cast it into the
            // correct type and clone it (in this case into a &str that will
            // become a String using to_owned())
            continuation_token: derive_continuation_token(headers),
            // Here we assume the Charge header to always be present.
            // If problems arise we
            // will change the field to be Option(al).
            charge: derive_request_charge(headers),
        };
        debug!("additional_headers == {:?}", additional_headers);

        let query_response_meta = serde_json::from_slice::<QueryResponseMeta>(body)?;
        debug!("query_response_meta == {:?}", &query_response_meta);

        let json = str::from_utf8(body)?;
        debug!("json == {}", json);

        let mut v: serde_json::Value = serde_json::from_slice(body)?;

        // Work on Documents section
        let mut d = v.get_mut("Documents").unwrap().take();
        debug!("\n\nd == {:?}\n\n", d);

        let docs = d.as_array_mut().unwrap().iter_mut().map(|doc| {
            // We could either have a Document or a plain entry.
            // We will find out here.
            let mut doc = doc.take();

            let attrs = {
                if let Some(map) = doc.as_object_mut() {
                    DocumentAttributes::try_extract(map)
                } else {
                    None
                }
            };

            debug!("attrs == {:?}", attrs);

            QueryResult {
                document_attributes: attrs,
                result: doc,
            }
        });

        Ok(QueryDocumentResponse {
            query_response_meta,
            additional_headers,
            results: docs.collect(),
        })
    }

    #[inline]
    fn convert_query_document_type<T>(
        qdr: QueryDocumentResponse<serde_json::Value>,
    ) -> Result<QueryDocumentResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        let mut qdr_converted: QueryDocumentResponse<T> = QueryDocumentResponse {
            query_response_meta: qdr.query_response_meta,
            results: Vec::new(),
            additional_headers: qdr.additional_headers,
        };

        for res_json in qdr.results {
            qdr_converted.results.push(QueryResult {
                document_attributes: res_json.document_attributes,
                result: serde_json::from_value(res_json.result)?,
            });
        }

        Ok(qdr_converted)
    }
}

pub struct ReplaceDocumentRequest<T> {
    hyper_client: HyperClient,
    request: RequestBuilder,
    payload: Result<String, serde_json::Error>,
    _t: PhantomData<T>,
}

impl<T> DocumentRequestExt for ReplaceDocumentRequest<T> {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

impl<T: DeserializeOwned> ReplaceDocumentRequest<T> {
    pub(crate) fn new(
        hyper_client: HyperClient,
        request: RequestBuilder,
        payload: Result<String, serde_json::Error>,
    ) -> ReplaceDocumentRequest<T> {
        ReplaceDocumentRequest {
            hyper_client,
            request,
            payload,
            _t: PhantomData,
        }
    }

    request_bytes_ref!(if_match, header::IF_MATCH);
    request_option!(
        indexing_directive,
        IndexingDirective,
        HEADER_INDEXING_DIRECTIVE
    );
    request_bytes_ref!(partition_key, HEADER_DOCUMENTDB_PARTITIONKEY);
    request_option!(
        use_multiple_write_locations,
        bool,
        HEADER_ALLOW_MULTIPLE_WRITES
    );

    pub async fn execute(self) -> Result<ReplaceDocumentResponse<T>, AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        let payload = self.payload?;
        let r = req.body(payload.into())?;
        let (headers, body) =
            check_status_extract_headers_and_body(hc.request(r), StatusCode::OK).await?;
        Self::extract_result(&headers, &body)
    }

    fn extract_result<R: DeserializeOwned>(
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<ReplaceDocumentResponse<R>, AzureError> {
        let additional_headers = DocumentAdditionalHeaders::try_from(headers)?;
        let document = Document::try_from((headers, body))?;
        Ok(ReplaceDocumentResponse {
            document,
            additional_headers,
        })
    }
}

pub struct DeleteDocumentRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
}

impl DocumentRequestExt for DeleteDocumentRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

impl DeleteDocumentRequest {
    pub(crate) fn new(hyper_client: HyperClient, request: RequestBuilder) -> DeleteDocumentRequest {
        DeleteDocumentRequest {
            hyper_client,
            request,
        }
    }

    request_bytes_ref!(if_match, header::IF_MATCH);
    request_bytes_ref!(partition_key, HEADER_DOCUMENTDB_PARTITIONKEY);
    request_option!(
        use_multiple_write_locations,
        bool,
        HEADER_ALLOW_MULTIPLE_WRITES
    );

    pub async fn execute(mut self) -> Result<(), AzureError> {
        trace!("get_document called(request == {:?}", self.request);

        let r = self.request.body(hyper::Body::empty())?;
        check_status_extract_body(self.hyper_client.request(r), StatusCode::NO_CONTENT).await?;
        Ok(())
    }
}

pub trait DocumentRequestExt: Sized {
    fn request(&mut self) -> &mut RequestBuilder;

    fn session_token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.request()
            .header_formatted(HEADER_SESSION_TOKEN, token.as_ref());
        self
    }

    fn partition_key<'a, P: Into<PartitionKey<'a>>>(mut self, key: P) -> Self {
        // todo: move unwrap into PartitionKey impl itself as we control the impl and it surely won't error out
        if let Some(ser_key) = key.into().to_json().unwrap() {
            self.request()
                .header_formatted(HEADER_DOCUMENTDB_PARTITIONKEY, ser_key);
        }
        self
    }
}

fn derive_continuation_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(HEADER_CONTINUATION)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_owned())
}

fn derive_request_charge(headers: &HeaderMap) -> f64 {
    headers
        .get(HEADER_REQUEST_CHARGE)
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {}
