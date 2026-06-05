use serde::{Deserialize, Serialize};

/// Bundles all properties of SimpleInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleInterfaceData {
    #[serde(rename = "propBool")]
    pub prop_bool: bool,
    #[serde(rename = "propInt")]
    pub prop_int: i32,
    #[serde(rename = "propInt32")]
    pub prop_int32: i32,
    #[serde(rename = "propInt64")]
    pub prop_int64: i64,
    #[serde(rename = "propFloat")]
    pub prop_float: f32,
    #[serde(rename = "propFloat32")]
    pub prop_float32: f32,
    #[serde(rename = "propFloat64")]
    pub prop_float64: f64,
    #[serde(rename = "propString")]
    pub prop_string: String,
}
