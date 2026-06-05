#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of StructArrayInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructArrayInterfaceData {
    #[serde(rename = "propBool")]
    pub prop_bool: Vec<StructBool>,
    #[serde(rename = "propInt")]
    pub prop_int: Vec<StructInt>,
    #[serde(rename = "propFloat")]
    pub prop_float: Vec<StructFloat>,
    #[serde(rename = "propString")]
    pub prop_string: Vec<StructString>,
    #[serde(rename = "propEnum")]
    pub prop_enum: Vec<Enum0Enum>,
}
