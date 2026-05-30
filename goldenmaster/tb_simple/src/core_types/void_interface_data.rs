use serde::{Deserialize, Serialize};

/// Bundles all properties of VoidInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoidInterfaceData {}
