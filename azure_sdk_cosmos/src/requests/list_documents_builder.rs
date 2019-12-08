use crate::client2::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::request_response::ListDocumentsResponse;
use crate::CollectionClient;
use crate::CollectionClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use chrono::{DateTime, Utc};
use hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    continuation: Option<&'b str>,
    max_item_count: i32,
    partition_key: Option<&'b [&'b str]>,
    query_cross_partition: bool,
    a_im: bool,
    partition_range_id: Option<&'b str>,
}

impl<'a, 'b, CUB> ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> ListDocumentsBuilder<'a, 'b, CUB> {
        ListDocumentsBuilder {
            collection_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
            partition_key: None,
            query_cross_partition: false,
            a_im: false,
            partition_range_id: None,
        }
    }
}

impl<'a, 'b, CUB> CollectionClientRequired<'a, CUB> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

impl<'a, 'b, CUB> IfMatchConditionOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB> IfModifiedSinceOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, CUB> UserAgentOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, CUB> ContinuationOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b, CUB> MaxItemCountOption for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, CUB> PartitionKeyOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_key(&self) -> Option<&'b [&'b str]> {
        self.partition_key
    }
}

impl<'a, 'b, CUB> QueryCrossPartitionOption for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn query_cross_partition(&self) -> bool {
        self.query_cross_partition
    }
}

impl<'a, 'b, CUB> AIMOption for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn a_im(&self) -> bool {
        self.a_im
    }
}

impl<'a, 'b, CUB> PartitionRangeIdOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_range_id(&self) -> Option<&'b str> {
        self.partition_range_id
    }
}

impl<'a, 'b, CUB> IfMatchConditionSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> IfModifiedSinceSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> ContinuationSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_continuation(self, continuation: &'b str) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> MaxItemCountSupport for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> PartitionKeySupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_partition_key(self, partition_key: &'b [&'b str]) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: Some(partition_key),
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> QueryCrossPartitionSupport for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition,
            a_im: self.a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> AIMSupport for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_a_im(self, a_im: bool) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im,
            partition_range_id: self.partition_range_id,
        }
    }
}

impl<'a, 'b, CUB> PartitionRangeIdSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_partition_range_id(self, partition_range_id: &'b str) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            partition_key: self.partition_key,
            query_cross_partition: self.query_cross_partition,
            a_im: self.a_im,
            partition_range_id: Some(partition_range_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> ListDocumentsBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn finalize<T>(&self) -> Result<ListDocumentsResponse<T>, AzureError> {
        let req = self
            .collection_client
            .main_client()
            .prepare_request(
                &format!(
                    "dbs/{}/colls/{}/docs",
                    self.collection_client.database(),
                    self.collection_client.collection()
                ),
                hyper::Method::GET,
                ResourceType::Documents,
            )
            .body(hyper::Body::empty())?;
        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.collection_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;
    }
}
