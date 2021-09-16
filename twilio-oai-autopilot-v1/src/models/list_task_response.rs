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
pub struct ListTaskResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListAssistantResponseMeta>>,
    #[serde(rename = "tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<crate::models::AutopilotV1AssistantTask>>,
}

impl ListTaskResponse {
    pub fn new() -> ListTaskResponse {
        ListTaskResponse {
            meta: None,
            tasks: None,
        }
    }
}

