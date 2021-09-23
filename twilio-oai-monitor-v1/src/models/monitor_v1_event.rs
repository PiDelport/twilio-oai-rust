/*
 * Twilio - Monitor
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MonitorV1Event {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The SID of the actor that caused the event, if available
    #[serde(rename = "actor_sid", skip_serializing_if = "Option::is_none")]
    pub actor_sid: Option<String>,
    /// The type of actor that caused the event
    #[serde(rename = "actor_type", skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<String>,
    /// A description of the event
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A JSON string that represents an object with additional data about the event
    #[serde(rename = "event_data", skip_serializing_if = "Option::is_none")]
    pub event_data: Option<serde_json::Value>,
    /// The ISO 8601 date and time in GMT when the event was recorded
    #[serde(rename = "event_date", skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    /// The event's type
    #[serde(rename = "event_type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// The absolute URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The SID of the resource that was affected
    #[serde(rename = "resource_sid", skip_serializing_if = "Option::is_none")]
    pub resource_sid: Option<String>,
    /// The type of resource that was affected
    #[serde(rename = "resource_type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The originating system or interface that caused the event
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The IP address of the source
    #[serde(rename = "source_ip_address", skip_serializing_if = "Option::is_none")]
    pub source_ip_address: Option<String>,
    /// The absolute URL of the resource that was affected
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl MonitorV1Event {
    pub fn new() -> MonitorV1Event {
        MonitorV1Event {
            account_sid: None,
            actor_sid: None,
            actor_type: None,
            description: None,
            event_data: None,
            event_date: None,
            event_type: None,
            links: None,
            resource_sid: None,
            resource_type: None,
            sid: None,
            source: None,
            source_ip_address: None,
            url: None,
        }
    }
}


