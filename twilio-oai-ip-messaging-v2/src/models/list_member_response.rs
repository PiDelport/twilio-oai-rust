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
pub struct ListMemberResponse {
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::IpMessagingV2ServiceChannelMember>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCredentialResponseMeta>>,
}

impl ListMemberResponse {
    pub fn new() -> ListMemberResponse {
        ListMemberResponse {
            members: None,
            meta: None,
        }
    }
}


