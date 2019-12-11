#![recursion_limit = "128"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_sdk_core;

mod authorization_token;
mod client;
mod client2;
pub mod collection;
mod collection_client;
mod consistency_level;
mod create_collection_builder;
pub mod database;
mod database_client;
pub mod document;
pub mod offer;
mod partition_key;
pub mod prelude;
pub mod query;
pub mod request_response;
mod requests;

pub use self::authorization_token::*;
pub use self::client::*;
pub use self::client2::{Client2, Client2Builder};
pub use self::consistency_level::ConsistencyLevel;
pub use self::offer::Offer;
pub use self::partition_key::*;
pub use self::requests::*;

use self::collection_client::CollectionClient;
use self::database_client::DatabaseClient;
use crate::client2::headers::*;
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use http::request::Builder;

pub trait ClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn client(&self) -> &'a Client<CUB>;
}

pub trait Client2Required<'a, CUB>
where
    CUB: crate::client2::CosmosUriBuilder,
{
    fn client(&self) -> &'a Client2<CUB>;
}

pub trait DatabaseRequired<'a> {
    fn database(&self) -> &'a str;
}

pub trait QueryCrossPartitionSupport {
    type O;
    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O;
}

pub trait QueryCrossPartitionOption {
    fn query_cross_partition(&self) -> bool;

    fn add_header(&self, builder: &mut Builder) {
        builder.header(
            HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION,
            self.query_cross_partition().to_string(),
        );
    }
}

pub trait AIMSupport {
    type O;
    fn with_a_im(self, a_im: bool) -> Self::O;
}

pub trait AIMOption {
    fn a_im(&self) -> bool;

    fn add_header(&self, builder: &mut Builder) {
        if self.a_im() == true {
            builder.header(HEADER_A_IM, "Incremental feed");
        }
    }
}

pub trait AllowTentativeWritesSupport {
    type O;
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O;
}

pub trait AllowTentativeWritesOption {
    fn allow_tentative_writes(&self) -> bool;

    fn add_header(&self, builder: &mut Builder) {
        builder.header(
            HEADER_ALLOW_MULTIPLE_WRITES,
            self.allow_tentative_writes().to_string(),
        );
    }
}

pub trait ConsistencyLevelSupport<'a> {
    type O;
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O;
}

pub trait ConsistencyLevelOption<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(consistency_level) = self.consistency_level() {
            builder.header(
                HEADER_CONSISTENCY_LEVEL,
                consistency_level.to_consistency_level_header(),
            );

            // if we have a Session consistency level we make sure to pass
            // the x-ms-session-token header too.
            if let ConsistencyLevel::Session(session_token) = consistency_level {
                builder.header(HEADER_SESSION_TOKEN, session_token);
            }
        }
    }
}

pub trait PartitionRangeIdSupport<'a> {
    type O;
    fn with_partition_range_id(self, partition_range_id: &'a str) -> Self::O;
}

pub trait PartitionRangeIdOption<'a> {
    fn partition_range_id(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(partition_range_id) = self.partition_range_id() {
            builder.header(HEADER_DOCUMENTDB_PARTITIONRANGEID, partition_range_id);
        }
    }
}

pub trait ContinuationSupport<'a> {
    type O;
    fn with_continuation(self, continuation: &'a str) -> Self::O;
}

pub trait ContinuationOption<'a> {
    fn continuation(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(continuation) = self.continuation() {
            builder.header(HEADER_CONTINUATION, continuation);
        }
    }
}

pub trait MaxItemCountSupport {
    type O;
    fn with_max_item_count(self, max_item_count: i32) -> Self::O;
}

pub trait MaxItemCountOption {
    fn max_item_count(&self) -> i32;

    fn add_header(&self, builder: &mut Builder) {
        if self.max_item_count() <= 0 {
            builder.header(HEADER_MAX_ITEM_COUNT, -1);
        } else {
            builder.header(HEADER_MAX_ITEM_COUNT, self.max_item_count());
        }
    }
}

pub trait PartitionKeySupport<'a> {
    type O;
    fn with_partition_key(self, partition_key: &'a [&'a str]) -> Self::O;
}

pub trait PartitionKeyOption<'a> {
    fn partition_key(&self) -> Option<&'a [&'a str]>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(partition_key) = self.partition_key() {
            let serialized = serde_json::to_string(partition_key).unwrap();
            builder.header(HEADER_DOCUMENTDB_PARTITIONKEY, serialized);
        }
    }
}

pub trait DatabaseClientRequired<'a, CUB>
where
    CUB: crate::client2::CosmosUriBuilder,
{
    fn database_client(&self) -> &'a DatabaseClient<'a, CUB>;
}

pub trait CollectionClientRequired<'a, CUB>
where
    CUB: crate::client2::CosmosUriBuilder,
{
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB>;
}

pub trait DatabaseSupport<'a> {
    type O;
    fn with_database(self, database: &'a str) -> Self::O;
}

pub trait CollectionRequired<'a> {
    fn collection(&self) -> &'a str;
}

pub trait CollectionSupport<'a> {
    type O;
    fn with_collection(self, collection: &'a str) -> Self::O;
}

pub trait DocumentIDRequired<'a> {
    fn document_id(&self) -> &'a str;
}

pub trait DocumentIDSupport<'a> {
    type O;
    fn with_document_id(self, document_id: &'a str) -> Self::O;
}

//// New implementation
pub trait CosmosTrait<CUB>
where
    CUB: crate::client2::CosmosUriBuilder,
{
    fn list(&self) -> requests::ListDatabasesBuilder<'_, CUB>;
    fn with_database<'d>(&'d self, database_name: &'d dyn DatabaseName) -> DatabaseClient<'d, CUB>;
}

pub trait DatabaseTrait<'a, CUB>
where
    CUB: crate::client2::CosmosUriBuilder,
{
    fn database(&self) -> &'a str;
    fn list(&self) -> requests::ListCollectionsBuilder<'_, CUB>;
    fn with_collection<'c>(
        &'c self,
        collection_name: &'c dyn CollectionName,
    ) -> CollectionClient<'c, CUB>;
}

pub trait CollectionTrait<'a, CUB>
where
    CUB: crate::client2::CosmosUriBuilder,
{
    fn database(&self) -> &'a str;
    fn collection(&self) -> &'a str;
    fn list(&self) -> requests::ListDocumentsBuilder<'_, '_, CUB>;
}
