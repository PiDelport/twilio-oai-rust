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
pub struct AutopilotV1AssistantTaskField {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The SID of the Assistant that is the parent of the Task associated with the resource
    #[serde(rename = "assistant_sid", skip_serializing_if = "Option::is_none")]
    pub assistant_sid: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The Field Type of the field
    #[serde(rename = "field_type", skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SID of the [Task](https://www.twilio.com/docs/autopilot/api/task) resource associated with this Field
    #[serde(rename = "task_sid", skip_serializing_if = "Option::is_none")]
    pub task_sid: Option<String>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Field resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AutopilotV1AssistantTaskField {
    pub fn new() -> AutopilotV1AssistantTaskField {
        AutopilotV1AssistantTaskField {
            account_sid: None,
            assistant_sid: None,
            date_created: None,
            date_updated: None,
            field_type: None,
            sid: None,
            task_sid: None,
            unique_name: None,
            url: None,
        }
    }
}

