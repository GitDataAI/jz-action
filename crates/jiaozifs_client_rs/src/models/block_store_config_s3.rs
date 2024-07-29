/*
 * jiaozifs API
 *
 * jiaozifs HTTP API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockStoreConfigS3 {
    #[serde(rename = "credentials")]
    pub credentials: Box<models::Credential>,
    #[serde(rename = "web_identity", skip_serializing_if = "Option::is_none")]
    pub web_identity: Option<Box<models::WebIdentity>>,
    #[serde(rename = "discover_bucket_region")]
    pub discover_bucket_region: bool,
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "force_path_style", skip_serializing_if = "Option::is_none")]
    pub force_path_style: Option<bool>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl BlockStoreConfigS3 {
    pub fn new(
        credentials: models::Credential,
        discover_bucket_region: bool,
        endpoint: String,
    ) -> BlockStoreConfigS3 {
        BlockStoreConfigS3 {
            credentials: Box::new(credentials),
            web_identity: None,
            discover_bucket_region,
            endpoint,
            force_path_style: None,
            region: None,
        }
    }
}