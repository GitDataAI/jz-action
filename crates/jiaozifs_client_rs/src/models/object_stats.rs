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
pub struct ObjectStats {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "path_mode", skip_serializing_if = "Option::is_none")]
    pub path_mode: Option<i32>,
    #[serde(rename = "checksum")]
    pub checksum: String,
    #[serde(rename = "size_bytes", skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// Unix Epoch in seconds
    #[serde(rename = "mtime")]
    pub mtime: i64,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Object media type
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl ObjectStats {
    pub fn new(path: String, checksum: String, mtime: i64) -> ObjectStats {
        ObjectStats {
            path,
            path_mode: None,
            checksum,
            size_bytes: None,
            mtime,
            metadata: None,
            content_type: None,
        }
    }
}

