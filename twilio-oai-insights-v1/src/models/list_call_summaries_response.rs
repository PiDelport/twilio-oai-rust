/*
 * Twilio - Insights
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListCallSummariesResponse {
    #[serde(rename = "call_summaries", skip_serializing_if = "Option::is_none")]
    pub call_summaries: Option<Vec<crate::models::InsightsV1CallSummaries>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListVideoRoomSummaryResponseMeta>>,
}

impl ListCallSummariesResponse {
    pub fn new() -> ListCallSummariesResponse {
        ListCallSummariesResponse {
            call_summaries: None,
            meta: None,
        }
    }
}


