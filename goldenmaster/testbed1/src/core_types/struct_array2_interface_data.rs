#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of StructArray2Interface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructArray2InterfaceData {
    #[serde(rename = "propBool")]
    pub prop_bool: StructBoolWithArray,
    #[serde(rename = "propInt")]
    pub prop_int: StructIntWithArray,
    #[serde(rename = "propFloat")]
    pub prop_float: StructFloatWithArray,
    #[serde(rename = "propString")]
    pub prop_string: StructStringWithArray,
    #[serde(rename = "propEnum")]
    pub prop_enum: StructEnumWithArray,
}
