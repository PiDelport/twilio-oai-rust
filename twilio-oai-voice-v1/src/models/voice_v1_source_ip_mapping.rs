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
pub struct VoiceV1SourceIpMapping {
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The unique string that identifies an IP Record
    #[serde(rename = "ip_record_sid", skip_serializing_if = "Option::is_none")]
    pub ip_record_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The unique string that identifies a SIP Domain
    #[serde(rename = "sip_domain_sid", skip_serializing_if = "Option::is_none")]
    pub sip_domain_sid: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VoiceV1SourceIpMapping {
    pub fn new() -> VoiceV1SourceIpMapping {
        VoiceV1SourceIpMapping {
            date_created: None,
            date_updated: None,
            ip_record_sid: None,
            sid: None,
            sip_domain_sid: None,
            url: None,
        }
    }
}
