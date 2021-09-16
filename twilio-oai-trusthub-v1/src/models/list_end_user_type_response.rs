/*
 * Twilio - Trusthub
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListEndUserTypeResponse {
    #[serde(rename = "end_user_types", skip_serializing_if = "Option::is_none")]
    pub end_user_types: Option<Vec<crate::models::TrusthubV1EndUserType>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCustomerProfileResponseMeta>>,
}

impl ListEndUserTypeResponse {
    pub fn new() -> ListEndUserTypeResponse {
        ListEndUserTypeResponse {
            end_user_types: None,
            meta: None,
        }
    }
}

