/*
 * Twilio - Voice
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VoiceV1ByocTrunk {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Whether Caller ID Name (CNAM) lookup is enabled for the trunk
    #[serde(
        rename = "cnam_lookup_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub cnam_lookup_enabled: Option<bool>,
    /// Origination Connection Policy (to your Carrier)
    #[serde(
        rename = "connection_policy_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_policy_sid: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The SID of the SIP Domain that should be used in the `From` header of originating calls
    #[serde(rename = "from_domain_sid", skip_serializing_if = "Option::is_none")]
    pub from_domain_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The HTTP method we use to call status_callback_url
    #[serde(
        rename = "status_callback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_method: Option<StatusCallbackMethod>,
    /// The URL that we call with status updates
    #[serde(
        rename = "status_callback_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_url: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The HTTP method used with voice_fallback_url
    #[serde(
        rename = "voice_fallback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_fallback_method: Option<VoiceFallbackMethod>,
    /// The URL we call when an error occurs while executing TwiML
    #[serde(rename = "voice_fallback_url", skip_serializing_if = "Option::is_none")]
    pub voice_fallback_url: Option<String>,
    /// The HTTP method to use with voice_url
    #[serde(rename = "voice_method", skip_serializing_if = "Option::is_none")]
    pub voice_method: Option<VoiceMethod>,
    /// The URL we call when receiving a call
    #[serde(rename = "voice_url", skip_serializing_if = "Option::is_none")]
    pub voice_url: Option<String>,
}

impl VoiceV1ByocTrunk {
    pub fn new() -> VoiceV1ByocTrunk {
        VoiceV1ByocTrunk {
            account_sid: None,
            cnam_lookup_enabled: None,
            connection_policy_sid: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            from_domain_sid: None,
            sid: None,
            status_callback_method: None,
            status_callback_url: None,
            url: None,
            voice_fallback_method: None,
            voice_fallback_url: None,
            voice_method: None,
            voice_url: None,
        }
    }
}

/// The HTTP method we use to call status_callback_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCallbackMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}
/// The HTTP method used with voice_fallback_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceFallbackMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}
/// The HTTP method to use with voice_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}
