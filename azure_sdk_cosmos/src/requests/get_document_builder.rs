
use crate::{ClientRequired, DatabaseRequired, DatabaseSupport, CollectionRequired, CollectionSupport};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper_rustls::HttpsConnector;
use std::marker::PhantomData;

pub struct GetDocumentBuilder<'a, DatabaseSet, CollectionSet>
where
	DatabaseSet : ToAssign,
	CollectionSet : ToAssign,
 {
	client: &'a hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
	p_database: PhantomData<DatabaseSet>,
	p_collection: PhantomData<CollectionSet>,
	database: Option<&'a str>,
	collection: Option<&'a str>,
	if_match_condition: Option<IfMatchCondition<'a>>,
}

impl<'a> GetDocumentBuilder<'a, No, No>  {
	 pub(crate) fn new(client: &'a hyper::Client<HttpsConnector<hyper::client::HttpConnector>>) -> GetDocumentBuilder<'a, No, No> {
		GetDocumentBuilder {
			client,
			p_database: PhantomData {},
			database: None,
			p_collection: PhantomData {},
			collection: None,
			if_match_condition: None,
		}
	}
}

impl<'a, DatabaseSet, CollectionSet> ClientRequired<'a> for GetDocumentBuilder<'a, DatabaseSet, CollectionSet>
where
	DatabaseSet : ToAssign,
	CollectionSet : ToAssign,
{
	fn client(&self) -> &'a hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
		self.client
	}

}

impl<'a, CollectionSet> DatabaseRequired<'a> for GetDocumentBuilder<'a, Yes, CollectionSet>
where
	CollectionSet : ToAssign,

{
	fn database(&self) -> &'a str {
		self.database.unwrap()
	}
}

impl<'a, DatabaseSet> CollectionRequired<'a> for GetDocumentBuilder<'a, DatabaseSet, Yes>
where
	DatabaseSet : ToAssign,

{
	fn collection(&self) -> &'a str {
		self.collection.unwrap()
	}
}

impl<'a, DatabaseSet, CollectionSet> IfMatchConditionOption<'a> for GetDocumentBuilder<'a, DatabaseSet, CollectionSet>
where
	DatabaseSet : ToAssign,
	CollectionSet : ToAssign,

{
	fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
		self.if_match_condition
	}
}

impl<'a, CollectionSet> DatabaseSupport<'a> for GetDocumentBuilder<'a, No, CollectionSet>
where
	CollectionSet : ToAssign,

{
	type O = GetDocumentBuilder<'a, Yes, CollectionSet>;

	fn with_database(self, database: &'a str) -> Self::O {
		GetDocumentBuilder {
				client: self.client,
				p_database: PhantomData{},
				p_collection: PhantomData{},
				database: Some(database),
				collection: self.collection,
				if_match_condition: self.if_match_condition,
		}
	}
}

impl<'a, DatabaseSet> CollectionSupport<'a> for GetDocumentBuilder<'a, DatabaseSet, No>
where
	DatabaseSet : ToAssign,

{
	type O = GetDocumentBuilder<'a, DatabaseSet, Yes>;

	fn with_collection(self, collection: &'a str) -> Self::O {
		GetDocumentBuilder {
				client: self.client,
				p_database: PhantomData{},
				p_collection: PhantomData{},
				database: self.database,
				collection: Some(collection),
				if_match_condition: self.if_match_condition,
		}
	}
}

impl<'a, DatabaseSet, CollectionSet> IfMatchConditionSupport<'a> for GetDocumentBuilder<'a, DatabaseSet, CollectionSet>
where
	DatabaseSet : ToAssign,
	CollectionSet : ToAssign,

{
	type O = GetDocumentBuilder<'a, DatabaseSet, CollectionSet>;

	fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
		GetDocumentBuilder {
				client: self.client,
				p_database: PhantomData{},
				p_collection: PhantomData{},
				database: self.database,
				collection: self.collection,
				if_match_condition: Some(if_match_condition),
		}
	}
}

// methods callable regardless
impl<'a, DatabaseSet, CollectionSet> GetDocumentBuilder<'a, DatabaseSet, CollectionSet>
where
	DatabaseSet : ToAssign,
	CollectionSet : ToAssign,

{

}

// methods callable only when every mandatory field has been filled
impl<'a> GetDocumentBuilder<'a, Yes, Yes>

{

}

