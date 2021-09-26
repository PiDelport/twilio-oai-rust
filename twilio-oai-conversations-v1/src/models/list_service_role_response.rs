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
pub struct ListServiceRoleResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConversationResponseMeta>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::models::ConversationsV1ServiceServiceRole>>,
}

impl ListServiceRoleResponse {
    pub fn new() -> ListServiceRoleResponse {
        ListServiceRoleResponse {
            meta: None,
            roles: None,
        }
    }
}
