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
pub use self::offer::Offer;
pub use self::partition_key::*;
pub use self::requests::*;

use self::collection_client::CollectionClient;
use self::database_client::DatabaseClient;
use crate::client2::headers::*;
use azure_sdk_core::enumerations;
use azure_sdk_core::errors::TraversingError;
use azure_sdk_core::parsing::FromStringOptional;
use http::request::Builder;
//use azure_sdk_core::No;
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use std::fmt;
use std::str::FromStr;

create_enum!(
    ConsistencyLevel,
    (Strong, "Strong"),
    (Bounded, "Bounded"),
    (Session, "Session"),
    (Eventual, "Eventual")
);

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
}
