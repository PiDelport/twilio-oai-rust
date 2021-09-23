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
pub struct VoiceV1DialingPermissionsDialingPermissionsSettings {
    /// `true` if the sub-account will inherit voice dialing permissions from the Master Project; otherwise `false`
    #[serde(rename = "dialing_permissions_inheritance", skip_serializing_if = "Option::is_none")]
    pub dialing_permissions_inheritance: Option<bool>,
    /// The absolute URL of this resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VoiceV1DialingPermissionsDialingPermissionsSettings {
    pub fn new() -> VoiceV1DialingPermissionsDialingPermissionsSettings {
        VoiceV1DialingPermissionsDialingPermissionsSettings {
            dialing_permissions_inheritance: None,
            url: None,
        }
    }
}


