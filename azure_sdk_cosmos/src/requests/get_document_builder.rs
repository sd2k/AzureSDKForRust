use crate::client2::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::request_response::GetDocumentResponse;
use crate::CollectionClient;
use crate::CollectionClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use azure_sdk_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    p_document_id: PhantomData<DocumentIdSet>,
    document_id: &'b str,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    partition_keys: Option<&'b [&'b str]>,
    query_cross_partition: bool,
    a_im: bool,
    partition_range_id: Option<&'b str>,
}

impl<'a, 'b, CUB> GetDocumentBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> GetDocumentBuilder<'a, 'b, CUB, No> {
        GetDocumentBuilder {
            collection_client,
            p_document_id: PhantomData {},
            document_id: "",
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            partition_keys: None,
            query_cross_partition: false,
            a_im: false,
            partition_range_id: None,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> CollectionClientRequired<'a, CUB>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

impl<'a, 'b, CUB> DocumentIdRequired<'b> for GetDocumentBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn document_id(&self) -> &'b str {
        self.document_id
    }
}

impl<'a, 'b, CUB, DocumentIdSet> IfMatchConditionOption<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB, DocumentIdSet> IfModifiedSinceOption<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, CUB, DocumentIdSet> UserAgentOption<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB, DocumentIdSet> ActivityIdOption<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB, DocumentIdSet> ConsistencyLevelOption<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, CUB, DocumentIdSet> PartitionKeysOption<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_keys(&self) -> Option<&'b [&'b str]> {
        self.partition_keys
    }
}

impl<'a, 'b, CUB, DocumentIdSet> QueryCrossPartitionOption
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn query_cross_partition(&self) -> bool {
        self.query_cross_partition
    }
}

impl<'a, 'b, CUB, DocumentIdSet> AIMOption for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn a_im(&self) -> bool {
        self.a_im
    }
}

impl<'a, 'b, CUB, DocumentIdSet> PartitionRangeIdOption<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_range_id(&self) -> Option<&'b str> {
        self.partition_range_id
    }
}

impl<'a, 'b, CUB> DocumentIdSupport<'b> for GetDocumentBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, Yes>;

    #[inline]
    fn with_document_id(self, document_id: &'b str) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> IfMatchConditionSupport<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> IfModifiedSinceSupport<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> UserAgentSupport<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> ActivityIdSupport<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> ConsistencyLevelSupport<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> PartitionKeysSupport<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_partition_keys(self, partition_keys: &'b [&'b str]) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            partition_keys: Some(partition_keys),
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> QueryCrossPartitionSupport
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            partition_keys: self.partition_keys,
            query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> AIMSupport for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_a_im(self, a_im: bool) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB, DocumentIdSet> PartitionRangeIdSupport<'b>
    for GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>;

    #[inline]
    fn with_partition_range_id(self, partition_range_id: &'b str) -> Self::O {
        GetDocumentBuilder {
            collection_client: self.collection_client,
            p_document_id: PhantomData {},
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            partition_keys: self.partition_keys,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: Some(partition_range_id),
        }
    }
}

// methods callable regardless
impl<'a, 'b, CUB, DocumentIdSet> GetDocumentBuilder<'a, 'b, CUB, DocumentIdSet>
where
    DocumentIdSet: ToAssign,
    CUB: CosmosUriBuilder,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> GetDocumentBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    async fn perform_request(&self) -> Result<(hyper::HeaderMap, hyper::Chunk), AzureError> {
        let mut req = self.collection_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.collection_client.database(),
                self.collection_client.collection(),
                self.document_id()
            ),
            hyper::Method::GET,
            ResourceType::Documents,
        );

        // add trait headers
        IfMatchConditionOption::add_header(self, &mut req);
        IfModifiedSinceOption::add_header(self, &mut req);
        UserAgentOption::add_header(self, &mut req);
        ActivityIdOption::add_header(self, &mut req);
        ConsistencyLevelOption::add_header(self, &mut req);
        PartitionKeysOption::add_header(self, &mut req);
        QueryCrossPartitionOption::add_header(self, &mut req);
        AIMOption::add_header(self, &mut req);
        PartitionRangeIdOption::add_header(self, &mut req);

        let req = req.body(hyper::Body::empty())?;

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.collection_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        println!("\nheaders == {:?}", headers);
        println!("\nwhole body == {:#?}", whole_body);

        Ok((headers, whole_body))
    }

    pub async fn get_as_entity<T>(&self) -> Result<GetDocumentResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        let (headers, whole_body) = self.perform_request().await?;
        let resp = GetDocumentResponse::try_from((&headers, &whole_body as &[u8]))?;
        Ok(resp)
    }

    pub async fn get_as_json(&self) -> Result<GetDocumentResponse<serde_json::Value>, AzureError> {
        let (headers, whole_body) = self.perform_request().await?;
        let resp = GetDocumentResponse::new_json((&headers, &whole_body as &[u8]))?;
        Ok(resp)
    }
}
