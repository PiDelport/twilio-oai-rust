/*
 * Twilio - Proxy
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProxyV1Service {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The URL we call when the interaction status changes
    #[serde(rename = "callback_url", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// The SID of the Chat Service Instance
    #[serde(rename = "chat_instance_sid", skip_serializing_if = "Option::is_none")]
    pub chat_instance_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Default TTL for a Session, in seconds
    #[serde(rename = "default_ttl", skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<i32>,
    /// Where a proxy number must be located relative to the participant identifier
    #[serde(rename = "geo_match_level", skip_serializing_if = "Option::is_none")]
    pub geo_match_level: Option<GeoMatchLevel>,
    /// The URL we call on each interaction
    #[serde(rename = "intercept_callback_url", skip_serializing_if = "Option::is_none")]
    pub intercept_callback_url: Option<String>,
    /// The URLs of resources related to the Service
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The preference for Proxy Number selection for the Service instance
    #[serde(rename = "number_selection_behavior", skip_serializing_if = "Option::is_none")]
    pub number_selection_behavior: Option<NumberSelectionBehavior>,
    /// The URL we call when an inbound call or SMS action occurs on a closed or non-existent Session
    #[serde(rename = "out_of_session_callback_url", skip_serializing_if = "Option::is_none")]
    pub out_of_session_callback_url: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Service resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ProxyV1Service {
    pub fn new() -> ProxyV1Service {
        ProxyV1Service {
            account_sid: None,
            callback_url: None,
            chat_instance_sid: None,
            date_created: None,
            date_updated: None,
            default_ttl: None,
            geo_match_level: None,
            intercept_callback_url: None,
            links: None,
            number_selection_behavior: None,
            out_of_session_callback_url: None,
            sid: None,
            unique_name: None,
            url: None,
        }
    }
}

/// Where a proxy number must be located relative to the participant identifier
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GeoMatchLevel {
    #[serde(rename = "area-code")]
    AreaCode,
    #[serde(rename = "overlay")]
    Overlay,
    #[serde(rename = "radius")]
    Radius,
    #[serde(rename = "country")]
    Country,
}
/// The preference for Proxy Number selection for the Service instance
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NumberSelectionBehavior {
    #[serde(rename = "avoid-sticky")]
    AvoidSticky,
    #[serde(rename = "prefer-sticky")]
    PreferSticky,
}

