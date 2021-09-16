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
pub struct ListVariableResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<crate::models::ServerlessV1ServiceEnvironmentVariable>>,
}

impl ListVariableResponse {
    pub fn new() -> ListVariableResponse {
        ListVariableResponse {
            meta: None,
            variables: None,
        }
    }
}

