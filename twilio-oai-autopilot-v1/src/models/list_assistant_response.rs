/*
 * Twilio - Autopilot
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListAssistantResponse {
    #[serde(rename = "assistants", skip_serializing_if = "Option::is_none")]
    pub assistants: Option<Vec<crate::models::AutopilotV1Assistant>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListAssistantResponseMeta>>,
}

impl ListAssistantResponse {
    pub fn new() -> ListAssistantResponse {
        ListAssistantResponse {
            assistants: None,
            meta: None,
        }
    }
}

