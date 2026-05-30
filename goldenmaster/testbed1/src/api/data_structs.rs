#![allow(non_camel_case_types)]
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

// Structs
/// Struct StructBool
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructBool {
    pub field_bool: bool,
}

/// Struct StructInt
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructInt {
    pub field_int: i32,
}

/// Struct StructFloat
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructFloat {
    pub field_float: f32,
}

/// Struct StructString
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructString {
    pub field_string: String,
}

/// Struct StructStruct
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructStruct {
    pub field_string: StructString,
}

/// Struct StructEnum
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructEnum {
    pub field_enum: Enum0Enum,
}

/// Struct StructBoolWithArray
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructBoolWithArray {
    pub field_bool: Vec<bool>,
}

/// Struct StructIntWithArray
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructIntWithArray {
    pub field_int: Vec<i32>,
}

/// Struct StructFloatWithArray
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructFloatWithArray {
    pub field_float: Vec<f32>,
}

/// Struct StructStringWithArray
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructStringWithArray {
    pub field_string: Vec<String>,
}

/// Struct StructStructWithArray
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructStructWithArray {
    pub field_struct: Vec<StructStringWithArray>,
}

/// Struct StructEnumWithArray
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructEnumWithArray {
    pub field_enum: Vec<Enum0Enum>,
}
