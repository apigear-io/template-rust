#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of NestedStruct3Interface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedStruct3InterfaceData {
    pub prop1: NestedStruct1,
    pub prop2: NestedStruct2,
    pub prop3: NestedStruct3,
}
