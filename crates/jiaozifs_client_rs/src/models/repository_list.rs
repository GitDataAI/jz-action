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
pub struct RepositoryList {
    #[serde(rename = "pagination")]
    pub pagination: Box<models::Pagination>,
    #[serde(rename = "results")]
    pub results: Vec<models::Repository>,
}

impl RepositoryList {
    pub fn new(pagination: models::Pagination, results: Vec<models::Repository>) -> RepositoryList {
        RepositoryList {
            pagination: Box::new(pagination),
            results,
        }
    }
}

