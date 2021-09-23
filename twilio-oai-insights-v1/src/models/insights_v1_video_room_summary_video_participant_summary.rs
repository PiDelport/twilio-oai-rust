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
pub struct InsightsV1VideoRoomSummaryVideoParticipantSummary {
    /// Account SID associated with the room.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Codecs detected from the participant.
    #[serde(rename = "codecs", skip_serializing_if = "Option::is_none")]
    pub codecs: Option<Vec<Codecs>>,
    /// Amount of time in seconds the participant was in the room.
    #[serde(rename = "duration_sec", skip_serializing_if = "Option::is_none")]
    pub duration_sec: Option<i32>,
    /// Name of the edge location the participant connected to.
    #[serde(rename = "edge_location", skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<EdgeLocation>,
    /// Reason the participant left the room.
    #[serde(rename = "end_reason", skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<String>,
    /// Errors encountered by the participant.
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// Twilio error code dictionary link.
    #[serde(rename = "error_code_url", skip_serializing_if = "Option::is_none")]
    pub error_code_url: Option<String>,
    /// When the participant joined the room.
    #[serde(rename = "join_time", skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    /// When the participant left the room
    #[serde(rename = "leave_time", skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<String>,
    /// Twilio media region the participant connected to.
    #[serde(rename = "media_region", skip_serializing_if = "Option::is_none")]
    pub media_region: Option<MediaRegion>,
    /// The application-defined string that uniquely identifies the participant within a Room.
    #[serde(rename = "participant_identity", skip_serializing_if = "Option::is_none")]
    pub participant_identity: Option<String>,
    /// Unique identifier for the participant.
    #[serde(rename = "participant_sid", skip_serializing_if = "Option::is_none")]
    pub participant_sid: Option<String>,
    /// Object containing information about the participant's data from the room.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Object containing information about the SDK name and version.
    #[serde(rename = "publisher_info", skip_serializing_if = "Option::is_none")]
    pub publisher_info: Option<serde_json::Value>,
    /// Unique identifier for the room.
    #[serde(rename = "room_sid", skip_serializing_if = "Option::is_none")]
    pub room_sid: Option<String>,
    /// Status of the room.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// URL of the participant resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl InsightsV1VideoRoomSummaryVideoParticipantSummary {
    pub fn new() -> InsightsV1VideoRoomSummaryVideoParticipantSummary {
        InsightsV1VideoRoomSummaryVideoParticipantSummary {
            account_sid: None,
            codecs: None,
            duration_sec: None,
            edge_location: None,
            end_reason: None,
            error_code: None,
            error_code_url: None,
            join_time: None,
            leave_time: None,
            media_region: None,
            participant_identity: None,
            participant_sid: None,
            properties: None,
            publisher_info: None,
            room_sid: None,
            status: None,
            url: None,
        }
    }
}

/// Codecs detected from the participant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Codecs {
    #[serde(rename = "VP8")]
    VP8,
    #[serde(rename = "H264")]
    H264,
    #[serde(rename = "VP9")]
    VP9,
}
/// Name of the edge location the participant connected to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EdgeLocation {
    #[serde(rename = "ashburn")]
    Ashburn,
    #[serde(rename = "dublin")]
    Dublin,
    #[serde(rename = "frankfurt")]
    Frankfurt,
    #[serde(rename = "singapore")]
    Singapore,
    #[serde(rename = "sydney")]
    Sydney,
    #[serde(rename = "sao_paulo")]
    SaoPaulo,
    #[serde(rename = "roaming")]
    Roaming,
    #[serde(rename = "umatilla")]
    Umatilla,
    #[serde(rename = "tokyo")]
    Tokyo,
}
/// Twilio media region the participant connected to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaRegion {
    #[serde(rename = "us1")]
    Us1,
    #[serde(rename = "us2")]
    Us2,
    #[serde(rename = "au1")]
    Au1,
    #[serde(rename = "br1")]
    Br1,
    #[serde(rename = "ie1")]
    Ie1,
    #[serde(rename = "jp1")]
    Jp1,
    #[serde(rename = "sg1")]
    Sg1,
    #[serde(rename = "in1")]
    In1,
    #[serde(rename = "de1")]
    De1,
    #[serde(rename = "gll")]
    Gll,
}
/// Status of the room.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}

