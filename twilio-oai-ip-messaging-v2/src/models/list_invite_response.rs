/*
 * Twilio - Ip_messaging
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListInviteResponse {
    #[serde(rename = "invites", skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<crate::models::IpMessagingV2ServiceChannelInvite>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCredentialResponseMeta>>,
}

impl ListInviteResponse {
    pub fn new() -> ListInviteResponse {
        ListInviteResponse {
            invites: None,
            meta: None,
        }
    }
}


