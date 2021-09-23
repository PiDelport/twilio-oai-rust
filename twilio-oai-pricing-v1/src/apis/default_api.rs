/*
 * Twilio - Pricing
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `fetch_messaging_country`
#[derive(Clone, Debug, Default)]
pub struct FetchMessagingCountryParams {
    /// The [ISO country code](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) of the pricing information to fetch.
    pub iso_country: String
}

/// struct for passing parameters to the method `fetch_phone_number_country`
#[derive(Clone, Debug, Default)]
pub struct FetchPhoneNumberCountryParams {
    /// The [ISO country code](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) of the pricing information to fetch.
    pub iso_country: String
}

/// struct for passing parameters to the method `fetch_voice_country`
#[derive(Clone, Debug, Default)]
pub struct FetchVoiceCountryParams {
    /// The [ISO country code](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) of the pricing information to fetch.
    pub iso_country: String
}

/// struct for passing parameters to the method `fetch_voice_number`
#[derive(Clone, Debug, Default)]
pub struct FetchVoiceNumberParams {
    /// The phone number to fetch.
    pub number: String
}

/// struct for passing parameters to the method `list_messaging_country`
#[derive(Clone, Debug, Default)]
pub struct ListMessagingCountryParams {
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>
}

/// struct for passing parameters to the method `list_phone_number_country`
#[derive(Clone, Debug, Default)]
pub struct ListPhoneNumberCountryParams {
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>
}

/// struct for passing parameters to the method `list_voice_country`
#[derive(Clone, Debug, Default)]
pub struct ListVoiceCountryParams {
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>
}


/// struct for typed successes of method `fetch_messaging_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchMessagingCountrySuccess {
    Status200(crate::models::PricingV1MessagingMessagingCountryInstance),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `fetch_phone_number_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchPhoneNumberCountrySuccess {
    Status200(crate::models::PricingV1PhoneNumberPhoneNumberCountryInstance),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `fetch_voice_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchVoiceCountrySuccess {
    Status200(crate::models::PricingV1VoiceVoiceCountryInstance),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `fetch_voice_number`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchVoiceNumberSuccess {
    Status200(crate::models::PricingV1VoiceVoiceNumber),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `list_messaging_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMessagingCountrySuccess {
    Status200(crate::models::ListMessagingCountryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `list_phone_number_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPhoneNumberCountrySuccess {
    Status200(crate::models::ListPhoneNumberCountryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `list_voice_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVoiceCountrySuccess {
    Status200(crate::models::ListVoiceCountryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `fetch_messaging_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchMessagingCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `fetch_phone_number_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchPhoneNumberCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `fetch_voice_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchVoiceCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `fetch_voice_number`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchVoiceNumberError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_messaging_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMessagingCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_phone_number_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPhoneNumberCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_voice_country`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVoiceCountryError {
    UnknownValue(serde_json::Value),
}


pub async fn fetch_messaging_country(configuration: &configuration::Configuration, params: FetchMessagingCountryParams) -> Result<ResponseContent<FetchMessagingCountrySuccess>, Error<FetchMessagingCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let iso_country = params.iso_country;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Messaging/Countries/{IsoCountry}", local_var_configuration.base_path, IsoCountry=crate::apis::urlencode(iso_country));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchMessagingCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchMessagingCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn fetch_phone_number_country(configuration: &configuration::Configuration, params: FetchPhoneNumberCountryParams) -> Result<ResponseContent<FetchPhoneNumberCountrySuccess>, Error<FetchPhoneNumberCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let iso_country = params.iso_country;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/PhoneNumbers/Countries/{IsoCountry}", local_var_configuration.base_path, IsoCountry=crate::apis::urlencode(iso_country));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchPhoneNumberCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchPhoneNumberCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn fetch_voice_country(configuration: &configuration::Configuration, params: FetchVoiceCountryParams) -> Result<ResponseContent<FetchVoiceCountrySuccess>, Error<FetchVoiceCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let iso_country = params.iso_country;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Voice/Countries/{IsoCountry}", local_var_configuration.base_path, IsoCountry=crate::apis::urlencode(iso_country));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchVoiceCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchVoiceCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn fetch_voice_number(configuration: &configuration::Configuration, params: FetchVoiceNumberParams) -> Result<ResponseContent<FetchVoiceNumberSuccess>, Error<FetchVoiceNumberError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let number = params.number;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Voice/Numbers/{Number}", local_var_configuration.base_path, Number=crate::apis::urlencode(number));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchVoiceNumberSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchVoiceNumberError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_messaging_country(configuration: &configuration::Configuration, params: ListMessagingCountryParams) -> Result<ResponseContent<ListMessagingCountrySuccess>, Error<ListMessagingCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Messaging/Countries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListMessagingCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListMessagingCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_phone_number_country(configuration: &configuration::Configuration, params: ListPhoneNumberCountryParams) -> Result<ResponseContent<ListPhoneNumberCountrySuccess>, Error<ListPhoneNumberCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/PhoneNumbers/Countries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListPhoneNumberCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListPhoneNumberCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_voice_country(configuration: &configuration::Configuration, params: ListVoiceCountryParams) -> Result<ResponseContent<ListVoiceCountrySuccess>, Error<ListVoiceCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Voice/Countries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListVoiceCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListVoiceCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

