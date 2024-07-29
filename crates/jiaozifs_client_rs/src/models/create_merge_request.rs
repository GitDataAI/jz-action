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
pub struct CreateMergeRequest {
    #[serde(rename = "target_branch_name")]
    pub target_branch_name: String,
    #[serde(rename = "source_branch_name")]
    pub source_branch_name: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateMergeRequest {
    pub fn new(
        target_branch_name: String,
        source_branch_name: String,
        title: String,
    ) -> CreateMergeRequest {
        CreateMergeRequest {
            target_branch_name,
            source_branch_name,
            title,
            description: None,
        }
    }
}