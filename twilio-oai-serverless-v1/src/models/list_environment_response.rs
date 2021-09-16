/*
 * Twilio - Serverless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListEnvironmentResponse {
    #[serde(rename = "environments", skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<crate::models::ServerlessV1ServiceEnvironment>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
}

impl ListEnvironmentResponse {
    pub fn new() -> ListEnvironmentResponse {
        ListEnvironmentResponse {
            environments: None,
            meta: None,
        }
    }
}

