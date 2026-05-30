#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of StructInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructInterfaceData {
    pub prop_bool: StructBool,
    pub prop_int: StructInt,
    pub prop_float: StructFloat,
    pub prop_string: StructString,
}
