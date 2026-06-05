#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of NestedStruct2Interface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedStruct2InterfaceData {
    #[serde(rename = "prop1")]
    pub prop1: NestedStruct1,
    #[serde(rename = "prop2")]
    pub prop2: NestedStruct2,
}
