use serde::{Deserialize, Serialize};

/// Bundles all properties of SimpleArrayInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleArrayInterfaceData {
    #[serde(rename = "propBool")]
    pub prop_bool: Vec<bool>,
    #[serde(rename = "propInt")]
    pub prop_int: Vec<i32>,
    #[serde(rename = "propInt32")]
    pub prop_int32: Vec<i32>,
    #[serde(rename = "propInt64")]
    pub prop_int64: Vec<i64>,
    #[serde(rename = "propFloat")]
    pub prop_float: Vec<f32>,
    #[serde(rename = "propFloat32")]
    pub prop_float32: Vec<f32>,
    #[serde(rename = "propFloat64")]
    pub prop_float64: Vec<f64>,
    #[serde(rename = "propString")]
    pub prop_string: Vec<String>,
    #[serde(rename = "propReadOnlyString")]
    pub prop_read_only_string: String,
}
