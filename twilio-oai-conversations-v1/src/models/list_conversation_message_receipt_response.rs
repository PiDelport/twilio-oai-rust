/*
 * Twilio - Conversations
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListConversationMessageReceiptResponse {
    #[serde(rename = "delivery_receipts", skip_serializing_if = "Option::is_none")]
    pub delivery_receipts: Option<Vec<crate::models::ConversationsV1ConversationConversationMessageConversationMessageReceipt>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConversationResponseMeta>>,
}

impl ListConversationMessageReceiptResponse {
    pub fn new() -> ListConversationMessageReceiptResponse {
        ListConversationMessageReceiptResponse {
            delivery_receipts: None,
            meta: None,
        }
    }
}

