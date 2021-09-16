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
pub struct TaskrouterV1WorkspaceTaskQueueTaskQueueCumulativeStatistics {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The average time in seconds between Task creation and acceptance
    #[serde(rename = "avg_task_acceptance_time", skip_serializing_if = "Option::is_none")]
    pub avg_task_acceptance_time: Option<i32>,
    /// The end of the interval during which these statistics were calculated
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The total number of Reservations accepted for Tasks in the TaskQueue
    #[serde(rename = "reservations_accepted", skip_serializing_if = "Option::is_none")]
    pub reservations_accepted: Option<i32>,
    /// The total number of Reservations canceled for Tasks in the TaskQueue
    #[serde(rename = "reservations_canceled", skip_serializing_if = "Option::is_none")]
    pub reservations_canceled: Option<i32>,
    /// The total number of Reservations created for Tasks in the TaskQueue
    #[serde(rename = "reservations_created", skip_serializing_if = "Option::is_none")]
    pub reservations_created: Option<i32>,
    /// The total number of Reservations rejected for Tasks in the TaskQueue
    #[serde(rename = "reservations_rejected", skip_serializing_if = "Option::is_none")]
    pub reservations_rejected: Option<i32>,
    /// The total number of Reservations rescinded
    #[serde(rename = "reservations_rescinded", skip_serializing_if = "Option::is_none")]
    pub reservations_rescinded: Option<i32>,
    /// The total number of Reservations that timed out for Tasks in the TaskQueue
    #[serde(rename = "reservations_timed_out", skip_serializing_if = "Option::is_none")]
    pub reservations_timed_out: Option<i32>,
    /// A list of objects that describe the Tasks canceled and reservations accepted above and below the specified thresholds
    #[serde(rename = "split_by_wait_time", skip_serializing_if = "Option::is_none")]
    pub split_by_wait_time: Option<serde_json::Value>,
    /// The beginning of the interval during which these statistics were calculated
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The SID of the TaskQueue from which these statistics were calculated
    #[serde(rename = "task_queue_sid", skip_serializing_if = "Option::is_none")]
    pub task_queue_sid: Option<String>,
    /// The total number of Tasks canceled in the TaskQueue
    #[serde(rename = "tasks_canceled", skip_serializing_if = "Option::is_none")]
    pub tasks_canceled: Option<i32>,
    /// The total number of Tasks completed in the TaskQueue
    #[serde(rename = "tasks_completed", skip_serializing_if = "Option::is_none")]
    pub tasks_completed: Option<i32>,
    /// The total number of Tasks deleted in the TaskQueue
    #[serde(rename = "tasks_deleted", skip_serializing_if = "Option::is_none")]
    pub tasks_deleted: Option<i32>,
    /// The total number of Tasks entered into the TaskQueue
    #[serde(rename = "tasks_entered", skip_serializing_if = "Option::is_none")]
    pub tasks_entered: Option<i32>,
    /// The total number of Tasks that were moved from one queue to another
    #[serde(rename = "tasks_moved", skip_serializing_if = "Option::is_none")]
    pub tasks_moved: Option<i32>,
    /// The absolute URL of the TaskQueue statistics resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The relative wait duration statistics for Tasks accepted while in the TaskQueue
    #[serde(rename = "wait_duration_in_queue_until_accepted", skip_serializing_if = "Option::is_none")]
    pub wait_duration_in_queue_until_accepted: Option<serde_json::Value>,
    /// The wait duration statistics for Tasks accepted while in the TaskQueue
    #[serde(rename = "wait_duration_until_accepted", skip_serializing_if = "Option::is_none")]
    pub wait_duration_until_accepted: Option<serde_json::Value>,
    /// The wait duration statistics for Tasks canceled while in the TaskQueue
    #[serde(rename = "wait_duration_until_canceled", skip_serializing_if = "Option::is_none")]
    pub wait_duration_until_canceled: Option<serde_json::Value>,
    /// The SID of the Workspace that contains the TaskQueue
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceTaskQueueTaskQueueCumulativeStatistics {
    pub fn new() -> TaskrouterV1WorkspaceTaskQueueTaskQueueCumulativeStatistics {
        TaskrouterV1WorkspaceTaskQueueTaskQueueCumulativeStatistics {
            account_sid: None,
            avg_task_acceptance_time: None,
            end_time: None,
            reservations_accepted: None,
            reservations_canceled: None,
            reservations_created: None,
            reservations_rejected: None,
            reservations_rescinded: None,
            reservations_timed_out: None,
            split_by_wait_time: None,
            start_time: None,
            task_queue_sid: None,
            tasks_canceled: None,
            tasks_completed: None,
            tasks_deleted: None,
            tasks_entered: None,
            tasks_moved: None,
            url: None,
            wait_duration_in_queue_until_accepted: None,
            wait_duration_until_accepted: None,
            wait_duration_until_canceled: None,
            workspace_sid: None,
        }
    }
}


