use crate::client2::{Client2, CosmosUriBuilder, ResourceType};
use crate::request_response::ListDatabasesResponse;
use crate::Client2Required;
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use hyper::StatusCode;

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
    pub async fn finalize(self) -> Result<ListDatabasesResponse, AzureError> {
        trace!("ListDatabasesBuilder::finalize called");

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
