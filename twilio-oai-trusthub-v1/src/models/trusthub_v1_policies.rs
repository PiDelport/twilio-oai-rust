/*
 * Twilio - Trusthub
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrusthubV1Policies {
    /// A human-readable description of the Policy resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The sid of a Policy object that dictates requirements
    #[serde(rename = "requirements", skip_serializing_if = "Option::is_none")]
    pub requirements: Option<serde_json::Value>,
    /// The unique string that identifies the Policy resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Policy resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TrusthubV1Policies {
    pub fn new() -> TrusthubV1Policies {
        TrusthubV1Policies {
            friendly_name: None,
            requirements: None,
            sid: None,
            url: None,
        }
    }
}
