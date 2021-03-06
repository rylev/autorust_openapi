use crate::{is_false, DataType, MsParameterGrouping, ReferenceOr, Schema};
use serde::{Deserialize, Serialize};

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// values depend on parameter type
    /// may be `header`, `query`, 'path`, `formData`
    #[serde(rename = "in")]
    pub in_: String,
    /// A brief description of the parameter. This could contain examples
    /// of use.  GitHub Flavored Markdown is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOr<Schema>>,

    // fields also in Schema Object
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<DataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ReferenceOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub exclusive_maximum: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub exclusive_minimum: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub unique_items: bool,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,

    // fields not in Schema Object
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_empty_value: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_format: Option<String>,

    /// provides a mechanism to specify that the global parameter is actually a parameter on the operation and not a client property
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-parameter-location
    #[serde(rename = "x-ms-parameter-location", skip_serializing_if = "Option::is_none")]
    pub x_ms_parameter_location: Option<String>,

    /// skips URL encoding for path and query parameters
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-skip-url-encoding
    #[serde(rename = "x-ms-skip-url-encoding", default, skip_serializing_if = "is_false")]
    pub x_ms_skip_url_encoding: bool,

    /// groups method parameters in generated clients
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-parameter-grouping
    #[serde(rename = "x-ms-parameter-grouping", skip_serializing_if = "Option::is_none")]
    pub x_ms_parameter_grouping: Option<MsParameterGrouping>,
}
