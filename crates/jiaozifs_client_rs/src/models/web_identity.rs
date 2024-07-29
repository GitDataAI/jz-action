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
pub struct WebIdentity {
    #[serde(rename = "session_duration")]
    pub session_duration: i64,
    #[serde(rename = "session_expiry_window")]
    pub session_expiry_window: i64,
}

impl WebIdentity {
    pub fn new(session_duration: i64, session_expiry_window: i64) -> WebIdentity {
        WebIdentity {
            session_duration,
            session_expiry_window,
        }
    }
}