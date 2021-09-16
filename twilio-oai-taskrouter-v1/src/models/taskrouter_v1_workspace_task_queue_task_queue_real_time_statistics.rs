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
pub struct TaskrouterV1WorkspaceTaskQueueTaskQueueRealTimeStatistics {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The number of current Workers by Activity
    #[serde(rename = "activity_statistics", skip_serializing_if = "Option::is_none")]
    pub activity_statistics: Option<Vec<serde_json::Value>>,
    /// The relative age in the TaskQueue for the longest waiting Task.
    #[serde(rename = "longest_relative_task_age_in_queue", skip_serializing_if = "Option::is_none")]
    pub longest_relative_task_age_in_queue: Option<i32>,
    /// The SID of the Task waiting in the TaskQueue the longest.
    #[serde(rename = "longest_relative_task_sid_in_queue", skip_serializing_if = "Option::is_none")]
    pub longest_relative_task_sid_in_queue: Option<String>,
    /// The age of the longest waiting Task
    #[serde(rename = "longest_task_waiting_age", skip_serializing_if = "Option::is_none")]
    pub longest_task_waiting_age: Option<i32>,
    /// The SID of the longest waiting Task
    #[serde(rename = "longest_task_waiting_sid", skip_serializing_if = "Option::is_none")]
    pub longest_task_waiting_sid: Option<String>,
    /// The SID of the TaskQueue from which these statistics were calculated
    #[serde(rename = "task_queue_sid", skip_serializing_if = "Option::is_none")]
    pub task_queue_sid: Option<String>,
    /// The number of Tasks by priority
    #[serde(rename = "tasks_by_priority", skip_serializing_if = "Option::is_none")]
    pub tasks_by_priority: Option<serde_json::Value>,
    /// The number of Tasks by their current status
    #[serde(rename = "tasks_by_status", skip_serializing_if = "Option::is_none")]
    pub tasks_by_status: Option<serde_json::Value>,
    /// The total number of Workers available for Tasks in the TaskQueue
    #[serde(rename = "total_available_workers", skip_serializing_if = "Option::is_none")]
    pub total_available_workers: Option<i32>,
    /// The total number of Workers eligible for Tasks in the TaskQueue, independent of their Activity state
    #[serde(rename = "total_eligible_workers", skip_serializing_if = "Option::is_none")]
    pub total_eligible_workers: Option<i32>,
    /// The total number of Tasks
    #[serde(rename = "total_tasks", skip_serializing_if = "Option::is_none")]
    pub total_tasks: Option<i32>,
    /// The absolute URL of the TaskQueue statistics resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the Workspace that contains the TaskQueue
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceTaskQueueTaskQueueRealTimeStatistics {
    pub fn new() -> TaskrouterV1WorkspaceTaskQueueTaskQueueRealTimeStatistics {
        TaskrouterV1WorkspaceTaskQueueTaskQueueRealTimeStatistics {
            account_sid: None,
            activity_statistics: None,
            longest_relative_task_age_in_queue: None,
            longest_relative_task_sid_in_queue: None,
            longest_task_waiting_age: None,
            longest_task_waiting_sid: None,
            task_queue_sid: None,
            tasks_by_priority: None,
            tasks_by_status: None,
            total_available_workers: None,
            total_eligible_workers: None,
            total_tasks: None,
            url: None,
            workspace_sid: None,
        }
    }
}


