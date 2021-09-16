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
pub struct VerifyV2ServiceVerification {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The amount of the associated PSD2 compliant transaction.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The verification method used.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Information about the phone number being verified
    #[serde(rename = "lookup", skip_serializing_if = "Option::is_none")]
    pub lookup: Option<serde_json::Value>,
    /// The payee of the associated PSD2 compliant transaction
    #[serde(rename = "payee", skip_serializing_if = "Option::is_none")]
    pub payee: Option<String>,
    /// An array of verification attempt objects.
    #[serde(rename = "send_code_attempts", skip_serializing_if = "Option::is_none")]
    pub send_code_attempts: Option<Vec<serde_json::Value>>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of the verification resource
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The phone number or email being verified
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The absolute URL of the Verification resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether the verification was successful
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

impl VerifyV2ServiceVerification {
    pub fn new() -> VerifyV2ServiceVerification {
        VerifyV2ServiceVerification {
            account_sid: None,
            amount: None,
            channel: None,
            date_created: None,
            date_updated: None,
            lookup: None,
            payee: None,
            send_code_attempts: None,
            service_sid: None,
            sid: None,
            status: None,
            to: None,
            url: None,
            valid: None,
        }
    }
}

/// The verification method used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "whatsapp")]
    Whatsapp,
}

