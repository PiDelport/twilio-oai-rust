/*
 * Twilio - Trusthub
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrusthubV1EndUser {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The set of parameters that compose the End Users resource
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The type of end user of the Bundle resource
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The absolute URL of the End User resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TrusthubV1EndUser {
    pub fn new() -> TrusthubV1EndUser {
        TrusthubV1EndUser {
            account_sid: None,
            attributes: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            sid: None,
            _type: None,
            url: None,
        }
    }
}


