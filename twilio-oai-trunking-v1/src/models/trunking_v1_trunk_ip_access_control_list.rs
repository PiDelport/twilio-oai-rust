/*
 * Twilio - Trunking
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrunkingV1TrunkIpAccessControlList {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SID of the Trunk the resource is associated with
    #[serde(rename = "trunk_sid", skip_serializing_if = "Option::is_none")]
    pub trunk_sid: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TrunkingV1TrunkIpAccessControlList {
    pub fn new() -> TrunkingV1TrunkIpAccessControlList {
        TrunkingV1TrunkIpAccessControlList {
            account_sid: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            sid: None,
            trunk_sid: None,
            url: None,
        }
    }
}


