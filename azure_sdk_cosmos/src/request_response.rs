use crate::{
    client::headers::HEADER_REQUEST_CHARGE, collection::Collection, database::Database,
    document::DocumentAttributes, request_charge_from_headers, request_item_count_from_headers,
};
use azure_sdk_core::{
    continuation_token_from_headers_optional, errors::AzureError, etag_from_headers_optional,
    session_token_from_headers, util::HeaderMapExt, SessionToken,
};
use http::header::HeaderMap;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListDatabasesResponse {
    _rid: String,
    #[serde(rename = "Databases")]
    pub databases: Vec<Database>,
    #[serde(rename = "_count")]
    pub count: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ListCollectionsResponse {
    _rid: String,
    #[serde(rename = "DocumentCollections")]
    pub collections: Vec<Collection>,
    #[serde(rename = "_count")]
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentsResponseAttributes {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub documents: Vec<DocumentAttributes>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDocumentsResponseEntities<T> {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub entities: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document<T> {
    pub document_attributes: DocumentAttributes,
    pub entity: T,
}

impl<T: DeserializeOwned> Document<T> {
    pub(crate) fn from_json(json: &[u8]) -> Result<Document<T>, AzureError> {
        Ok(Document {
            document_attributes: ::serde_json::from_slice::<DocumentAttributes>(json)?,
            entity: ::serde_json::from_slice::<T>(json)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult<T> {
    pub document_attributes: Option<DocumentAttributes>,
    pub result: T,
}

#[derive(Debug, Clone)]
pub struct ListDocumentsResponseAdditionalHeaders {
    pub continuation_token: Option<String>,
    pub charge: f64,
    pub etag: Option<String>,
    pub session_token: SessionToken,
    pub item_count: u64,
}

#[derive(Debug, Clone)]
pub struct QueryDocumentResponseAdditonalHeaders {
    pub continuation_token: Option<String>,
    pub charge: f64,
}

#[derive(Debug, Clone)]
pub struct QueryDocumentResponse<T> {
    pub query_response_meta: QueryResponseMeta,
    pub results: Vec<QueryResult<T>>,
    pub additional_headers: QueryDocumentResponseAdditonalHeaders,
}

#[derive(Debug, Clone)]
pub struct ListDocumentsResponse<T> {
    pub rid: String,
    pub documents: Vec<Document<T>>,
    pub additional_headers: ListDocumentsResponseAdditionalHeaders,
}

#[derive(Debug, Clone)]
pub struct DocumentAdditionalHeaders {
    pub charge: f64,
}

impl DocumentAdditionalHeaders {
    pub(crate) fn derive_from(headers: &::hyper::HeaderMap) -> DocumentAdditionalHeaders {
        DocumentAdditionalHeaders {
            charge: headers
                .get_as_str(HEADER_REQUEST_CHARGE)
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetDocumentResponse<T> {
    pub document: Option<Document<T>>,
    pub additional_headers: DocumentAdditionalHeaders,
}

#[derive(Debug, Clone)]
pub struct ReplaceDocumentResponse<T> {
    pub document: Document<T>,
    pub additional_headers: DocumentAdditionalHeaders,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryResponseMeta {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_count")]
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureResponse<T> {
    pub result: T,
    pub additional_headers: DocumentAdditionalHeaders,
}

impl std::convert::TryFrom<&HeaderMap> for ListDocumentsResponseAdditionalHeaders {
    type Error = AzureError;
    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:?}", headers);

        let ado = ListDocumentsResponseAdditionalHeaders {
            continuation_token: continuation_token_from_headers_optional(headers)?,
            charge: request_charge_from_headers(headers)?,
            etag: etag_from_headers_optional(headers)?,
            session_token: session_token_from_headers(headers)?,
            item_count: request_item_count_from_headers(headers)?,
        };
        debug!("ado == {:?}", ado);

        Ok(ado)
    }
}

impl std::convert::TryFrom<&[u8]> for ListDocumentsResponseAttributes {
    type Error = AzureError;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(body)?)
    }
}

impl<T> std::convert::TryFrom<&[u8]> for ListDocumentsResponseEntities<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(body)?)
    }
}

impl ListDocumentsResponseEntities<serde_json::Value> {
    pub(crate) fn to_json(body: &[u8]) -> Result<Self, AzureError> {
        debug!(
            "\nListDocumentsResponseEntities<serde_json::Value>::to_json({}) called",
            std::str::from_utf8(body)?
        );

        use serde_json::{from_slice, Value};
        let val: Value = from_slice(body)?;

        let rid = match &val["_rid"] {
            Value::String(rid) => rid,
            _ => {
                return Err(AzureError::MissingValueError(
                    "_rid".to_owned(),
                    "String".to_owned(),
                ))
            }
        };

        if let Value::Array(documents) = &val["Documents"] {
            Ok(ListDocumentsResponseEntities {
                rid: rid.to_owned(),
                entities: documents.to_vec(),
            })
        } else {
            return Err(AzureError::MissingValueError(
                "_rid".to_owned(),
                "Array".to_owned(),
            ));
        }
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for ListDocumentsResponse<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;
        debug!("headers == {:?}", headers);

        let ado = ListDocumentsResponseAdditionalHeaders::try_from(headers)?;

        // we will proceed in three steps:
        // 1- Deserialize the result as DocumentAttributes. The extra field will be ignored.
        // 2- Deserialize the result a type T. The extra fields will be ignored.
        // 3- Zip 1 and 2 in the resulting structure.
        // There is a lot of data movement here, let's hope the compiler is smarter than me :)
        let document_attributes = ListDocumentsResponseAttributes::try_from(body)?;
        debug!("document_attributes == {:?}", document_attributes);
        let entries = ListDocumentsResponseEntities::try_from(body)?;

        let documents = document_attributes
            .documents
            .into_iter()
            .zip(entries.entities.into_iter())
            .map(|(da, e)| Document {
                document_attributes: da,
                entity: e,
            })
            .collect();

        Ok(ListDocumentsResponse {
            rid: document_attributes.rid,
            documents,
            additional_headers: ado,
        })
    }
}

impl ListDocumentsResponse<serde_json::Value> {
    pub(crate) fn to_json(value: (&HeaderMap, &[u8])) -> Result<Self, AzureError> {
        let headers = value.0;
        let body = value.1;
        debug!("headers == {:?}", headers);

        let ado = ListDocumentsResponseAdditionalHeaders::try_from(headers)?;

        // we will proceed in three steps:
        // 1- Deserialize the result as DocumentAttributes. The extra field will be ignored.
        // 2- Deserialize the result a type T. The extra fields will be ignored.
        // 3- Zip 1 and 2 in the resulting structure.
        // There is a lot of data movement here, let's hope the compiler is smarter than me :)
        let document_attributes = ListDocumentsResponseAttributes::try_from(body)?;
        debug!("document_attributes == {:?}", document_attributes);
        let entries = ListDocumentsResponseEntities::to_json(body)?;
        debug!("\n\nentries == {:?}\n\n", entries);

        let documents = document_attributes
            .documents
            .into_iter()
            .zip(entries.entities.into_iter())
            .map(|(da, e)| Document {
                document_attributes: da,
                entity: e,
            })
            .collect();

        Ok(ListDocumentsResponse {
            rid: document_attributes.rid,
            documents,
            additional_headers: ado,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &'static str = "
{
    \"_rid\": \"3iNTAJKxVCk=\",
    \"Documents\": [
        {
            \"color\": \"red\",
            \"myvalue\": \"#f00\",
            \"id\": \"c5d11a65-2e5a-3d9f-4de8-2447259dff38\",
            \"_rid\": \"3iNTAJKxVCkBAAAAAAAAAA==\",
            \"_self\": \"dbs/3iNTAA==/colls/3iNTAJKxVCk=/docs/3iNTAJKxVCkBAAAAAAAAAA==/\",
            \"_etag\": \"\\\"0100eb0a-0000-0c00-0000-5ded4fe30000\\\"\",
            \"_attachments\": \"attachments/\",
            \"_ts\": 1575833571
        },
        {
            \"color\": \"yellow\",
            \"myvalue\": \"#ff0\",
            \"id\": \"894dd5ff-573e-f38a-b8c4-5eae5071c900\",
            \"_rid\": \"3iNTAJKxVCkCAAAAAAAAAA==\",
            \"_self\": \"dbs/3iNTAA==/colls/3iNTAJKxVCk=/docs/3iNTAJKxVCkCAAAAAAAAAA==/\",
            \"_etag\": \"\\\"0100ec0a-0000-0c00-0000-5ded4fe30000\\\"\",
            \"_attachments\": \"attachments/\",
            \"_ts\": 1575833571
        }
    ],
    \"_count\": 7
}";

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct MyStruct {
        id: String,
        color: String,
        myvalue: String,
    }

    #[test]
    fn test_list_document() {
        let document_attributes =
            serde_json::from_slice::<ListDocumentsResponseAttributes>(BODY.as_bytes()).unwrap();
        let entries =
            serde_json::from_slice::<ListDocumentsResponseEntities<MyStruct>>(BODY.as_bytes())
                .unwrap();
    }
}
