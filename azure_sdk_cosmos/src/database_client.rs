use crate::client2::{Client2, CosmosUriBuilder};
use crate::requests::ListCollectionsBuilder;
use crate::{CollectionClient, CollectionName, DatabaseTrait};

#[derive(Debug, Clone)]
pub struct DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    main_client: &'a Client2<CUB>,
    database: &'a str,
}

impl<'a, CUB> DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(main_client: &'a Client2<CUB>, database: &'a str) -> Self {
        DatabaseClient {
            main_client,
            database,
        }
    }

    pub(crate) fn main_client(&self) -> &Client2<CUB> {
        self.main_client
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> DatabaseTrait<'a, CUB> for DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database(&self) -> &'a str {
        self.database
    }

    fn list(&self) -> ListCollectionsBuilder<'_, CUB> {
        ListCollectionsBuilder::new(self)
    }

    fn with_collection<'c>(
        &'c self,
        collection_name: &'c dyn CollectionName,
    ) -> CollectionClient<'c, CUB> {
        CollectionClient::new(self, collection_name.name())
    }
}
