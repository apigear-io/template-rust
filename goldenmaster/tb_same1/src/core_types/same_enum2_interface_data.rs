#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of SameEnum2Interface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SameEnum2InterfaceData {
    pub prop1: Enum1Enum,
    pub prop2: Enum2Enum,
}
