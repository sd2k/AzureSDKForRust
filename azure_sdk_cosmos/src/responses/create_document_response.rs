use crate::document_attributes::DocumentAttributes;
use crate::responses::DocumentAdditionalHeaders;
use azure_sdk_core::errors::AzureError;
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct CreateDocumentResponse {
    pub document_attributes: DocumentAttributes, //pub additional_headers: DocumentAdditionalHeaders,
    pub additional_headers: DocumentAdditionalHeaders,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for CreateDocumentResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let _headers = value.0;
        let body = value.1;

        Ok(CreateDocumentResponse {
            document_attributes: DocumentAttributes::try_from(value)?,
            additional_headers: DocumentAdditionalHeaders::try_from(value)?,
        })
    }
}
