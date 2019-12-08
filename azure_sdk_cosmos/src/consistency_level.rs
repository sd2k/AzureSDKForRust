use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConsistencyLevel<'a> {
    Strong,
    Bounded,
    Session(&'a str),
    ConsistentPrefix,
    Eventual,
}

impl<'a> fmt::Display for ConsistencyLevel<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Strong => write!(f, "Strong"),
            Self::Bounded => write!(f, "Bounded"),
            Self::Session(_) => write!(f, "Session"),
            Self::ConsistentPrefix => write!(f, "Prefix"), //this is guessed since it's missing here: https://docs.microsoft.com/en-us/rest/api/cosmos-db/common-cosmosdb-rest-request-headers
            Self::Eventual => write!(f, "Eventual"),
        }
    }
}
