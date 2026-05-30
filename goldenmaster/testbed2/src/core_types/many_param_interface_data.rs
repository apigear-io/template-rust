#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of ManyParamInterface for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManyParamInterfaceData {
    pub prop1: i32,
    pub prop2: i32,
    pub prop3: i32,
    pub prop4: i32,
}
