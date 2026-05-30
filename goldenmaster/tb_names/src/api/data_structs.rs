#![allow(non_camel_case_types)]
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
// Enumerations
/// Enumeration Enum_With_Under_scores
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Enum_With_Under_scoresEnum {
    #[default]
    First_Value = 0,
    Second_value = 1,
    Third_Value = 2,
}

impl TryFrom<u8> for Enum_With_Under_scoresEnum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Enum_With_Under_scoresEnum::First_Value),
            1 => Ok(Enum_With_Under_scoresEnum::Second_value),
            2 => Ok(Enum_With_Under_scoresEnum::Third_Value),
            _ => Err(()),
        }
    }
}
