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
pub struct SyncV1ServiceSyncMapSyncMapItem {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The identity of the Map Item's creator
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// An arbitrary, schema-less object that the Map Item stores
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the Map Item expires
    #[serde(rename = "date_expires", skip_serializing_if = "Option::is_none")]
    pub date_expires: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The unique, user-defined key for the Map Item
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The SID of the Sync Map that contains the Map Item
    #[serde(rename = "map_sid", skip_serializing_if = "Option::is_none")]
    pub map_sid: Option<String>,
    /// The current revision of the Map Item, represented as a string
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// The SID of the Sync Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The absolute URL of the Map Item resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SyncV1ServiceSyncMapSyncMapItem {
    pub fn new() -> SyncV1ServiceSyncMapSyncMapItem {
        SyncV1ServiceSyncMapSyncMapItem {
            account_sid: None,
            created_by: None,
            data: None,
            date_created: None,
            date_expires: None,
            date_updated: None,
            key: None,
            map_sid: None,
            revision: None,
            service_sid: None,
            url: None,
        }
    }
}
