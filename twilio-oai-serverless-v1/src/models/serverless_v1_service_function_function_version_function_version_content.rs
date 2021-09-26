/*
 * Twilio - Serverless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerlessV1ServiceFunctionFunctionVersionFunctionVersionContent {
    /// The SID of the Account that created the Function Version resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The content of the Function Version resource
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The SID of the Function that is the parent of the Function Version
    #[serde(rename = "function_sid", skip_serializing_if = "Option::is_none")]
    pub function_sid: Option<String>,
    /// The SID of the Service that the Function Version resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the Function Version resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ServerlessV1ServiceFunctionFunctionVersionFunctionVersionContent {
    pub fn new() -> ServerlessV1ServiceFunctionFunctionVersionFunctionVersionContent {
        ServerlessV1ServiceFunctionFunctionVersionFunctionVersionContent {
            account_sid: None,
            content: None,
            function_sid: None,
            service_sid: None,
            sid: None,
            url: None,
        }
    }
}
