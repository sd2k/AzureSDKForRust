use crate::client2::{Client2, CosmosUriBuilder};
use crate::database_client::DatabaseClient;
use crate::requests::ListDocumentsBuilder;
use crate::{CollectionTrait, DatabaseTrait, HyperClient, MainClient};

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
}

impl<'a, CUB> MainClient<CUB> for CollectionClient<'a, CUB>
where
    CUB: crate::client2::CosmosUriBuilder,
{
    fn main_client(&self) -> &Client2<CUB> {
        self.database_client.main_client()
    }
}

impl<'a, CUB> HyperClient<CUB> for CollectionClient<'a, CUB> {}

impl<'a, CUB> CollectionTrait<'a, CUB> for CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database(&self) -> &'a str {
        self.client.database()
    }

    fn collection(&self) -> &'a str {
        self.collection
    }

    fn list(&self) -> ListDocumentsBuilder<'_, '_, CUB> {
        ListDocumentsBuilder::new(self)
    }
}
