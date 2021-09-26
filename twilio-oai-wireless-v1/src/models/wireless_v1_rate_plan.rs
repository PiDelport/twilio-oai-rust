/*
 * Twilio - Wireless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WirelessV1RatePlan {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Whether SIMs can use GPRS/3G/4G/LTE data connectivity
    #[serde(rename = "data_enabled", skip_serializing_if = "Option::is_none")]
    pub data_enabled: Option<bool>,
    /// The total data usage in Megabytes that the Network allows during one month on the home network
    #[serde(rename = "data_limit", skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i32>,
    /// The model used to meter data usage
    #[serde(rename = "data_metering", skip_serializing_if = "Option::is_none")]
    pub data_metering: Option<String>,
    /// The date when the resource was created, given as GMT in ISO 8601 format
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date when the resource was last updated, given as GMT in ISO 8601 format
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The services that SIMs capable of using GPRS/3G/4G/LTE data connectivity can use outside of the United States
    #[serde(
        rename = "international_roaming",
        skip_serializing_if = "Option::is_none"
    )]
    pub international_roaming: Option<Vec<String>>,
    /// The total data usage (download and upload combined) in Megabytes that the Network allows during one month when roaming outside the United States
    #[serde(
        rename = "international_roaming_data_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub international_roaming_data_limit: Option<i32>,
    /// Whether SIMs can make, send, and receive SMS using Commands
    #[serde(rename = "messaging_enabled", skip_serializing_if = "Option::is_none")]
    pub messaging_enabled: Option<bool>,
    /// The total data usage in Megabytes that the Network allows during one month on non-home networks in the United States
    #[serde(
        rename = "national_roaming_data_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub national_roaming_data_limit: Option<i32>,
    /// Whether SIMs can roam on networks other than the home network in the United States
    #[serde(
        rename = "national_roaming_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub national_roaming_enabled: Option<bool>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether SIMs can make and receive voice calls
    #[serde(rename = "voice_enabled", skip_serializing_if = "Option::is_none")]
    pub voice_enabled: Option<bool>,
}

impl WirelessV1RatePlan {
    pub fn new() -> WirelessV1RatePlan {
        WirelessV1RatePlan {
            account_sid: None,
            data_enabled: None,
            data_limit: None,
            data_metering: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            international_roaming: None,
            international_roaming_data_limit: None,
            messaging_enabled: None,
            national_roaming_data_limit: None,
            national_roaming_enabled: None,
            sid: None,
            unique_name: None,
            url: None,
            voice_enabled: None,
        }
    }
}
