/*
 * Twilio - Video
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListRecordingResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCompositionHookResponseMeta>>,
    #[serde(rename = "recordings", skip_serializing_if = "Option::is_none")]
    pub recordings: Option<Vec<crate::models::VideoV1Recording>>,
}

impl ListRecordingResponse {
    pub fn new() -> ListRecordingResponse {
        ListRecordingResponse {
            meta: None,
            recordings: None,
        }
    }
}
