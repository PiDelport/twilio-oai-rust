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
pub struct SupersimV1Sim {
    /// The SID of the Account that the Super SIM belongs to
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The unique ID of the Fleet configured for this SIM
    #[serde(rename = "fleet_sid", skip_serializing_if = "Option::is_none")]
    pub fleet_sid: Option<String>,
    /// The ICCID associated with the SIM
    #[serde(rename = "iccid", skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of the Super SIM
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Sim Resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SupersimV1Sim {
    pub fn new() -> SupersimV1Sim {
        SupersimV1Sim {
            account_sid: None,
            date_created: None,
            date_updated: None,
            fleet_sid: None,
            iccid: None,
            links: None,
            sid: None,
            status: None,
            unique_name: None,
            url: None,
        }
    }
}

/// The status of the Super SIM
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "scheduled")]
    Scheduled,
}

