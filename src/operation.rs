use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#operation-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub produces: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "operationId", skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// Required. The list of possible responses as they are returned from executing this operation.
    pub responses: IndexMap<String, Response>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<SecurityRequirement>,

    #[serde(rename = "x-ms-pageable", skip_serializing_if = "Option::is_none")]
    pub x_ms_pageable: Option<MsPageable>,
    #[serde(rename = "x-ms-examples", default, skip_serializing_if = "IndexMap::is_empty")]
    pub x_ms_examples: MsExamples,
    #[serde(rename = "x-ms-long-running-operation", default, skip_serializing_if = "is_false")]
    pub x_ms_long_running_operation: bool,
    #[serde(rename = "x-ms-long-running-operation-options", skip_serializing_if = "Option::is_none")]
    pub x_ms_long_running_operation_options: Option<MsLongRunningOperationOptions>,

    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
}
