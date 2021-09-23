/*
 * Twilio - Conversations
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListConversationScopedWebhookResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConversationResponseMeta>>,
    #[serde(rename = "webhooks", skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<crate::models::ConversationsV1ConversationConversationScopedWebhook>>,
}

impl ListConversationScopedWebhookResponse {
    pub fn new() -> ListConversationScopedWebhookResponse {
        ListConversationScopedWebhookResponse {
            meta: None,
            webhooks: None,
        }
    }
}


