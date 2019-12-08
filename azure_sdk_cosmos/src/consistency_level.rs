use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConsistencyLevel<'a> {
    Strong,
    Bounded,
    Session(&'a str),
    ConsistentPrefix,
    Eventual,
}

impl<'a> ConsistencyLevel<'a> {
    pub fn to_consistency_level_header(&self) -> &'static str {
        match self {
            Self::Strong => "Strong",
            Self::Bounded => "Bounded",
            Self::Session(_) => "Session",
            Self::ConsistentPrefix => "Prefix", //this is guessed since it's missing here: https://docs.microsoft.com/en-us/rest/api/cosmos-db/common-cosmosdb-rest-request-headers
            Self::Eventual => "Eventual",
        }
    }
}
