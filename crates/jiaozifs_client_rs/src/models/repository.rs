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
pub struct Repository {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner_id")]
    pub owner_id: uuid::Uuid,
    #[serde(rename = "visible")]
    pub visible: bool,
    #[serde(rename = "head")]
    pub head: String,
    #[serde(rename = "use_public_storage")]
    pub use_public_storage: bool,
    #[serde(rename = "storage_adapter_params", skip_serializing_if = "Option::is_none")]
    pub storage_adapter_params: Option<String>,
    #[serde(rename = "storage_namespace", skip_serializing_if = "Option::is_none")]
    pub storage_namespace: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "creator_id")]
    pub creator_id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl Repository {
    pub fn new(id: uuid::Uuid, name: String, owner_id: uuid::Uuid, visible: bool, head: String, use_public_storage: bool, creator_id: uuid::Uuid, created_at: i64, updated_at: i64) -> Repository {
        Repository {
            id,
            name,
            owner_id,
            visible,
            head,
            use_public_storage,
            storage_adapter_params: None,
            storage_namespace: None,
            description: None,
            creator_id,
            created_at,
            updated_at,
        }
    }
}

