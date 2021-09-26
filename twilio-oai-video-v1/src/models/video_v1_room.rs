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
pub struct VideoV1Room {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The duration of the room in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Enable Twilio's Network Traversal TURN service
    #[serde(rename = "enable_turn", skip_serializing_if = "Option::is_none")]
    pub enable_turn: Option<bool>,
    /// The UTC end time of the room in UTC ISO 8601 format
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The maximum number of published tracks allowed in the room at the same time
    #[serde(
        rename = "max_concurrent_published_tracks",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrent_published_tracks: Option<i32>,
    /// The maximum number of concurrent Participants allowed in the room
    #[serde(rename = "max_participants", skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<i32>,
    /// The region for the media server in Group Rooms
    #[serde(rename = "media_region", skip_serializing_if = "Option::is_none")]
    pub media_region: Option<String>,
    /// Whether to start recording when Participants connect
    #[serde(
        rename = "record_participants_on_connect",
        skip_serializing_if = "Option::is_none"
    )]
    pub record_participants_on_connect: Option<bool>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of the room
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The URL to send status information to your application
    #[serde(rename = "status_callback", skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<String>,
    /// The HTTP method we use to call status_callback
    #[serde(
        rename = "status_callback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_method: Option<StatusCallbackMethod>,
    /// The type of room
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// An array of the video codecs that are supported when publishing a track in the room
    #[serde(rename = "video_codecs", skip_serializing_if = "Option::is_none")]
    pub video_codecs: Option<Vec<VideoCodecs>>,
}

impl VideoV1Room {
    pub fn new() -> VideoV1Room {
        VideoV1Room {
            account_sid: None,
            date_created: None,
            date_updated: None,
            duration: None,
            enable_turn: None,
            end_time: None,
            links: None,
            max_concurrent_published_tracks: None,
            max_participants: None,
            media_region: None,
            record_participants_on_connect: None,
            sid: None,
            status: None,
            status_callback: None,
            status_callback_method: None,
            type_: None,
            unique_name: None,
            url: None,
            video_codecs: None,
        }
    }
}

/// The status of the room
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
/// The HTTP method we use to call status_callback
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCallbackMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}
/// The type of room
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "peer-to-peer")]
    PeerToPeer,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "group-small")]
    GroupSmall,
}
/// An array of the video codecs that are supported when publishing a track in the room
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VideoCodecs {
    #[serde(rename = "VP8")]
    VP8,
    #[serde(rename = "H264")]
    H264,
}
