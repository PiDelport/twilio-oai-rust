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
pub struct TaskrouterV1WorkspaceWorkflowWorkflowStatistics {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// An object that contains the cumulative statistics for the Workflow
    #[serde(rename = "cumulative", skip_serializing_if = "Option::is_none")]
    pub cumulative: Option<serde_json::Value>,
    /// An object that contains the real-time statistics for the Workflow
    #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
    pub realtime: Option<serde_json::Value>,
    /// The absolute URL of the Workflow statistics resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Returns the list of Tasks that are being controlled by the Workflow with the specified SID value
    #[serde(rename = "workflow_sid", skip_serializing_if = "Option::is_none")]
    pub workflow_sid: Option<String>,
    /// The SID of the Workspace that contains the Workflow
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceWorkflowWorkflowStatistics {
    pub fn new() -> TaskrouterV1WorkspaceWorkflowWorkflowStatistics {
        TaskrouterV1WorkspaceWorkflowWorkflowStatistics {
            account_sid: None,
            cumulative: None,
            realtime: None,
            url: None,
            workflow_sid: None,
            workspace_sid: None,
        }
    }
}
