/*
 * Twilio - Wireless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListUsageRecordResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCommandResponseMeta>>,
    #[serde(rename = "usage_records", skip_serializing_if = "Option::is_none")]
    pub usage_records: Option<Vec<crate::models::WirelessV1SimUsageRecord>>,
}

impl ListUsageRecordResponse {
    pub fn new() -> ListUsageRecordResponse {
        ListUsageRecordResponse {
            meta: None,
            usage_records: None,
        }
    }
}


