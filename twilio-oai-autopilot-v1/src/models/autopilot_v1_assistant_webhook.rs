/*
 * Twilio - Autopilot
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutopilotV1AssistantWebhook {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The SID of the Assistant that is the parent of the resource
    #[serde(rename = "assistant_sid", skip_serializing_if = "Option::is_none")]
    pub assistant_sid: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The list of space-separated events that this Webhook is subscribed to.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Webhook resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The method used when calling the webhook's URL.
    #[serde(rename = "webhook_method", skip_serializing_if = "Option::is_none")]
    pub webhook_method: Option<String>,
    /// The URL associated with this Webhook.
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

impl AutopilotV1AssistantWebhook {
    pub fn new() -> AutopilotV1AssistantWebhook {
        AutopilotV1AssistantWebhook {
            account_sid: None,
            assistant_sid: None,
            date_created: None,
            date_updated: None,
            events: None,
            sid: None,
            unique_name: None,
            url: None,
            webhook_method: None,
            webhook_url: None,
        }
    }
}


