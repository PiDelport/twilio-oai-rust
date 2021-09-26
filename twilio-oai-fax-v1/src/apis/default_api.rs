/*
 * Twilio - Fax
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`create_fax`]
#[derive(Clone, Debug, Default)]
pub struct CreateFaxParams {
    /// The URL of the PDF that contains the fax. See our [security](https://www.twilio.com/docs/usage/security) page for information on how to ensure the request for your media comes from Twilio.
    pub media_url: String,
    /// The phone number to receive the fax in [E.164](https://www.twilio.com/docs/glossary/what-e164) format or the recipient's SIP URI.
    pub to: String,
    /// The number the fax was sent from. Can be the phone number in [E.164](https://www.twilio.com/docs/glossary/what-e164) format or the SIP `from` value. The caller ID displayed to the recipient uses this value. If this is a phone number, it must be a Twilio number or a verified outgoing caller id from your account. If `to` is a SIP address, this can be any alphanumeric string (and also the characters `+`, `_`, `.`, and `-`), which will be used in the `from` header of the SIP request.
    pub from: Option<String>,
    /// The [Fax Quality value](https://www.twilio.com/docs/fax/api/fax-resource#fax-quality-values) that describes the fax quality. Can be: `standard`, `fine`, or `superfine` and defaults to `fine`.
    pub quality: Option<String>,
    /// The password to use with `sip_auth_username` to authenticate faxes sent to a SIP address.
    pub sip_auth_password: Option<String>,
    /// The username to use with the `sip_auth_password` to authenticate faxes sent to a SIP address.
    pub sip_auth_username: Option<String>,
    /// The URL we should call using the `POST` method to send [status information](https://www.twilio.com/docs/fax/api/fax-resource#fax-status-callback) to your application when the status of the fax changes.
    pub status_callback: Option<String>,
    /// Whether to store a copy of the sent media on our servers for later retrieval. Can be: `true` or `false` and the default is `true`.
    pub store_media: Option<bool>,
    /// How long in minutes from when the fax is initiated that we should try to send the fax.
    pub ttl: Option<i32>,
}

/// struct for passing parameters to the method [`delete_fax`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFaxParams {
    /// The Twilio-provided string that uniquely identifies the Fax resource to delete.
    pub sid: String,
}

/// struct for passing parameters to the method [`delete_fax_media`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFaxMediaParams {
    /// The SID of the fax with the FaxMedia resource to delete.
    pub fax_sid: String,
    /// The Twilio-provided string that uniquely identifies the FaxMedia resource to delete.
    pub sid: String,
}

/// struct for passing parameters to the method [`fetch_fax`]
#[derive(Clone, Debug, Default)]
pub struct FetchFaxParams {
    /// The Twilio-provided string that uniquely identifies the Fax resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`fetch_fax_media`]
#[derive(Clone, Debug, Default)]
pub struct FetchFaxMediaParams {
    /// The SID of the fax with the FaxMedia resource to fetch.
    pub fax_sid: String,
    /// The Twilio-provided string that uniquely identifies the FaxMedia resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`list_fax`]
#[derive(Clone, Debug, Default)]
pub struct ListFaxParams {
    /// Retrieve only those faxes sent from this phone number, specified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format.
    pub from: Option<String>,
    /// Retrieve only those faxes sent to this phone number, specified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format.
    pub to: Option<String>,
    /// Retrieve only those faxes with a `date_created` that is before or equal to this value, specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    pub date_created_on_or_before: Option<String>,
    /// Retrieve only those faxes with a `date_created` that is later than this value, specified in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    pub date_created_after: Option<String>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
}

/// struct for passing parameters to the method [`list_fax_media`]
#[derive(Clone, Debug, Default)]
pub struct ListFaxMediaParams {
    /// The SID of the fax with the FaxMedia resources to read.
    pub fax_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
}

/// struct for passing parameters to the method [`update_fax`]
#[derive(Clone, Debug, Default)]
pub struct UpdateFaxParams {
    /// The Twilio-provided string that uniquely identifies the Fax resource to update.
    pub sid: String,
    /// The new [status](https://www.twilio.com/docs/fax/api/fax-resource#fax-status-values) of the resource. Can be only `canceled`. This may fail if transmission has already started.
    pub status: Option<String>,
}

/// struct for typed successes of method [`create_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFaxSuccess {
    Status201(crate::models::FaxV1Fax),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFaxSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_fax_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFaxMediaSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`fetch_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchFaxSuccess {
    Status200(crate::models::FaxV1Fax),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`fetch_fax_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchFaxMediaSuccess {
    Status200(crate::models::FaxV1FaxFaxMedia),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFaxSuccess {
    Status200(crate::models::ListFaxResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_fax_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFaxMediaSuccess {
    Status200(crate::models::ListFaxMediaResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFaxSuccess {
    Status200(crate::models::FaxV1Fax),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFaxError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFaxError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_fax_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFaxMediaError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchFaxError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_fax_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchFaxMediaError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFaxError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_fax_media`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFaxMediaError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_fax`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFaxError {
    UnknownValue(serde_json::Value),
}

/// Create a new fax to send to a phone number or SIP endpoint.
pub async fn create_fax(
    configuration: &configuration::Configuration,
    params: CreateFaxParams,
) -> Result<ResponseContent<CreateFaxSuccess>, Error<CreateFaxError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let media_url = params.media_url;
    let to = params.to;
    let from = params.from;
    let quality = params.quality;
    let sip_auth_password = params.sip_auth_password;
    let sip_auth_username = params.sip_auth_username;
    let status_callback = params.status_callback;
    let store_media = params.store_media;
    let ttl = params.ttl;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Faxes", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = from {
        local_var_form_params.insert("From", local_var_param_value.to_string());
    }
    local_var_form_params.insert("MediaUrl", media_url.to_string());
    if let Some(local_var_param_value) = quality {
        local_var_form_params.insert("Quality", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sip_auth_password {
        local_var_form_params.insert("SipAuthPassword", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sip_auth_username {
        local_var_form_params.insert("SipAuthUsername", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_callback {
        local_var_form_params.insert("StatusCallback", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = store_media {
        local_var_form_params.insert("StoreMedia", local_var_param_value.to_string());
    }
    local_var_form_params.insert("To", to.to_string());
    if let Some(local_var_param_value) = ttl {
        local_var_form_params.insert("Ttl", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<CreateFaxSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CreateFaxError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a specific fax and its associated media.
pub async fn delete_fax(
    configuration: &configuration::Configuration,
    params: DeleteFaxParams,
) -> Result<ResponseContent<DeleteFaxSuccess>, Error<DeleteFaxError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/Faxes/{Sid}",
        local_var_configuration.base_path,
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<DeleteFaxSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteFaxError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a specific fax media instance.
pub async fn delete_fax_media(
    configuration: &configuration::Configuration,
    params: DeleteFaxMediaParams,
) -> Result<ResponseContent<DeleteFaxMediaSuccess>, Error<DeleteFaxMediaError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fax_sid = params.fax_sid;
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/Faxes/{FaxSid}/Media/{Sid}",
        local_var_configuration.base_path,
        FaxSid = crate::apis::urlencode(fax_sid),
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<DeleteFaxMediaSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteFaxMediaError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch a specific fax.
pub async fn fetch_fax(
    configuration: &configuration::Configuration,
    params: FetchFaxParams,
) -> Result<ResponseContent<FetchFaxSuccess>, Error<FetchFaxError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/Faxes/{Sid}",
        local_var_configuration.base_path,
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchFaxSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchFaxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch a specific fax media instance.
pub async fn fetch_fax_media(
    configuration: &configuration::Configuration,
    params: FetchFaxMediaParams,
) -> Result<ResponseContent<FetchFaxMediaSuccess>, Error<FetchFaxMediaError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fax_sid = params.fax_sid;
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/Faxes/{FaxSid}/Media/{Sid}",
        local_var_configuration.base_path,
        FaxSid = crate::apis::urlencode(fax_sid),
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchFaxMediaSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchFaxMediaError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of all faxes.
pub async fn list_fax(
    configuration: &configuration::Configuration,
    params: ListFaxParams,
) -> Result<ResponseContent<ListFaxSuccess>, Error<ListFaxError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let from = params.from;
    let to = params.to;
    let date_created_on_or_before = params.date_created_on_or_before;
    let date_created_after = params.date_created_after;
    let page_size = params.page_size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Faxes", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder =
            local_var_req_builder.query(&[("From", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("To", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = date_created_on_or_before {
        local_var_req_builder =
            local_var_req_builder.query(&[("DateCreatedOnOrBefore", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = date_created_after {
        local_var_req_builder =
            local_var_req_builder.query(&[("DateCreatedAfter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListFaxSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListFaxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of all fax media instances for the specified fax.
pub async fn list_fax_media(
    configuration: &configuration::Configuration,
    params: ListFaxMediaParams,
) -> Result<ResponseContent<ListFaxMediaSuccess>, Error<ListFaxMediaError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fax_sid = params.fax_sid;
    let page_size = params.page_size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/Faxes/{FaxSid}/Media",
        local_var_configuration.base_path,
        FaxSid = crate::apis::urlencode(fax_sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListFaxMediaSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListFaxMediaError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a specific fax.
pub async fn update_fax(
    configuration: &configuration::Configuration,
    params: UpdateFaxParams,
) -> Result<ResponseContent<UpdateFaxSuccess>, Error<UpdateFaxError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sid = params.sid;
    let status = params.status;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/Faxes/{Sid}",
        local_var_configuration.base_path,
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = status {
        local_var_form_params.insert("Status", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateFaxSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateFaxError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
