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
pub struct Blob {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "repository_id")]
    pub repository_id: uuid::Uuid,
    #[serde(rename = "check_sum")]
    pub check_sum: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "properties")]
    pub properties: std::collections::HashMap<String, String>,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl Blob {
    pub fn new(
        hash: String,
        repository_id: uuid::Uuid,
        check_sum: String,
        r#type: i32,
        size: i64,
        properties: std::collections::HashMap<String, String>,
        created_at: i64,
        updated_at: i64,
    ) -> Blob {
        Blob {
            hash,
            repository_id,
            check_sum,
            r#type,
            size,
            properties,
            created_at,
            updated_at,
        }
    }
}