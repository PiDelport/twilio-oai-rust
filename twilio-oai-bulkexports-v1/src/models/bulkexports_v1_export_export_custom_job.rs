/*
 * Twilio - Bulkexports
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkexportsV1ExportExportCustomJob {
    /// The details of a job state which is an object that contains a `status` string, a day count integer, and list of days in the job
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// The optional email to send the completion notification to
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The end day for the custom export specified as a string in the format of yyyy-MM-dd. This will be the last day exported. For instance, to export a single day, choose the same day for start and end day. To export the first 4 days of July, you would set the start date to 2020-07-01 and the end date to 2020-07-04. The end date must be the UTC day before yesterday.
    #[serde(rename = "end_day", skip_serializing_if = "Option::is_none")]
    pub end_day: Option<String>,
    /// this is the time estimated until your job is complete. This is calculated each time you request the job list. The time is calculated based on the current rate of job completion (which may vary) and your job queue position
    #[serde(rename = "estimated_completion_time", skip_serializing_if = "Option::is_none")]
    pub estimated_completion_time: Option<String>,
    /// The friendly name specified when creating the job
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// This is the job position from the 1st in line. Your queue position will never increase. As jobs ahead of yours in the queue are processed, the queue position number will decrease
    #[serde(rename = "job_queue_position", skip_serializing_if = "Option::is_none")]
    pub job_queue_position: Option<String>,
    /// The unique job_sid returned when the custom export was created. This can be used to look up the status of the job.
    #[serde(rename = "job_sid", skip_serializing_if = "Option::is_none")]
    pub job_sid: Option<String>,
    /// The type of communication – Messages, Calls, Conferences, and Participants
    #[serde(rename = "resource_type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The start day for the custom export specified as a string in the format of yyyy-MM-dd
    #[serde(rename = "start_day", skip_serializing_if = "Option::is_none")]
    pub start_day: Option<String>,
    /// This is the method used to call the webhook
    #[serde(rename = "webhook_method", skip_serializing_if = "Option::is_none")]
    pub webhook_method: Option<String>,
    /// The optional webhook url called on completion
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

impl BulkexportsV1ExportExportCustomJob {
    pub fn new() -> BulkexportsV1ExportExportCustomJob {
        BulkexportsV1ExportExportCustomJob {
            details: None,
            email: None,
            end_day: None,
            estimated_completion_time: None,
            friendly_name: None,
            job_queue_position: None,
            job_sid: None,
            resource_type: None,
            start_day: None,
            webhook_method: None,
            webhook_url: None,
        }
    }
}

