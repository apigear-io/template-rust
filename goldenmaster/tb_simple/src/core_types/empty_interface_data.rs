use serde::{Deserialize, Serialize};

/// Bundles all properties of EmptyInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyInterfaceData {}
