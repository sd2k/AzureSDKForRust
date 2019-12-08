
use crate::client2::{Client2, CosmosUriBuilder, ResourceType};
use crate::database::Database;
use crate::DatabaseClient;
use crate::request_response::{Document, ListCollectionsResponse, ListDatabasesResponse};
use crate::{DatabaseClientRequired};
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct ListCollectionsBuilder<'a, CUB>
where
	CUB: CosmosUriBuilder,
 {
	database_client: &'a DatabaseClient<'a, CUB>,
}

impl<'a, CUB> ListCollectionsBuilder<'a, CUB> where
	CUB: CosmosUriBuilder,
 {
	 pub(crate) fn new(database_client: &'a DatabaseClient<'a, CUB>) -> ListCollectionsBuilder<'a, CUB> {
		ListCollectionsBuilder {
			database_client,
		}
	}
}

impl<'a, CUB> DatabaseClientRequired<'a, CUB> for ListCollectionsBuilder<'a, CUB>
where
	CUB: CosmosUriBuilder,
{
	fn database_client(&self) -> &'a DatabaseClient<'a, CUB> {
		self.database_client
	}

}

// methods callable regardless
impl<'a, CUB> ListCollectionsBuilder<'a, CUB>
where
	CUB: CosmosUriBuilder,

{

}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListCollectionsBuilder<'a, CUB>
where
	CUB: CosmosUriBuilder,

{

}

