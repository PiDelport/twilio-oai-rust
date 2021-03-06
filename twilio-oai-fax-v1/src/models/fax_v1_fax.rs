/*
 * Twilio - Fax
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FaxV1Fax {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The API version used to transmit the fax
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The ISO 8601 formatted date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 formatted date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The direction of the fax
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// The time it took to transmit the fax
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// The number the fax was sent from
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The URLs of the fax's related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The SID of the FaxMedia resource that is associated with the Fax
    #[serde(rename = "media_sid", skip_serializing_if = "Option::is_none")]
    pub media_sid: Option<String>,
    /// The Twilio-hosted URL that can be used to download fax media
    #[serde(rename = "media_url", skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    /// The number of pages contained in the fax document
    #[serde(rename = "num_pages", skip_serializing_if = "Option::is_none")]
    pub num_pages: Option<i32>,
    /// The fax transmission price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// The ISO 4217 currency used for billing
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The quality of the fax
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Quality>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of the fax
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The phone number that received the fax
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The absolute URL of the fax resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FaxV1Fax {
    pub fn new() -> FaxV1Fax {
        FaxV1Fax {
            account_sid: None,
            api_version: None,
            date_created: None,
            date_updated: None,
            direction: None,
            duration: None,
            from: None,
            links: None,
            media_sid: None,
            media_url: None,
            num_pages: None,
            price: None,
            price_unit: None,
            quality: None,
            sid: None,
            status: None,
            to: None,
            url: None,
        }
    }
}

/// The direction of the fax
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
}
/// The quality of the fax
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Quality {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "fine")]
    Fine,
    #[serde(rename = "superfine")]
    Superfine,
}
/// The status of the fax
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "receiving")]
    Receiving,
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "no-answer")]
    NoAnswer,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "canceled")]
    Canceled,
}
