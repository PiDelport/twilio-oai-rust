/*
 * Twilio - Trunking
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrunkingV1TrunkOriginationUrl {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Whether the URL is enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The relative importance of the URI
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SIP address you want Twilio to route your Origination calls to
    #[serde(rename = "sip_url", skip_serializing_if = "Option::is_none")]
    pub sip_url: Option<String>,
    /// The SID of the Trunk that owns the Origination URL
    #[serde(rename = "trunk_sid", skip_serializing_if = "Option::is_none")]
    pub trunk_sid: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The value that determines the relative load the URI should receive compared to others with the same priority
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl TrunkingV1TrunkOriginationUrl {
    pub fn new() -> TrunkingV1TrunkOriginationUrl {
        TrunkingV1TrunkOriginationUrl {
            account_sid: None,
            date_created: None,
            date_updated: None,
            enabled: None,
            friendly_name: None,
            priority: None,
            sid: None,
            sip_url: None,
            trunk_sid: None,
            url: None,
            weight: None,
        }
    }
}


