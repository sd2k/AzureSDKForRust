use crate::client2::{Client2, CosmosUriBuilder, ResourceType};
use crate::database::Database;
use crate::request_response::{Document, ListCollectionsResponse, ListDatabasesResponse};
use crate::{
    Client2Required, CollectionRequired, CollectionSupport, DatabaseRequired, DatabaseSupport,
    DocumentIDRequired, DocumentIDSupport,
};
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    client: &'a Client2<CUB>,
}

impl<'a, CUB> ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(client: &'a Client2<CUB>) -> ListDatabasesBuilder<'a, CUB> {
        ListDatabasesBuilder { client }
    }
}

impl<'a, CUB> Client2Required<'a, CUB> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn client(&self) -> &'a Client2<CUB> {
        self.client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn finalize(&self) -> Result<ListDatabasesResponse, AzureError> {
        trace!("list_databases called");

        let request = self
            .client
            .prepare_request("dbs", hyper::Method::GET, ResourceType::Databases)
            .body(hyper::Body::empty())?;

        let future_response = self.client.hyper_client().request(request);
        let body = check_status_extract_body(future_response, StatusCode::OK).await?;
        let res = serde_json::from_str::<ListDatabasesResponse>(&body)?;
        Ok(res)
    }
}
