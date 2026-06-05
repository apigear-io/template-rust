#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of EnumInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumInterfaceData {
    #[serde(rename = "prop0")]
    pub prop0: Enum0Enum,
    #[serde(rename = "prop1")]
    pub prop1: Enum1Enum,
    #[serde(rename = "prop2")]
    pub prop2: Enum2Enum,
    #[serde(rename = "prop3")]
    pub prop3: Enum3Enum,
}
