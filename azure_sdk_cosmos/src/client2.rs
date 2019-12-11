use super::{
    //collection::Collection,
    //database::Database,
    //query::Query,
    requests::*,
    AuthorizationToken,
    CosmosTrait,
    TokenType,
};
//use crate::create_collection_builder::CreateCollectionBuilder;
use crate::database::DatabaseName;
use crate::database_client::DatabaseClient;
use azure_sdk_core::errors::AzureError;
use base64;
use chrono;
use http::request::Builder as RequestBuilder;
use hyper::{
    self,
    header::{self, HeaderValue},
};
use hyper_rustls::HttpsConnector;
use ring::hmac;
use url::form_urlencoded;

const AZURE_VERSION: &str = "2017-02-22";
const VERSION: &str = "1.0";
const TIME_FORMAT: &str = "%a, %d %h %Y %T GMT";

pub(crate) mod headers {
    pub const HEADER_VERSION: &str = "x-ms-version"; // Cow[str]
    pub const HEADER_DATE: &str = "x-ms-date"; // [String]
                                               //pub const HEADER_OFFER_THROUGHPUT: &str = "x-ms-offer-throughput"; // [u64]
                                               //pub const HEADER_OFFER_TYPE: &str = "x-ms-offer-type"; // [&str]
                                               //pub const HEADER_DOCUMENTDB_IS_UPSERT: &str = "x-ms-documentdb-is-upsert"; // [bool]
                                               //pub const HEADER_INDEXING_DIRECTIVE: &str = "x-ms-indexing-directive"; // [IndexingDirective]
    pub const HEADER_MAX_ITEM_COUNT: &str = "x-ms-max-item-count"; // [u64]
    pub const HEADER_CONTINUATION: &str = "x-ms-continuation"; // [ContinuationToken]
    pub const HEADER_CONSISTENCY_LEVEL: &str = "x-ms-consistency-level"; // [ConsistencyLevel]
    pub const HEADER_SESSION_TOKEN: &str = "x-ms-session-token"; // [ContinuationToken]
    pub const HEADER_ALLOW_MULTIPLE_WRITES: &str = "x-ms-cosmos-allow-tentative-writes"; // [bool]
    pub const HEADER_A_IM: &str = "A-IM"; // Cow[str]
    pub const HEADER_DOCUMENTDB_PARTITIONRANGEID: &str = "x-ms-documentdb-partitionkeyrangeid"; // [String]
                                                                                                //pub const HEADER_REQUEST_CHARGE: &str = "x-ms-request-charge"; // [f64]
    pub const HEADER_DOCUMENTDB_PARTITIONKEY: &str = "x-ms-documentdb-partitionkey";
    // [String]
    //pub const HEADER_DOCUMENTDB_ISQUERY: &str = "x-ms-documentdb-isquery"; // [bool]
    pub const HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION: &str =
        "x-ms-documentdb-query-enablecrosspartition"; // [bool]
                                                      //pub const HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY: &str =
                                                      //    "x-ms-documentdb-query-parallelizecrosspartitionquery";
                                                      //// [bool]
                                                      //pub const HEADER_ITEM_COUNT: &'static str = "x-ms-item-count"; // u64
                                                      //pub const HEADER_NUMBER_OF_READ_REGIONS: &str = "x-ms-number-of-read-regions";
                                                      // u32
}
use self::headers::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum ResourceType {
    Databases,
    Collections,
    Documents,
    StoredProcedures,
}

pub trait CosmosUriBuilder {
    fn build_base_uri(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct Client2<CUB>
where
    CUB: CosmosUriBuilder,
{
    hyper_client: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
    auth_token: AuthorizationToken,
    cosmos_uri_builder: CUB,
}

#[derive(Debug, Clone)]
pub struct DefaultCosmosUri {
    uri: String,
}

impl DefaultCosmosUri {
    fn new(account: &str) -> DefaultCosmosUri {
        DefaultCosmosUri {
            uri: format!("https://{}.documents.azure.com", account),
        }
    }
}

impl CosmosUriBuilder for DefaultCosmosUri {
    fn build_base_uri(&self) -> &str {
        &self.uri
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChinaCosmosUri {
    uri: String,
}

impl ChinaCosmosUri {
    fn new(account: &str) -> ChinaCosmosUri {
        ChinaCosmosUri {
            uri: format!("https://{}.documents.azure.cn", account),
        }
    }
}

impl CosmosUriBuilder for ChinaCosmosUri {
    fn build_base_uri(&self) -> &str {
        &self.uri
    }
}

#[derive(Debug, Clone, Default)]
pub struct CustomCosmosUri {
    uri: String,
}

impl CosmosUriBuilder for CustomCosmosUri {
    fn build_base_uri(&self) -> &str {
        &self.uri
    }
}

pub struct Client2Builder {}

impl Client2Builder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(auth_token: AuthorizationToken) -> Result<Client2<DefaultCosmosUri>, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());
        let cosmos_uri_builder = DefaultCosmosUri::new(auth_token.account());

        Ok(Client2 {
            hyper_client: client,
            auth_token,
            cosmos_uri_builder,
        })
    }

    pub fn new_china(
        auth_token: AuthorizationToken,
    ) -> Result<Client2<ChinaCosmosUri>, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());
        let cosmos_uri_builder = ChinaCosmosUri::new(auth_token.account());

        Ok(Client2 {
            hyper_client: client,
            auth_token,
            cosmos_uri_builder,
        })
    }

    pub fn new_custom(
        auth_token: AuthorizationToken,
        uri: String,
    ) -> Result<Client2<CustomCosmosUri>, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        Ok(Client2 {
            hyper_client: client,
            auth_token,
            cosmos_uri_builder: CustomCosmosUri { uri },
        })
    }

    pub fn new_emulator(address: &str, port: u16) -> Result<Client2<CustomCosmosUri>, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        //Account name: localhost:<port>
        //Account key: C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==
        let auth_token = AuthorizationToken::new(
            format!("{}:{}", address, port),
            TokenType::Master,
            "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==",
        ).unwrap();
        Ok(Client2 {
            hyper_client: client,
            auth_token,
            cosmos_uri_builder: CustomCosmosUri {
                uri: format!("https://{}:{}", address, port),
            },
        })
    }
}

impl<CUB> CosmosTrait<CUB> for Client2<CUB>
where
    CUB: CosmosUriBuilder,
{
    fn list(&self) -> ListDatabasesBuilder<'_, CUB> {
        ListDatabasesBuilder::new(self)
    }

    fn with_database<'a>(&'a self, database_name: &'a dyn DatabaseName) -> DatabaseClient<'a, CUB> {
        DatabaseClient::new(self, database_name.name())
    }
}

impl<CUB> Client2<CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
        &self.hyper_client
    }

    #[inline]
    pub(crate) fn prepare_request(
        &self,
        uri_path: &str,
        http_method: hyper::Method,
        resource_type: ResourceType,
    ) -> RequestBuilder {
        let time = format!("{}", chrono::Utc::now().format(TIME_FORMAT));

        let auth = {
            let resource_link = generate_resource_link(&uri_path);
            generate_authorization(
                &self.auth_token,
                &http_method,
                resource_type,
                resource_link,
                &time,
            )
        };
        self.prepare_request_with_signature(uri_path, http_method, &time, &auth)
    }

    //#[inline]
    //fn prepare_request_with_resource_link(
    //    &self,
    //    uri_path: &str,
    //    http_method: hyper::Method,
    //    resource_type: ResourceType,
    //    resource_link: &str,
    //) -> RequestBuilder {
    //    let time = format!("{}", chrono::Utc::now().format(TIME_FORMAT));

    //    let sig = {
    //        generate_authorization(
    //            &self.auth_token,
    //            &http_method,
    //            resource_type,
    //            resource_link,
    //            &time,
    //        )
    //    };
    //    self.prepare_request_with_signature(uri_path, http_method, &time, &sig)
    //}

    #[inline]
    fn prepare_request_with_signature(
        &self,
        uri_path: &str,
        http_method: hyper::Method,
        time: &str,
        signature: &str,
    ) -> RequestBuilder {
        trace!("prepare_request::auth == {:?}", signature);
        let uri = format!("{}/{}", self.cosmos_uri_builder.build_base_uri(), uri_path);
        debug!(
            "cosmos::client::prepare_request_with_resource_signature::uri == {:?}",
            uri
        );
        let mut request = hyper::Request::builder();
        request
            .method(http_method)
            .uri(uri)
            .header(HEADER_DATE, time)
            .header(HEADER_VERSION, HeaderValue::from_static(AZURE_VERSION))
            .header(header::AUTHORIZATION, signature);
        request
    }
}

fn generate_authorization(
    auth_token: &AuthorizationToken,
    http_method: &hyper::Method,
    resource_type: ResourceType,
    resource_link: &str,
    time: &str,
) -> String {
    let string_to_sign = string_to_sign(http_method, resource_type, resource_link, time);
    trace!(
        "generate_authorization::string_to_sign == {:?}",
        string_to_sign
    );

    let str_unencoded = format!(
        "type={}&ver={}&sig={}",
        match auth_token.token_type() {
            TokenType::Master => "master",
            TokenType::Resource => "resource",
        },
        VERSION,
        encode_str_to_sign(&string_to_sign, auth_token)
    );

    trace!(
        "generate_authorization::str_unencoded == {:?}",
        str_unencoded
    );

    form_urlencoded::byte_serialize(&str_unencoded.as_bytes()).collect::<String>()
}

fn encode_str_to_sign(str_to_sign: &str, auth_token: &AuthorizationToken) -> String {
    let key = hmac::Key::new(ring::hmac::HMAC_SHA256, auth_token.key());
    let sig = hmac::sign(&key, str_to_sign.as_bytes());
    base64::encode(sig.as_ref())
}

fn string_to_sign(
    http_method: &hyper::Method,
    rt: ResourceType,
    resource_link: &str,
    time: &str,
) -> String {
    // From official docs:
    // StringToSign =
    //      Verb.toLowerCase() + "\n" +
    //      ResourceType.toLowerCase() + "\n" +
    //      ResourceLink + "\n" +
    //      Date.toLowerCase() + "\n" +
    //      "" + "\n";
    // Notice the empty string at the end so we need to add two carriage returns

    format!(
        "{}\n{}\n{}\n{}\n\n",
        match *http_method {
            hyper::Method::GET => "get",
            hyper::Method::PUT => "put",
            hyper::Method::POST => "post",
            hyper::Method::DELETE => "delete",
            hyper::Method::HEAD => "head",
            hyper::Method::TRACE => "trace",
            hyper::Method::OPTIONS => "options",
            hyper::Method::CONNECT => "connect",
            hyper::Method::PATCH => "patch",
            _ => "extension",
        },
        match rt {
            ResourceType::Databases => "dbs",
            ResourceType::Collections => "colls",
            ResourceType::Documents => "docs",
            ResourceType::StoredProcedures => "sprocs",
        },
        resource_link,
        time.to_lowercase()
    )
}

fn generate_resource_link(u: &str) -> &str {
    static ENDING_STRINGS: &[&str] = &["dbs", "colls", "docs"];

    // store the element only if it does not end with dbs, colls or docs
    let p = u;
    let len = p.len();
    for str_to_match in ENDING_STRINGS {
        let end_len = str_to_match.len();

        if end_len <= len {
            let end_offset = len - end_len;
            let sm = &p[end_offset..];
            if sm == *str_to_match {
                if len == end_len {
                    return "";
                }

                if &p[end_offset - 1..end_offset] == "/" {
                    let ret = &p[0..len - end_len - 1];
                    return ret;
                }
            }
        }
    }
    p
}
