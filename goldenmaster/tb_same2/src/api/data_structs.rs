use std::convert::TryFrom;
// Enumerations
/// Enumeration Enum1
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Enum1Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
}

impl TryFrom<u8> for Enum1Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Enum1Enum::Value1),
            2 => Ok(Enum1Enum::Value2),
            _ => Err(()),
        }
    }
}

/// Enumeration Enum2
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Enum2Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
}

impl TryFrom<u8> for Enum2Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Enum2Enum::Value1),
            2 => Ok(Enum2Enum::Value2),
            _ => Err(()),
        }
    }
}

// Structs
/// Struct Struct1
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Struct1 {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
}

/// Struct Struct2
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Struct2 {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
}
