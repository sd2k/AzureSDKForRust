#![recursion_limit = "128"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_sdk_core;

mod authorization_token;
mod client;
pub mod collection;
mod create_collection_builder;
pub mod database;
pub mod document;
pub mod offer;
mod partition_key;
pub mod prelude;
pub mod query;
pub mod request_response;
mod requests;

pub use self::authorization_token::*;
pub use self::client::*;
pub use self::offer::Offer;
pub use self::partition_key::*;
pub use self::requests::*;

use azure_sdk_core::enumerations;
use azure_sdk_core::errors::TraversingError;
use azure_sdk_core::parsing::FromStringOptional;
use azure_sdk_core::No;
use hyper_rustls::HttpsConnector;
use std::fmt;
use std::str::FromStr;

create_enum!(
    ConsistencyLevel,
    (Strong, "Strong"),
    (Bounded, "Bounded"),
    (Session, "Session"),
    (Eventual, "Eventual")
);

pub trait ClientRequired<'a> {
    fn client(&self) -> &'a hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;
}

pub trait DatabaseRequired<'a> {
    fn database(&self) -> &'a str;
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

pub trait Cosmos {
    fn get_document<'a>(&'a self) -> requests::GetDocumentBuilder<'a, No, No>;
}
