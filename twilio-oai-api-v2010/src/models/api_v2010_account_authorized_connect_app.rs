/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountAuthorizedConnectApp {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The company name set for the Connect App
    #[serde(rename = "connect_app_company_name", skip_serializing_if = "Option::is_none")]
    pub connect_app_company_name: Option<String>,
    /// A detailed description of the app
    #[serde(rename = "connect_app_description", skip_serializing_if = "Option::is_none")]
    pub connect_app_description: Option<String>,
    /// The name of the Connect App
    #[serde(rename = "connect_app_friendly_name", skip_serializing_if = "Option::is_none")]
    pub connect_app_friendly_name: Option<String>,
    /// The public URL for the Connect App
    #[serde(rename = "connect_app_homepage_url", skip_serializing_if = "Option::is_none")]
    pub connect_app_homepage_url: Option<String>,
    /// The SID that we assigned to the Connect App
    #[serde(rename = "connect_app_sid", skip_serializing_if = "Option::is_none")]
    pub connect_app_sid: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Permissions authorized to the app
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permissions>>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountAuthorizedConnectApp {
    pub fn new() -> ApiV2010AccountAuthorizedConnectApp {
        ApiV2010AccountAuthorizedConnectApp {
            account_sid: None,
            connect_app_company_name: None,
            connect_app_description: None,
            connect_app_friendly_name: None,
            connect_app_homepage_url: None,
            connect_app_sid: None,
            date_created: None,
            date_updated: None,
            permissions: None,
            uri: None,
        }
    }
}

/// Permissions authorized to the app
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "get-all")]
    GetAll,
    #[serde(rename = "post-all")]
    PostAll,
}
