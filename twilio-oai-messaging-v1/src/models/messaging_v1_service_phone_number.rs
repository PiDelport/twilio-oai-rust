/*
 * Twilio - Messaging
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessagingV1ServicePhoneNumber {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// An array of values that describe whether the number can receive calls or messages
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    /// The 2-character ISO Country Code of the number
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The phone number in E.164 format
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the PhoneNumber resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl MessagingV1ServicePhoneNumber {
    pub fn new() -> MessagingV1ServicePhoneNumber {
        MessagingV1ServicePhoneNumber {
            account_sid: None,
            capabilities: None,
            country_code: None,
            date_created: None,
            date_updated: None,
            phone_number: None,
            service_sid: None,
            sid: None,
            url: None,
        }
    }
}


