#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of SameEnum1Interface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SameEnum1InterfaceData {
    pub prop1: Enum1Enum,
}
