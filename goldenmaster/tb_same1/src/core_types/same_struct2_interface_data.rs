#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of SameStruct2Interface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SameStruct2InterfaceData {
    #[serde(rename = "prop1")]
    pub prop1: Struct2,
    #[serde(rename = "prop2")]
    pub prop2: Struct2,
}
