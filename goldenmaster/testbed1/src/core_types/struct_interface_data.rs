#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of StructInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructInterfaceData {
    #[serde(rename = "propBool")]
    pub prop_bool: StructBool,
    #[serde(rename = "propInt")]
    pub prop_int: StructInt,
    #[serde(rename = "propFloat")]
    pub prop_float: StructFloat,
    #[serde(rename = "propString")]
    pub prop_string: StructString,
}
