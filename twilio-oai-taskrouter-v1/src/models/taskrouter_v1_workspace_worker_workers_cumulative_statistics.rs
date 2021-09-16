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
pub struct TaskrouterV1WorkspaceWorkerWorkersCumulativeStatistics {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The minimum, average, maximum, and total time that Workers spent in each Activity
    #[serde(rename = "activity_durations", skip_serializing_if = "Option::is_none")]
    pub activity_durations: Option<Vec<serde_json::Value>>,
    /// The end of the interval during which these statistics were calculated
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The total number of Reservations that were accepted
    #[serde(rename = "reservations_accepted", skip_serializing_if = "Option::is_none")]
    pub reservations_accepted: Option<i32>,
    /// The total number of Reservations that were canceled
    #[serde(rename = "reservations_canceled", skip_serializing_if = "Option::is_none")]
    pub reservations_canceled: Option<i32>,
    /// The total number of Reservations that were created
    #[serde(rename = "reservations_created", skip_serializing_if = "Option::is_none")]
    pub reservations_created: Option<i32>,
    /// The total number of Reservations that were rejected
    #[serde(rename = "reservations_rejected", skip_serializing_if = "Option::is_none")]
    pub reservations_rejected: Option<i32>,
    /// The total number of Reservations that were rescinded
    #[serde(rename = "reservations_rescinded", skip_serializing_if = "Option::is_none")]
    pub reservations_rescinded: Option<i32>,
    /// The total number of Reservations that were timed out
    #[serde(rename = "reservations_timed_out", skip_serializing_if = "Option::is_none")]
    pub reservations_timed_out: Option<i32>,
    /// The beginning of the interval during which these statistics were calculated
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The absolute URL of the Workers statistics resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the Workspace that contains the Workers
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceWorkerWorkersCumulativeStatistics {
    pub fn new() -> TaskrouterV1WorkspaceWorkerWorkersCumulativeStatistics {
        TaskrouterV1WorkspaceWorkerWorkersCumulativeStatistics {
            account_sid: None,
            activity_durations: None,
            end_time: None,
            reservations_accepted: None,
            reservations_canceled: None,
            reservations_created: None,
            reservations_rejected: None,
            reservations_rescinded: None,
            reservations_timed_out: None,
            start_time: None,
            url: None,
            workspace_sid: None,
        }
    }
}

