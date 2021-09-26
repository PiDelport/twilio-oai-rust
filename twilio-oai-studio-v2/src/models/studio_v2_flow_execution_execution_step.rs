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
pub struct StudioV2FlowExecutionExecutionStep {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The current state of the flow
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The SID of the Execution
    #[serde(rename = "execution_sid", skip_serializing_if = "Option::is_none")]
    pub execution_sid: Option<String>,
    /// The SID of the Flow
    #[serde(rename = "flow_sid", skip_serializing_if = "Option::is_none")]
    pub flow_sid: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The event that caused the Flow to transition to the Step
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The Widget that preceded the Widget for the Step
    #[serde(rename = "transitioned_from", skip_serializing_if = "Option::is_none")]
    pub transitioned_from: Option<String>,
    /// The Widget that will follow the Widget for the Step
    #[serde(rename = "transitioned_to", skip_serializing_if = "Option::is_none")]
    pub transitioned_to: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl StudioV2FlowExecutionExecutionStep {
    pub fn new() -> StudioV2FlowExecutionExecutionStep {
        StudioV2FlowExecutionExecutionStep {
            account_sid: None,
            context: None,
            date_created: None,
            date_updated: None,
            execution_sid: None,
            flow_sid: None,
            links: None,
            name: None,
            sid: None,
            transitioned_from: None,
            transitioned_to: None,
            url: None,
        }
    }
}
