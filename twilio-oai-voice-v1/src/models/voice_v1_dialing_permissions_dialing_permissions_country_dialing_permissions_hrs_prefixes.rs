/*
 * Twilio - Voice
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VoiceV1DialingPermissionsDialingPermissionsCountryDialingPermissionsHrsPrefixes {
    /// A prefix that includes the E.164 assigned country code
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl VoiceV1DialingPermissionsDialingPermissionsCountryDialingPermissionsHrsPrefixes {
    pub fn new() -> VoiceV1DialingPermissionsDialingPermissionsCountryDialingPermissionsHrsPrefixes {
        VoiceV1DialingPermissionsDialingPermissionsCountryDialingPermissionsHrsPrefixes {
            prefix: None,
        }
    }
}


