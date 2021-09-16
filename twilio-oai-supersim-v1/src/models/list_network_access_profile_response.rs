/*
 * Twilio - Supersim
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListNetworkAccessProfileResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCommandResponseMeta>>,
    #[serde(rename = "network_access_profiles", skip_serializing_if = "Option::is_none")]
    pub network_access_profiles: Option<Vec<crate::models::SupersimV1NetworkAccessProfile>>,
}

impl ListNetworkAccessProfileResponse {
    pub fn new() -> ListNetworkAccessProfileResponse {
        ListNetworkAccessProfileResponse {
            meta: None,
            network_access_profiles: None,
        }
    }
}

