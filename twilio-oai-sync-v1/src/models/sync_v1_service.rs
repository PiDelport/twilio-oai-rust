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
pub struct SyncV1Service {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Whether token identities in the Service must be granted access to Sync objects by using the Permissions resource
    #[serde(rename = "acl_enabled", skip_serializing_if = "Option::is_none")]
    pub acl_enabled: Option<bool>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Whether every endpoint_disconnected event occurs after a configurable delay
    #[serde(
        rename = "reachability_debouncing_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub reachability_debouncing_enabled: Option<bool>,
    /// The reachability event delay in milliseconds
    #[serde(
        rename = "reachability_debouncing_window",
        skip_serializing_if = "Option::is_none"
    )]
    pub reachability_debouncing_window: Option<i32>,
    /// Whether the service instance calls webhook_url when client endpoints connect to Sync
    #[serde(
        rename = "reachability_webhooks_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub reachability_webhooks_enabled: Option<bool>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Service resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The URL we call when Sync objects are manipulated
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// Whether the Service instance should call webhook_url when the REST API is used to update Sync objects
    #[serde(
        rename = "webhooks_from_rest_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub webhooks_from_rest_enabled: Option<bool>,
}

impl SyncV1Service {
    pub fn new() -> SyncV1Service {
        SyncV1Service {
            account_sid: None,
            acl_enabled: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            links: None,
            reachability_debouncing_enabled: None,
            reachability_debouncing_window: None,
            reachability_webhooks_enabled: None,
            sid: None,
            unique_name: None,
            url: None,
            webhook_url: None,
            webhooks_from_rest_enabled: None,
        }
    }
}
