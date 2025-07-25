/*
 * Lance Namespace Specification
 *
 * This OpenAPI specification is a part of the Lance namespace specification. It contains 2 parts:  The `components/schemas`, `components/responses`, `components/examples`, `tags` sections define the request and response shape for each operation in a Lance Namespace across all implementations. See https://lancedb.github.io/lance-namespace/spec/operations for more details.  The `servers`, `security`, `paths`, `components/parameters` sections are for the  Lance REST Namespace implementation, which defines a complete REST server that can work with Lance datasets. See https://lancedb.github.io/lance-namespace/spec/impls/rest for more details. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ErrorResponse : JSON error response model based on [RFC-7807](https://datatracker.ietf.org/doc/html/rfc7807)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// a URI identifier that categorizes the error
    #[serde(rename = "type")]
    pub r#type: String,
    /// a brief, human-readable message about the error
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// HTTP response code, (if present) it must match the actual HTTP code returned by the service
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// a human-readable explanation of the error
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// a URI that identifies the specific occurrence of the error
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl ErrorResponse {
    /// JSON error response model based on [RFC-7807](https://datatracker.ietf.org/doc/html/rfc7807)
    pub fn new(r#type: String) -> ErrorResponse {
        ErrorResponse {
            r#type,
            title: None,
            status: None,
            detail: None,
            instance: None,
        }
    }
}

