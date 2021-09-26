/*
 * Twilio - Trusthub
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrusthubV1CustomerProfileCustomerProfileChannelEndpointAssignment {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The sid of an channel endpoint
    #[serde(
        rename = "channel_endpoint_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_endpoint_sid: Option<String>,
    /// The type of channel endpoint
    #[serde(
        rename = "channel_endpoint_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_endpoint_type: Option<String>,
    /// The unique string that identifies the CustomerProfile resource.
    #[serde(
        rename = "customer_profile_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_profile_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Identity resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl TrusthubV1CustomerProfileCustomerProfileChannelEndpointAssignment {
    pub fn new() -> TrusthubV1CustomerProfileCustomerProfileChannelEndpointAssignment {
        TrusthubV1CustomerProfileCustomerProfileChannelEndpointAssignment {
            account_sid: None,
            channel_endpoint_sid: None,
            channel_endpoint_type: None,
            customer_profile_sid: None,
            date_created: None,
            sid: None,
            url: None,
        }
    }
}
