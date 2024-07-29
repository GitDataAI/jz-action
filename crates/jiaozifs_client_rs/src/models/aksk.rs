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
pub struct Aksk {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "access_key")]
    pub access_key: String,
    #[serde(rename = "secret_key")]
    pub secret_key: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl Aksk {
    pub fn new(
        id: uuid::Uuid,
        access_key: String,
        secret_key: String,
        created_at: i64,
        updated_at: i64,
    ) -> Aksk {
        Aksk {
            id,
            access_key,
            secret_key,
            description: None,
            created_at,
            updated_at,
        }
    }
}