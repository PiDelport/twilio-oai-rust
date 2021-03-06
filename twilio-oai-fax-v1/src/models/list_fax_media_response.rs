/*
 * Twilio - Fax
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListFaxMediaResponse {
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<crate::models::FaxV1FaxFaxMedia>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListFaxResponseMeta>>,
}

impl ListFaxMediaResponse {
    pub fn new() -> ListFaxMediaResponse {
        ListFaxMediaResponse {
            media: None,
            meta: None,
        }
    }
}
