/*
 * Twilio - Verify
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyV2Service {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The length of the verification code
    #[serde(rename = "code_length", skip_serializing_if = "Option::is_none")]
    pub code_length: Option<i32>,
    /// Whether to allow sending verifications with a custom code.
    #[serde(rename = "custom_code_enabled", skip_serializing_if = "Option::is_none")]
    pub custom_code_enabled: Option<bool>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Whether to add a security warning at the end of an SMS.
    #[serde(rename = "do_not_share_warning_enabled", skip_serializing_if = "Option::is_none")]
    pub do_not_share_warning_enabled: Option<bool>,
    /// Whether to ask the user to press a number before delivering the verify code in a phone call
    #[serde(rename = "dtmf_input_required", skip_serializing_if = "Option::is_none")]
    pub dtmf_input_required: Option<bool>,
    /// The string that you assigned to describe the verification service
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Whether to perform a lookup with each verification
    #[serde(rename = "lookup_enabled", skip_serializing_if = "Option::is_none")]
    pub lookup_enabled: Option<bool>,
    /// Whether to pass PSD2 transaction parameters when starting a verification
    #[serde(rename = "psd2_enabled", skip_serializing_if = "Option::is_none")]
    pub psd2_enabled: Option<bool>,
    /// The service level configuration of factor push type.
    #[serde(rename = "push", skip_serializing_if = "Option::is_none")]
    pub push: Option<serde_json::Value>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Whether to skip sending SMS verifications to landlines
    #[serde(rename = "skip_sms_to_landlines", skip_serializing_if = "Option::is_none")]
    pub skip_sms_to_landlines: Option<bool>,
    /// The service level configuration of factor TOTP type.
    #[serde(rename = "totp", skip_serializing_if = "Option::is_none")]
    pub totp: Option<serde_json::Value>,
    /// The name of an alternative text-to-speech service to use in phone calls
    #[serde(rename = "tts_name", skip_serializing_if = "Option::is_none")]
    pub tts_name: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VerifyV2Service {
    pub fn new() -> VerifyV2Service {
        VerifyV2Service {
            account_sid: None,
            code_length: None,
            custom_code_enabled: None,
            date_created: None,
            date_updated: None,
            do_not_share_warning_enabled: None,
            dtmf_input_required: None,
            friendly_name: None,
            links: None,
            lookup_enabled: None,
            psd2_enabled: None,
            push: None,
            sid: None,
            skip_sms_to_landlines: None,
            totp: None,
            tts_name: None,
            url: None,
        }
    }
}


