use serde::{Deserialize, Serialize};

/// Bundles all properties of NoSignalsInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoSignalsInterfaceData {
    pub prop_bool: bool,
    pub prop_int: i32,
}
