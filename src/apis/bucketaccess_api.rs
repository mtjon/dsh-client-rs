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


/// struct for typed errors of method [`allocation_tenant_bucket_id_bucketaccess_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantBucketIdBucketaccessGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_bucket_id_bucketaccess_name_actual_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantBucketIdBucketaccessNameActualGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_bucket_id_bucketaccess_name_configuration_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantBucketIdBucketaccessNameConfigurationDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_bucket_id_bucketaccess_name_configuration_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantBucketIdBucketaccessNameConfigurationGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_bucket_id_bucketaccess_name_configuration_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantBucketIdBucketaccessNameConfigurationPutError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_bucket_id_bucketaccess_name_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantBucketIdBucketaccessNameGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_bucket_id_bucketaccess_name_status_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantBucketIdBucketaccessNameStatusGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`allocation_tenant_bucketaccess_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllocationTenantBucketaccessGetError {
    UnknownValue(serde_json::Value),
}


pub async fn allocation_tenant_bucket_id_bucketaccess_get(configuration: &configuration::Configuration, tenant: &str, id: &str) -> Result<Vec<String>, Error<AllocationTenantBucketIdBucketaccessGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/allocation/{tenant}/bucket/{id}/bucketaccess", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), id=crate::apis::urlencode(id));
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
        let local_var_entity: Option<AllocationTenantBucketIdBucketaccessGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_bucket_id_bucketaccess_name_actual_get(configuration: &configuration::Configuration, tenant: &str, id: &str, name: &str) -> Result<crate::models::BucketAccess, Error<AllocationTenantBucketIdBucketaccessNameActualGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/actual", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), id=crate::apis::urlencode(id), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<AllocationTenantBucketIdBucketaccessNameActualGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_bucket_id_bucketaccess_name_configuration_delete(configuration: &configuration::Configuration, tenant: &str, id: &str, name: &str) -> Result<(), Error<AllocationTenantBucketIdBucketaccessNameConfigurationDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), id=crate::apis::urlencode(id), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<AllocationTenantBucketIdBucketaccessNameConfigurationDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_bucket_id_bucketaccess_name_configuration_get(configuration: &configuration::Configuration, tenant: &str, id: &str, name: &str) -> Result<crate::models::BucketAccessConfiguration, Error<AllocationTenantBucketIdBucketaccessNameConfigurationGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), id=crate::apis::urlencode(id), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<AllocationTenantBucketIdBucketaccessNameConfigurationGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_bucket_id_bucketaccess_name_configuration_put(configuration: &configuration::Configuration, tenant: &str, id: &str, name: &str, bucket_access_configuration: crate::models::BucketAccessConfiguration) -> Result<(), Error<AllocationTenantBucketIdBucketaccessNameConfigurationPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), id=crate::apis::urlencode(id), name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&bucket_access_configuration);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AllocationTenantBucketIdBucketaccessNameConfigurationPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_bucket_id_bucketaccess_name_get(configuration: &configuration::Configuration, tenant: &str, id: &str, name: &str) -> Result<crate::models::BucketAccessStatus, Error<AllocationTenantBucketIdBucketaccessNameGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/allocation/{tenant}/bucket/{id}/bucketaccess/{name}", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), id=crate::apis::urlencode(id), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<AllocationTenantBucketIdBucketaccessNameGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_bucket_id_bucketaccess_name_status_get(configuration: &configuration::Configuration, tenant: &str, id: &str, name: &str) -> Result<crate::models::AllocationStatus, Error<AllocationTenantBucketIdBucketaccessNameStatusGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/status", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant), id=crate::apis::urlencode(id), name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<AllocationTenantBucketIdBucketaccessNameStatusGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn allocation_tenant_bucketaccess_get(configuration: &configuration::Configuration, tenant: &str) -> Result<Vec<String>, Error<AllocationTenantBucketaccessGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/allocation/{tenant}/bucketaccess", local_var_configuration.base_path, tenant=crate::apis::urlencode(tenant));
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
        let local_var_entity: Option<AllocationTenantBucketaccessGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

