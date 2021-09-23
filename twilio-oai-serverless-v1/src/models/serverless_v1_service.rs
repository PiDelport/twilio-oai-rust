/*
 * Twilio - Serverless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerlessV1Service {
    /// The SID of the Account that created the Service resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the Service resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the Service resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the Service resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Whether to inject Account credentials into a function invocation context
    #[serde(rename = "include_credentials", skip_serializing_if = "Option::is_none")]
    pub include_credentials: Option<bool>,
    /// The URLs of the Service's nested resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The unique string that identifies the Service resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Whether the Service resource's properties and subresources can be edited via the UI
    #[serde(rename = "ui_editable", skip_serializing_if = "Option::is_none")]
    pub ui_editable: Option<bool>,
    /// A user-defined string that uniquely identifies the Service resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Service resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ServerlessV1Service {
    pub fn new() -> ServerlessV1Service {
        ServerlessV1Service {
            account_sid: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            include_credentials: None,
            links: None,
            sid: None,
            ui_editable: None,
            unique_name: None,
            url: None,
        }
    }
}


