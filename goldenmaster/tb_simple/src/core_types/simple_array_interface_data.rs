use serde::{Deserialize, Serialize};

/// Bundles all properties of SimpleArrayInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleArrayInterfaceData {
    pub prop_bool: Vec<bool>,
    pub prop_int: Vec<i32>,
    pub prop_int32: Vec<i32>,
    pub prop_int64: Vec<i64>,
    pub prop_float: Vec<f32>,
    pub prop_float32: Vec<f32>,
    pub prop_float64: Vec<f64>,
    pub prop_string: Vec<String>,
    pub prop_read_only_string: String,
}
