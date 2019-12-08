use crate::client2::{CosmosUriBuilder, ResourceType};
use crate::request_response::{Document, ListCollectionsResponse, ListDatabasesResponse};
use crate::CollectionClient;
use crate::CollectionClientRequired;
use crate::{QueryCrossPartitionOption, QueryCrossPartitionSupport};
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct ListDocumentsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    if_match_condition: Option<IfMatchCondition<'a>>,
    query_cross_partition: bool,
}

impl<'a, CUB> ListDocumentsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> ListDocumentsBuilder<'a, CUB> {
        ListDocumentsBuilder {
            collection_client,
            if_match_condition: None,
            query_cross_partition: false,
        }
    }
}

impl<'a, CUB> CollectionClientRequired<'a, CUB> for ListDocumentsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

impl<'a, CUB> IfMatchConditionOption<'a> for ListDocumentsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, CUB> QueryCrossPartitionOption for ListDocumentsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn query_cross_partition(&self) -> bool {
        self.query_cross_partition
    }
}

impl<'a, CUB> IfMatchConditionSupport<'a> for ListDocumentsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, CUB>;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: Some(if_match_condition),
            query_cross_partition: self.query_cross_partition,
        }
    }
}

impl<'a, CUB> QueryCrossPartitionSupport for ListDocumentsBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListDocumentsBuilder<'a, CUB>;

    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O {
        ListDocumentsBuilder {
            collection_client: self.collection_client,
            if_match_condition: self.if_match_condition,
            query_cross_partition,
        }
    }
}

// methods callable regardless
impl<'a, CUB> ListDocumentsBuilder<'a, CUB> where CUB: CosmosUriBuilder {}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListDocumentsBuilder<'a, CUB> where CUB: CosmosUriBuilder {}
