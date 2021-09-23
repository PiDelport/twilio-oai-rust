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
pub struct ListServiceConversationMessageResponse {
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::ConversationsV1ServiceServiceConversationServiceConversationMessage>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConversationResponseMeta>>,
}

impl ListServiceConversationMessageResponse {
    pub fn new() -> ListServiceConversationMessageResponse {
        ListServiceConversationMessageResponse {
            messages: None,
            meta: None,
        }
    }
}


