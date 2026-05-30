#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of StructArrayInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructArrayInterfaceData {
    pub prop_bool: Vec<StructBool>,
    pub prop_int: Vec<StructInt>,
    pub prop_float: Vec<StructFloat>,
    pub prop_string: Vec<StructString>,
    pub prop_enum: Vec<Enum0Enum>,
}
