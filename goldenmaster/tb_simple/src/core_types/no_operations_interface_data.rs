use serde::{Deserialize, Serialize};

/// Bundles all properties of NoOperationsInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoOperationsInterfaceData {
    #[serde(rename = "propBool")]
    pub prop_bool: bool,
    #[serde(rename = "propInt")]
    pub prop_int: i32,
}
