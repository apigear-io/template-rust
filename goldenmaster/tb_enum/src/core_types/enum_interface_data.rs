#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of EnumInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumInterfaceData {
    pub prop0: Enum0Enum,
    pub prop1: Enum1Enum,
    pub prop2: Enum2Enum,
    pub prop3: Enum3Enum,
}
