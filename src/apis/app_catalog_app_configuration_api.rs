/*
 * DSH Tenant Resource Management REST API
 *
 * Resource management API for DSH
 *
 * The version of the OpenAPI document: 1.6.6
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppcatalogTenantAppcatalogappAppcatalogappidConfigurationDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppcatalogTenantAppcatalogappAppcatalogappidConfigurationGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppcatalogTenantAppcatalogappAppcatalogappidConfigurationPutError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`appcatalog_tenant_appcatalogapp_appcatalogappid_status_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppcatalogTenantAppcatalogappAppcatalogappidStatusGetError {
    UnknownValue(serde_json::Value),
}


pub async fn appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_delete(configuration: &configuration::Configuration, tenant: &str, appcatalogappid: &str) -> Result<(), Error<AppcatalogTenantAppcatalogappAppcatalogappidConfigurationDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), appcatalogappid=crate::apis::urlencode(appcatalogappid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AppcatalogTenantAppcatalogappAppcatalogappidConfigurationDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_get(configuration: &configuration::Configuration, tenant: &str, appcatalogappid: &str) -> Result<crate::models::AppCatalogAppConfiguration, Error<AppcatalogTenantAppcatalogappAppcatalogappidConfigurationGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), appcatalogappid=crate::apis::urlencode(appcatalogappid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AppcatalogTenantAppcatalogappAppcatalogappidConfigurationGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn appcatalog_tenant_appcatalogapp_appcatalogappid_configuration_put(configuration: &configuration::Configuration, tenant: &str, appcatalogappid: &str, app_catalog_app_configuration: crate::models::AppCatalogAppConfiguration) -> Result<(), Error<AppcatalogTenantAppcatalogappAppcatalogappidConfigurationPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), appcatalogappid=crate::apis::urlencode(appcatalogappid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&app_catalog_app_configuration);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AppcatalogTenantAppcatalogappAppcatalogappidConfigurationPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn appcatalog_tenant_appcatalogapp_appcatalogappid_status_get(configuration: &configuration::Configuration, tenant: &str, appcatalogappid: &str) -> Result<crate::models::AllocationStatus, Error<AppcatalogTenantAppcatalogappAppcatalogappidStatusGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/status", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), appcatalogappid=crate::apis::urlencode(appcatalogappid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AppcatalogTenantAppcatalogappAppcatalogappidStatusGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

