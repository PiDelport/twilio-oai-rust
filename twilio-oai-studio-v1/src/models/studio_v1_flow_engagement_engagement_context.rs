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
pub struct StudioV1FlowEngagementEngagementContext {
    /// Account SID
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Flow state
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    /// Engagement SID
    #[serde(rename = "engagement_sid", skip_serializing_if = "Option::is_none")]
    pub engagement_sid: Option<String>,
    /// Flow SID
    #[serde(rename = "flow_sid", skip_serializing_if = "Option::is_none")]
    pub flow_sid: Option<String>,
    /// The URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl StudioV1FlowEngagementEngagementContext {
    pub fn new() -> StudioV1FlowEngagementEngagementContext {
        StudioV1FlowEngagementEngagementContext {
            account_sid: None,
            context: None,
            engagement_sid: None,
            flow_sid: None,
            url: None,
        }
    }
}
