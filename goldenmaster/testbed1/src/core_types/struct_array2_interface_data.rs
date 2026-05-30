#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of StructArray2Interface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructArray2InterfaceData {
    pub prop_bool: StructBoolWithArray,
    pub prop_int: StructIntWithArray,
    pub prop_float: StructFloatWithArray,
    pub prop_string: StructStringWithArray,
    pub prop_enum: StructEnumWithArray,
}
