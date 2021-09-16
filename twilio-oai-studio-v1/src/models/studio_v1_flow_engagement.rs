/*
 * Twilio - Studio
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StudioV1FlowEngagement {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The phone number, SIP address or Client identifier that triggered this Engagement
    #[serde(rename = "contact_channel_address", skip_serializing_if = "Option::is_none")]
    pub contact_channel_address: Option<String>,
    /// The SID of the Contact
    #[serde(rename = "contact_sid", skip_serializing_if = "Option::is_none")]
    pub contact_sid: Option<String>,
    /// The current state of the execution flow
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    /// The ISO 8601 date and time in GMT when the Engagement was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the Engagement was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The SID of the Flow
    #[serde(rename = "flow_sid", skip_serializing_if = "Option::is_none")]
    pub flow_sid: Option<String>,
    /// The URLs of the Engagement's nested resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of the Engagement
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl StudioV1FlowEngagement {
    pub fn new() -> StudioV1FlowEngagement {
        StudioV1FlowEngagement {
            account_sid: None,
            contact_channel_address: None,
            contact_sid: None,
            context: None,
            date_created: None,
            date_updated: None,
            flow_sid: None,
            links: None,
            sid: None,
            status: None,
            url: None,
        }
    }
}

/// The status of the Engagement
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "ended")]
    Ended,
}
