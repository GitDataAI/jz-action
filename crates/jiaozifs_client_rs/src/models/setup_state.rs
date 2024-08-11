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
pub struct SetupState {
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// true if the comm prefs are missing.
    #[serde(rename = "comm_prefs_missing", skip_serializing_if = "Option::is_none")]
    pub comm_prefs_missing: Option<bool>,
    #[serde(rename = "login_config", skip_serializing_if = "Option::is_none")]
    pub login_config: Option<Box<models::LoginConfig>>,
}

impl SetupState {
    pub fn new() -> SetupState {
        SetupState {
            state: None,
            comm_prefs_missing: None,
            login_config: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "not_initialized")]
    NotInitialized,
}

impl Default for State {
    fn default() -> State {
        Self::Initialized
    }
}

