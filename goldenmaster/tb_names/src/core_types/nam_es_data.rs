#[allow(unused_imports)]
use crate::api::data_structs::*;
use serde::{Deserialize, Serialize};

/// Bundles all properties of NamEs for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamEsData {
    pub switch: bool,
    pub some_property: i32,
    pub some_poperty2: i32,
    pub enum_property: Enum_With_Under_scoresEnum,
}
