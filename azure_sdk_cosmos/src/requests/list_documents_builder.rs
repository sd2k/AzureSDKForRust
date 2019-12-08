
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use crate::client2::{CosmosUriBuilder, ResourceType};
use chrono::{DateTime, Utc};
use crate::CollectionClient;
use crate::{QueryCrossPartitionSupport, QueryCrossPartitionOption, ContinuationOption, ContinuationSupport};
use crate::request_response::{Document, ListCollectionsResponse, ListDatabasesResponse};
use crate::{CollectionClientRequired};
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,
 {
	collection_client: &'a CollectionClient<'a, CUB>,
	if_match_condition: Option<IfMatchCondition<'b>>,
	query_cross_partition: bool,
	if_modified_since: Option<&'b DateTime<Utc>>,
	user_agent: Option<&'b str>,
	activity_id: Option<&'b str>,
	continuation: Option<&'b str>,
}

impl<'a, 'b, CUB> ListDocumentsBuilder<'a, 'b, CUB> where
	CUB: CosmosUriBuilder,
 {
	 pub(crate) fn new(collection_client: &'a CollectionClient<'a, CUB>) -> ListDocumentsBuilder<'a, 'b, CUB> {
		ListDocumentsBuilder {
			collection_client,
			if_match_condition: None,
			query_cross_partition: false,
			if_modified_since: None,
			user_agent: None,
			activity_id: None,
			continuation: None,
		}
	}
}

impl<'a, 'b, CUB> CollectionClientRequired<'a, CUB> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,
{
	fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
		self.collection_client
	}

}

impl<'a, 'b, CUB> IfMatchConditionOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
		self.if_match_condition
	}
}

impl<'a, 'b, CUB> QueryCrossPartitionOption for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	fn query_cross_partition(&self) -> bool {
		self.query_cross_partition
	}
}

impl<'a, 'b, CUB> IfModifiedSinceOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
		self.if_modified_since
	}
}

impl<'a, 'b, CUB> UserAgentOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	fn user_agent(&self) -> Option<&'b str> {
		self.user_agent
	}
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	fn activity_id(&self) -> Option<&'b str> {
		self.activity_id
	}
}

impl<'a, 'b, CUB> ContinuationOption<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	fn continuation(&self) -> Option<&'b str> {
		self.continuation
	}
}

impl<'a, 'b, CUB> IfMatchConditionSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	type O = ListDocumentsBuilder<'a, 'b, CUB>;

	fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
		ListDocumentsBuilder {
				collection_client: self.collection_client,
				if_match_condition: Some(if_match_condition),
				query_cross_partition: self.query_cross_partition,
				if_modified_since: self.if_modified_since,
				user_agent: self.user_agent,
				activity_id: self.activity_id,
				continuation: self.continuation,
		}
	}
}

impl<'a, 'b, CUB> QueryCrossPartitionSupport for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	type O = ListDocumentsBuilder<'a, 'b, CUB>;

	fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O {
		ListDocumentsBuilder {
				collection_client: self.collection_client,
				if_match_condition: self.if_match_condition,
				query_cross_partition,
				if_modified_since: self.if_modified_since,
				user_agent: self.user_agent,
				activity_id: self.activity_id,
				continuation: self.continuation,
		}
	}
}

impl<'a, 'b, CUB> IfModifiedSinceSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	type O = ListDocumentsBuilder<'a, 'b, CUB>;

	fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
		ListDocumentsBuilder {
				collection_client: self.collection_client,
				if_match_condition: self.if_match_condition,
				query_cross_partition: self.query_cross_partition,
				if_modified_since: Some(if_modified_since),
				user_agent: self.user_agent,
				activity_id: self.activity_id,
				continuation: self.continuation,
		}
	}
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	type O = ListDocumentsBuilder<'a, 'b, CUB>;

	fn with_user_agent(self, user_agent: &'b str) -> Self::O {
		ListDocumentsBuilder {
				collection_client: self.collection_client,
				if_match_condition: self.if_match_condition,
				query_cross_partition: self.query_cross_partition,
				if_modified_since: self.if_modified_since,
				user_agent: Some(user_agent),
				activity_id: self.activity_id,
				continuation: self.continuation,
		}
	}
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	type O = ListDocumentsBuilder<'a, 'b, CUB>;

	fn with_activity_id(self, activity_id: &'b str) -> Self::O {
		ListDocumentsBuilder {
				collection_client: self.collection_client,
				if_match_condition: self.if_match_condition,
				query_cross_partition: self.query_cross_partition,
				if_modified_since: self.if_modified_since,
				user_agent: self.user_agent,
				activity_id: Some(activity_id),
				continuation: self.continuation,
		}
	}
}

impl<'a, 'b, CUB> ContinuationSupport<'b> for ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{
	type O = ListDocumentsBuilder<'a, 'b, CUB>;

	fn with_continuation(self, continuation: &'b str) -> Self::O {
		ListDocumentsBuilder {
				collection_client: self.collection_client,
				if_match_condition: self.if_match_condition,
				query_cross_partition: self.query_cross_partition,
				if_modified_since: self.if_modified_since,
				user_agent: self.user_agent,
				activity_id: self.activity_id,
				continuation: Some(continuation),
		}
	}
}

// methods callable regardless
impl<'a, 'b, CUB> ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{

}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> ListDocumentsBuilder<'a, 'b, CUB>
where
	CUB: CosmosUriBuilder,

{

}

