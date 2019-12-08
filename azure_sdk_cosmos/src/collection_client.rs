use crate::client2::CosmosUriBuilder;
use crate::database_client::DatabaseClient;
use crate::requests::ListDocumentsBuilder;
use crate::{CollectionTrait, DatabaseTrait};

#[derive(Debug, Clone)]
pub struct CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) client: &'a DatabaseClient<'a, CUB>,
    collection: &'a str,
}

impl<'a, CUB> CollectionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(client: &'a DatabaseClient<'a, CUB>, collection: &'a str) -> Self {
        CollectionClient { client, collection }
    }
}

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
