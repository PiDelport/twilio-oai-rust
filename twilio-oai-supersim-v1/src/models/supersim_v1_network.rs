/*
 * Twilio - Supersim
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SupersimV1Network {
    /// A human readable identifier of this resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The MCC/MNCs included in the Network resource
    #[serde(rename = "identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<serde_json::Value>>,
    /// The ISO country code of the Network resource
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Network resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SupersimV1Network {
    pub fn new() -> SupersimV1Network {
        SupersimV1Network {
            friendly_name: None,
            identifiers: None,
            iso_country: None,
            sid: None,
            url: None,
        }
    }
}


