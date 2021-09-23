/*
 * Twilio - Ip_messaging
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListServiceResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCredentialResponseMeta>>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::IpMessagingV1Service>>,
}

impl ListServiceResponse {
    pub fn new() -> ListServiceResponse {
        ListServiceResponse {
            meta: None,
            services: None,
        }
    }
}


