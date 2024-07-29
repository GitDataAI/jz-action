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
pub struct Credential {
    #[serde(rename = "access_key_id")]
    pub access_key_id: String,
    #[serde(rename = "secret_access_key")]
    pub secret_access_key: String,
    #[serde(rename = "session_token")]
    pub session_token: String,
}

impl Credential {
    pub fn new(
        access_key_id: String,
        secret_access_key: String,
        session_token: String,
    ) -> Credential {
        Credential {
            access_key_id,
            secret_access_key,
            session_token,
        }
    }
}