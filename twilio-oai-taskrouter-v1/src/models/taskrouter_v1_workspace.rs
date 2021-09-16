/*
 * Twilio - Taskrouter
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskrouterV1Workspace {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The name of the default activity
    #[serde(rename = "default_activity_name", skip_serializing_if = "Option::is_none")]
    pub default_activity_name: Option<String>,
    /// The SID of the Activity that will be used when new Workers are created in the Workspace
    #[serde(rename = "default_activity_sid", skip_serializing_if = "Option::is_none")]
    pub default_activity_sid: Option<String>,
    /// The URL we call when an event occurs
    #[serde(rename = "event_callback_url", skip_serializing_if = "Option::is_none")]
    pub event_callback_url: Option<String>,
    /// The list of Workspace events for which to call event_callback_url
    #[serde(rename = "events_filter", skip_serializing_if = "Option::is_none")]
    pub events_filter: Option<String>,
    /// The string that you assigned to describe the Workspace resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Whether multi-tasking is enabled
    #[serde(rename = "multi_task_enabled", skip_serializing_if = "Option::is_none")]
    pub multi_task_enabled: Option<bool>,
    /// The type of TaskQueue to prioritize when Workers are receiving Tasks from both types of TaskQueues
    #[serde(rename = "prioritize_queue_order", skip_serializing_if = "Option::is_none")]
    pub prioritize_queue_order: Option<PrioritizeQueueOrder>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The name of the timeout activity
    #[serde(rename = "timeout_activity_name", skip_serializing_if = "Option::is_none")]
    pub timeout_activity_name: Option<String>,
    /// The SID of the Activity that will be assigned to a Worker when a Task reservation times out without a response
    #[serde(rename = "timeout_activity_sid", skip_serializing_if = "Option::is_none")]
    pub timeout_activity_sid: Option<String>,
    /// The absolute URL of the Workspace resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TaskrouterV1Workspace {
    pub fn new() -> TaskrouterV1Workspace {
        TaskrouterV1Workspace {
            account_sid: None,
            date_created: None,
            date_updated: None,
            default_activity_name: None,
            default_activity_sid: None,
            event_callback_url: None,
            events_filter: None,
            friendly_name: None,
            links: None,
            multi_task_enabled: None,
            prioritize_queue_order: None,
            sid: None,
            timeout_activity_name: None,
            timeout_activity_sid: None,
            url: None,
        }
    }
}

/// The type of TaskQueue to prioritize when Workers are receiving Tasks from both types of TaskQueues
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PrioritizeQueueOrder {
    #[serde(rename = "FIFO")]
    FIFO,
    #[serde(rename = "LIFO")]
    LIFO,
}
