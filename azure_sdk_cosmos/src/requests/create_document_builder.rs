use crate::client2::{CosmosUriBuilder, ResourceType};
use crate::document_attributes::DocumentAttributes;
use crate::prelude::*;
use crate::request_response::ListDocumentsResponse;
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
use serde::Serialize;
use serde_json::Value;
use std::convert::TryFrom;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    p_document: PhantomData<DocumentSet>,
    document: Option<&'b T>,
    is_upsert: bool,
    indexing_directive: IndexingDirective,
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

impl<'a, 'b, T, CUB> CreateDocumentBuilder<'a, 'b, T, CUB, No>
where
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> CreateDocumentBuilder<'a, 'b, T, CUB, No> {
        CreateDocumentBuilder {
            collection_client,
            p_document: PhantomData {},
            document: None,
            is_upsert: false,
            indexing_directive: IndexingDirective::Default,
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

impl<'a, 'b, T, CUB, DocumentSet> CollectionClientRequired<'a, CUB>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

impl<'a, 'b, T, CUB> DocumentRequired<'b, T> for CreateDocumentBuilder<'a, 'b, T, CUB, Yes>
where
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn document(&self) -> &'b T {
        self.document.unwrap()
    }
}

impl<'a, 'b, T, CUB, DocumentSet> IsUpsertOption
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn is_upsert(&self) -> bool {
        self.is_upsert
    }
}

impl<'a, 'b, T, CUB, DocumentSet> IndexingDirectiveOption
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn indexing_directive(&self) -> IndexingDirective {
        self.indexing_directive
    }
}

impl<'a, 'b, T, CUB, DocumentSet> IfMatchConditionOption<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, T, CUB, DocumentSet> IfModifiedSinceOption<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, T, CUB, DocumentSet> UserAgentOption<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, T, CUB, DocumentSet> ActivityIdOption<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, T, CUB, DocumentSet> ConsistencyLevelOption<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, T, CUB, DocumentSet> PartitionKeysOption<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_keys(&self) -> Option<&'b [&'b str]> {
        self.partition_keys
    }
}

impl<'a, 'b, T, CUB, DocumentSet> QueryCrossPartitionOption
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn query_cross_partition(&self) -> bool {
        self.query_cross_partition
    }
}

impl<'a, 'b, T, CUB, DocumentSet> AIMOption for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn a_im(&self) -> bool {
        self.a_im
    }
}

impl<'a, 'b, T, CUB, DocumentSet> PartitionRangeIdOption<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_range_id(&self) -> Option<&'b str> {
        self.partition_range_id
    }
}

impl<'a, 'b, T, CUB> DocumentSupport<'b, T> for CreateDocumentBuilder<'a, 'b, T, CUB, No>
where
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, Yes>;

    #[inline]
    fn with_document(self, document: &'b T) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: Some(document),
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> IsUpsertSupport
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_is_upsert(self, is_upsert: bool) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> IndexingDirectiveSupport
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_indexing_directive(self, indexing_directive: IndexingDirective) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> IfMatchConditionSupport<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> IfModifiedSinceSupport<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> UserAgentSupport<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> ActivityIdSupport<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> ConsistencyLevelSupport<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> PartitionKeysSupport<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_partition_keys(self, partition_keys: &'b [&'b str]) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> QueryCrossPartitionSupport
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> AIMSupport for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_a_im(self, a_im: bool) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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

impl<'a, 'b, T, CUB, DocumentSet> PartitionRangeIdSupport<'b>
    for CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    type O = CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>;

    #[inline]
    fn with_partition_range_id(self, partition_range_id: &'b str) -> Self::O {
        CreateDocumentBuilder {
            collection_client: self.collection_client,
            p_document: PhantomData {},
            document: self.document,
            is_upsert: self.is_upsert,
            indexing_directive: self.indexing_directive,
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
impl<'a, 'b, T, CUB, DocumentSet> CreateDocumentBuilder<'a, 'b, T, CUB, DocumentSet>
where
    DocumentSet: ToAssign,
    T: Serialize,
    CUB: CosmosUriBuilder,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, T, CUB> CreateDocumentBuilder<'a, 'b, T, CUB, Yes>
where
    T: Serialize,
    CUB: CosmosUriBuilder,
{
    async fn perform_request(&self) -> Result<(hyper::HeaderMap, hyper::Chunk), AzureError> {
        let mut req = self.collection_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database(),
                self.collection_client.collection()
            ),
            hyper::Method::POST,
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
        IsUpsertOption::add_header(self, &mut req);
        IndexingDirectiveOption::add_header(self, &mut req);

        let serialized = serde_json::to_string(self.document())?;

        let req = req.body(hyper::Body::from(serialized))?;

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.collection_client.hyper_client().request(req),
            StatusCode::CREATED,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((headers, whole_body))
    }

    pub async fn execute(&self) -> Result<ListDocumentsResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        let (headers, whole_body) = self.perform_request().await?;
        let da = DocumentAttributes::try_from((&headers, &whole_body as &[u8]))?;
        let resp = ListDocumentsResponse::try_from((&headers, &whole_body as &[u8]))?;
        Ok(resp)
    }
}
