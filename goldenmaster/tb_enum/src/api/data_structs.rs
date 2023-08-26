use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
// Enumerations
/// Enumeration Enum0
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Enum0Enum {
    #[default]
    Value0 = 0,
    Value1 = 1,
    Value2 = 2,
}

impl TryFrom<u8> for Enum0Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Enum0Enum::Value0),
            1 => Ok(Enum0Enum::Value1),
            2 => Ok(Enum0Enum::Value2),
            _ => Err(()),
        }
    }
}

/// Enumeration Enum1
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Enum1Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
    Value3 = 3,
}

impl TryFrom<u8> for Enum1Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Enum1Enum::Value1),
            2 => Ok(Enum1Enum::Value2),
            3 => Ok(Enum1Enum::Value3),
            _ => Err(()),
        }
    }
}

/// Enumeration Enum2
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Enum2Enum {
    #[default]
    Value2 = 2,
    Value1 = 1,
    Value0 = 0,
}

impl TryFrom<u8> for Enum2Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Enum2Enum::Value2),
            1 => Ok(Enum2Enum::Value1),
            0 => Ok(Enum2Enum::Value0),
            _ => Err(()),
        }
    }
}

/// Enumeration Enum3
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Enum3Enum {
    #[default]
    Value3 = 3,
    Value2 = 2,
    Value1 = 1,
}

impl TryFrom<u8> for Enum3Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Enum3Enum::Value3),
            2 => Ok(Enum3Enum::Value2),
            1 => Ok(Enum3Enum::Value1),
            _ => Err(()),
        }
    }
}
