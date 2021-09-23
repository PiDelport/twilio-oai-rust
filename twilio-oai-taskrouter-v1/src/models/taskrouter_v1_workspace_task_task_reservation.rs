/*
 * Twilio - Taskrouter
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskrouterV1WorkspaceTaskTaskReservation {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The current status of the reservation
    #[serde(rename = "reservation_status", skip_serializing_if = "Option::is_none")]
    pub reservation_status: Option<ReservationStatus>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SID of the reserved Task resource
    #[serde(rename = "task_sid", skip_serializing_if = "Option::is_none")]
    pub task_sid: Option<String>,
    /// The absolute URL of the TaskReservation reservation
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The friendly_name of the Worker that is reserved
    #[serde(rename = "worker_name", skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
    /// The SID of the reserved Worker resource
    #[serde(rename = "worker_sid", skip_serializing_if = "Option::is_none")]
    pub worker_sid: Option<String>,
    /// The SID of the Workspace that this task is contained within.
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceTaskTaskReservation {
    pub fn new() -> TaskrouterV1WorkspaceTaskTaskReservation {
        TaskrouterV1WorkspaceTaskTaskReservation {
            account_sid: None,
            date_created: None,
            date_updated: None,
            links: None,
            reservation_status: None,
            sid: None,
            task_sid: None,
            url: None,
            worker_name: None,
            worker_sid: None,
            workspace_sid: None,
        }
    }
}

/// The current status of the reservation
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReservationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "rescinded")]
    Rescinded,
    #[serde(rename = "wrapping")]
    Wrapping,
    #[serde(rename = "completed")]
    Completed,
}

