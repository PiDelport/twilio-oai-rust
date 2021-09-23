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
pub struct ListRoomParticipantResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCompositionHookResponseMeta>>,
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<crate::models::VideoV1RoomRoomParticipant>>,
}

impl ListRoomParticipantResponse {
    pub fn new() -> ListRoomParticipantResponse {
        ListRoomParticipantResponse {
            meta: None,
            participants: None,
        }
    }
}


