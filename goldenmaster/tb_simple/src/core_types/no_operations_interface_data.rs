use serde::{Deserialize, Serialize};

/// Bundles all properties of NoOperationsInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoOperationsInterfaceData {
    pub prop_bool: bool,
    pub prop_int: i32,
}
