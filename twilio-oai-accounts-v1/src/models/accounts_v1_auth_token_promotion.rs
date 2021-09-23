/*
 * Twilio - Accounts
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountsV1AuthTokenPromotion {
    /// The SID of the Account that the secondary Auth Token was created for
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The promoted Auth Token
    #[serde(rename = "auth_token", skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// The ISO 8601 formatted date and time in UTC when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 formatted date and time in UTC when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The URI for this resource, relative to `https://accounts.twilio.com`
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AccountsV1AuthTokenPromotion {
    pub fn new() -> AccountsV1AuthTokenPromotion {
        AccountsV1AuthTokenPromotion {
            account_sid: None,
            auth_token: None,
            date_created: None,
            date_updated: None,
            url: None,
        }
    }
}


