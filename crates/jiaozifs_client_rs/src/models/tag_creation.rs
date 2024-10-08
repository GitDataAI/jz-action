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
pub struct TagCreation {
    #[serde(rename = "name")]
    pub name: String,
    /// target branch name or commit hex, first try branch and then commit
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl TagCreation {
    pub fn new(name: String, target: String) -> TagCreation {
        TagCreation {
            name,
            target,
            message: None,
        }
    }
}

