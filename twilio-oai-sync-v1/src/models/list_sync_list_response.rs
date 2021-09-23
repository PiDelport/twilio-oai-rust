/*
 * Twilio - Sync
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSyncListResponse {
    #[serde(rename = "lists", skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<crate::models::SyncV1ServiceSyncList>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
}

impl ListSyncListResponse {
    pub fn new() -> ListSyncListResponse {
        ListSyncListResponse {
            lists: None,
            meta: None,
        }
    }
}


