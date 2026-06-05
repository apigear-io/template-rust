#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of NamEs for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamEsData {
    #[serde(rename = "Switch")]
    pub switch: bool,
    #[serde(rename = "SOME_PROPERTY")]
    pub some_property: i32,
    #[serde(rename = "Some_Poperty2")]
    pub some_poperty2: i32,
    #[serde(rename = "enum_property")]
    pub enum_property: Enum_With_Under_scoresEnum,
}
