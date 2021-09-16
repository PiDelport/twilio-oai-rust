/*
 * Twilio - Insights
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InsightsV1CallSummary {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(rename = "call_sid", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<String>,
    #[serde(rename = "call_state", skip_serializing_if = "Option::is_none")]
    pub call_state: Option<CallState>,
    #[serde(rename = "call_type", skip_serializing_if = "Option::is_none")]
    pub call_type: Option<CallType>,
    #[serde(rename = "carrier_edge", skip_serializing_if = "Option::is_none")]
    pub carrier_edge: Option<serde_json::Value>,
    #[serde(rename = "client_edge", skip_serializing_if = "Option::is_none")]
    pub client_edge: Option<serde_json::Value>,
    #[serde(rename = "connect_duration", skip_serializing_if = "Option::is_none")]
    pub connect_duration: Option<i32>,
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<serde_json::Value>,
    #[serde(rename = "processing_state", skip_serializing_if = "Option::is_none")]
    pub processing_state: Option<ProcessingState>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "sdk_edge", skip_serializing_if = "Option::is_none")]
    pub sdk_edge: Option<serde_json::Value>,
    #[serde(rename = "sip_edge", skip_serializing_if = "Option::is_none")]
    pub sip_edge: Option<serde_json::Value>,
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<serde_json::Value>,
    #[serde(rename = "trust", skip_serializing_if = "Option::is_none")]
    pub trust: Option<serde_json::Value>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl InsightsV1CallSummary {
    pub fn new() -> InsightsV1CallSummary {
        InsightsV1CallSummary {
            account_sid: None,
            attributes: None,
            call_sid: None,
            call_state: None,
            call_type: None,
            carrier_edge: None,
            client_edge: None,
            connect_duration: None,
            created_time: None,
            duration: None,
            end_time: None,
            from: None,
            processing_state: None,
            properties: None,
            sdk_edge: None,
            sip_edge: None,
            start_time: None,
            tags: None,
            to: None,
            trust: None,
            url: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallState {
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "noanswer")]
    Noanswer,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "answered")]
    Answered,
    #[serde(rename = "undialed")]
    Undialed,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallType {
    #[serde(rename = "carrier")]
    Carrier,
    #[serde(rename = "sip")]
    Sip,
    #[serde(rename = "trunking")]
    Trunking,
    #[serde(rename = "client")]
    Client,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProcessingState {
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "partial")]
    Partial,
}

