use serde::{Deserialize, Serialize};

/// Bundles all properties of NoPropertiesInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoPropertiesInterfaceData {}
