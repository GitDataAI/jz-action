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
pub struct BlockStoreConfigGs {
    #[serde(rename = "credentials_json")]
    pub credentials_json: String,
    #[serde(rename = "s3_endpoint")]
    pub s3_endpoint: String,
}

impl BlockStoreConfigGs {
    pub fn new(credentials_json: String, s3_endpoint: String) -> BlockStoreConfigGs {
        BlockStoreConfigGs {
            credentials_json,
            s3_endpoint,
        }
    }
}

