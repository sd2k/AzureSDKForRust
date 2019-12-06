use crate::{
    Client, ClientRequired, CollectionRequired, CollectionSupport, CosmosUriBuilder,
    DatabaseRequired, DatabaseSupport, DocumentIDRequired, DocumentIDSupport,
};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet>
where
    DatabaseSet: ToAssign,
    CollectionSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    client: &'a Client<CUB>,
    p_database: PhantomData<DatabaseSet>,
    p_collection: PhantomData<CollectionSet>,
    p_document_id: PhantomData<DocumentIDSet>,
    database: Option<&'a str>,
    collection: Option<&'a str>,
    document_id: Option<&'a str>,
    if_match_condition: Option<IfMatchCondition<'a>>,
}

impl<'a, CUB> GetDocumentBuilder<'a, CUB, No, No, No>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(client: &'a Client<CUB>) -> GetDocumentBuilder<'a, CUB, No, No, No> {
        GetDocumentBuilder {
            client,
            p_database: PhantomData {},
            database: None,
            p_collection: PhantomData {},
            collection: None,
            p_document_id: PhantomData {},
            document_id: None,
            if_match_condition: None,
        }
    }
}

impl<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet> ClientRequired<'a, CUB>
    for GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet>
where
    DatabaseSet: ToAssign,
    CollectionSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn client(&self) -> &'a Client<CUB> {
        self.client
    }
}

impl<'a, CUB, CollectionSet, DocumentIDSet> DatabaseRequired<'a>
    for GetDocumentBuilder<'a, CUB, Yes, CollectionSet, DocumentIDSet>
where
    CollectionSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn database(&self) -> &'a str {
        self.database.unwrap()
    }
}

impl<'a, CUB, DatabaseSet, DocumentIDSet> CollectionRequired<'a>
    for GetDocumentBuilder<'a, CUB, DatabaseSet, Yes, DocumentIDSet>
where
    DatabaseSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn collection(&self) -> &'a str {
        self.collection.unwrap()
    }
}

impl<'a, CUB, DatabaseSet, CollectionSet> DocumentIDRequired<'a>
    for GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, Yes>
where
    DatabaseSet: ToAssign,
    CollectionSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn document_id(&self) -> &'a str {
        self.document_id.unwrap()
    }
}

impl<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet> IfMatchConditionOption<'a>
    for GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet>
where
    DatabaseSet: ToAssign,
    CollectionSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a, CUB, CollectionSet, DocumentIDSet> DatabaseSupport<'a>
    for GetDocumentBuilder<'a, CUB, No, CollectionSet, DocumentIDSet>
where
    CollectionSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, CUB, Yes, CollectionSet, DocumentIDSet>;

    fn with_database(self, database: &'a str) -> Self::O {
        GetDocumentBuilder {
            client: self.client,
            p_database: PhantomData {},
            p_collection: PhantomData {},
            p_document_id: PhantomData {},
            database: Some(database),
            collection: self.collection,
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
        }
    }
}

impl<'a, CUB, DatabaseSet, DocumentIDSet> CollectionSupport<'a>
    for GetDocumentBuilder<'a, CUB, DatabaseSet, No, DocumentIDSet>
where
    DatabaseSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, CUB, DatabaseSet, Yes, DocumentIDSet>;

    fn with_collection(self, collection: &'a str) -> Self::O {
        GetDocumentBuilder {
            client: self.client,
            p_database: PhantomData {},
            p_collection: PhantomData {},
            p_document_id: PhantomData {},
            database: self.database,
            collection: Some(collection),
            document_id: self.document_id,
            if_match_condition: self.if_match_condition,
        }
    }
}

impl<'a, CUB, DatabaseSet, CollectionSet> DocumentIDSupport<'a>
    for GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, No>
where
    DatabaseSet: ToAssign,
    CollectionSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, Yes>;

    fn with_document_id(self, document_id: &'a str) -> Self::O {
        GetDocumentBuilder {
            client: self.client,
            p_database: PhantomData {},
            p_collection: PhantomData {},
            p_document_id: PhantomData {},
            database: self.database,
            collection: self.collection,
            document_id: Some(document_id),
            if_match_condition: self.if_match_condition,
        }
    }
}

impl<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet> IfMatchConditionSupport<'a>
    for GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet>
where
    DatabaseSet: ToAssign,
    CollectionSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet>;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        GetDocumentBuilder {
            client: self.client,
            p_database: PhantomData {},
            p_collection: PhantomData {},
            p_document_id: PhantomData {},
            database: self.database,
            collection: self.collection,
            document_id: self.document_id,
            if_match_condition: Some(if_match_condition),
        }
    }
}

// methods callable regardless
impl<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet>
    GetDocumentBuilder<'a, CUB, DatabaseSet, CollectionSet, DocumentIDSet>
where
    DatabaseSet: ToAssign,
    CollectionSet: ToAssign,
    DocumentIDSet: ToAssign,
    CUB: CosmosUriBuilder,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> GetDocumentBuilder<'a, CUB, Yes, Yes, Yes> where CUB: CosmosUriBuilder {}
