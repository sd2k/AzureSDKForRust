#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub id: String,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(rename = "_colls")]
    pub colls: String,
    #[serde(rename = "_users")]
    pub users: String,
}

pub trait DatabaseName {
    fn name(&self) -> &str;
}

impl DatabaseName for Database {
    fn name(&self) -> &str {
        &self.id
    }
}

impl DatabaseName for dyn AsRef<str> {
    fn name(&self) -> &str {
        self.as_ref()
    }
}

impl DatabaseName for String {
    fn name(&self) -> &str {
        self.as_ref()
    }
}
