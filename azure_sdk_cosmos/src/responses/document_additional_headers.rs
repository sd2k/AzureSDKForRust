use crate::{
    number_of_read_regions_from_headers, request_charge_from_headers,
    request_item_count_from_headers,
};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{session_token_from_headers, SessionToken};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct DocumentAdditionalHeaders {
    pub charge: f64,
    pub session_token: String,
    pub number_of_read_regions: u32,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for DocumentAdditionalHeaders {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        DocumentAdditionalHeaders::try_from(value.0)
    }
}

impl std::convert::TryFrom<&HeaderMap> for DocumentAdditionalHeaders {
    type Error = AzureError;
    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:?}", headers);
        let dah = DocumentAdditionalHeaders {
            charge: request_charge_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
        };

        debug!("dah == {:?}", dah);
        Ok(dah)
    }
}
