/*
 * Twilio - Verify
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyV2ServiceMessagingConfiguration {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The ISO-3166-1 country code of the country or `all`.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The SID of the Messaging Service used for this configuration.
    #[serde(rename = "messaging_service_sid", skip_serializing_if = "Option::is_none")]
    pub messaging_service_sid: Option<String>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The URL of this resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VerifyV2ServiceMessagingConfiguration {
    pub fn new() -> VerifyV2ServiceMessagingConfiguration {
        VerifyV2ServiceMessagingConfiguration {
            account_sid: None,
            country: None,
            date_created: None,
            date_updated: None,
            messaging_service_sid: None,
            service_sid: None,
            url: None,
        }
    }
}


