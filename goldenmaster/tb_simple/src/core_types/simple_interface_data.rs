use serde::{Deserialize, Serialize};

/// Bundles all properties of SimpleInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleInterfaceData {
    pub prop_bool: bool,
    pub prop_int: i32,
    pub prop_int32: i32,
    pub prop_int64: i64,
    pub prop_float: f32,
    pub prop_float32: f32,
    pub prop_float64: f64,
    pub prop_string: String,
}
