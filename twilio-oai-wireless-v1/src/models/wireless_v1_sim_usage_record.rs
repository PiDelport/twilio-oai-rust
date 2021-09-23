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
pub struct WirelessV1SimUsageRecord {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// An object that describes the SIM's usage of Commands during the specified period
    #[serde(rename = "commands", skip_serializing_if = "Option::is_none")]
    pub commands: Option<serde_json::Value>,
    /// An object that describes the SIM's data usage during the specified period
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// The time period for which the usage is reported
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<serde_json::Value>,
    /// The SID of the Sim resource that this Usage Record is for
    #[serde(rename = "sim_sid", skip_serializing_if = "Option::is_none")]
    pub sim_sid: Option<String>,
}

impl WirelessV1SimUsageRecord {
    pub fn new() -> WirelessV1SimUsageRecord {
        WirelessV1SimUsageRecord {
            account_sid: None,
            commands: None,
            data: None,
            period: None,
            sim_sid: None,
        }
    }
}


