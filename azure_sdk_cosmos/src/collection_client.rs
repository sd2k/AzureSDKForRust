use crate::client2::{Client2, CosmosUriBuilder};
use crate::database_client::DatabaseClient;
use crate::requests;
use crate::{CollectionTrait, DatabaseTrait};
use azure_sdk_core::No;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
    collection: &'a str,
}

impl<'a, CUB> CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(database_client: &'a DatabaseClient<'a, CUB>, collection: &'a str) -> Self {
        CollectionClient {
            database_client,
            collection,
        }
    }

    pub(crate) fn main_client(&self) -> &Client2<CUB> {
        self.database_client.main_client()
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> CollectionTrait<'a, CUB> for CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database(&self) -> &'a str {
        self.database_client.database()
    }

    fn collection(&self) -> &'a str {
        self.collection
    }

    fn list(&self) -> requests::ListDocumentsBuilder<'_, '_, CUB> {
        requests::ListDocumentsBuilder::new(self)
    }

    fn get(&self) -> requests::GetDocumentBuilder<'_, '_, CUB, No> {
        requests::GetDocumentBuilder::new(self)
    }

    fn create<T>(&self) -> requests::CreateDocumentBuilder<'_, '_, T, CUB, No>
    where
        T: Serialize,
    {
        requests::CreateDocumentBuilder::new(self)
    }
}
