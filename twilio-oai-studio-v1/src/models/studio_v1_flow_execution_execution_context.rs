/*
 * Twilio - Studio
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StudioV1FlowExecutionExecutionContext {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The current state of the flow
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    /// The SID of the Execution
    #[serde(rename = "execution_sid", skip_serializing_if = "Option::is_none")]
    pub execution_sid: Option<String>,
    /// The SID of the Flow
    #[serde(rename = "flow_sid", skip_serializing_if = "Option::is_none")]
    pub flow_sid: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl StudioV1FlowExecutionExecutionContext {
    pub fn new() -> StudioV1FlowExecutionExecutionContext {
        StudioV1FlowExecutionExecutionContext {
            account_sid: None,
            context: None,
            execution_sid: None,
            flow_sid: None,
            url: None,
        }
    }
}


