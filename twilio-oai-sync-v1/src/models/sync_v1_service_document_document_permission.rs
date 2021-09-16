/*
 * Twilio - Sync
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SyncV1ServiceDocumentDocumentPermission {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The Sync Document SID
    #[serde(rename = "document_sid", skip_serializing_if = "Option::is_none")]
    pub document_sid: Option<String>,
    /// The identity of the user to whom the Sync Document Permission applies
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Manage access
    #[serde(rename = "manage", skip_serializing_if = "Option::is_none")]
    pub manage: Option<bool>,
    /// Read access
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// The SID of the Sync Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The absolute URL of the Sync Document Permission resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Write access
    #[serde(rename = "write", skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
}

impl SyncV1ServiceDocumentDocumentPermission {
    pub fn new() -> SyncV1ServiceDocumentDocumentPermission {
        SyncV1ServiceDocumentDocumentPermission {
            account_sid: None,
            document_sid: None,
            identity: None,
            manage: None,
            read: None,
            service_sid: None,
            url: None,
            write: None,
        }
    }
}

