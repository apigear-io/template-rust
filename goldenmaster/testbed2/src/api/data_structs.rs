use std::convert::TryFrom;
// Enumerations
/// Enumeration Enum1
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Enum1Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
    Value3 = 3,
    Value4 = 4,
}

impl TryFrom<u8> for Enum1Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Enum1Enum::Value1),
            2 => Ok(Enum1Enum::Value2),
            3 => Ok(Enum1Enum::Value3),
            4 => Ok(Enum1Enum::Value4),
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
    Value3 = 3,
    Value4 = 4,
}

impl TryFrom<u8> for Enum2Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Enum2Enum::Value1),
            2 => Ok(Enum2Enum::Value2),
            3 => Ok(Enum2Enum::Value3),
            4 => Ok(Enum2Enum::Value4),
            _ => Err(()),
        }
    }
}

/// Enumeration Enum3
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Enum3Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
    Value3 = 3,
    Value4 = 4,
}

impl TryFrom<u8> for Enum3Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Enum3Enum::Value1),
            2 => Ok(Enum3Enum::Value2),
            3 => Ok(Enum3Enum::Value3),
            4 => Ok(Enum3Enum::Value4),
            _ => Err(()),
        }
    }
}

// Structs
/// Struct Struct1
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Struct1 {
    pub field1: i32,
}

/// Struct Struct2
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Struct2 {
    pub field1: i32,
    pub field2: i32,
}

/// Struct Struct3
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Struct3 {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
}

/// Struct Struct4
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Struct4 {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
    pub field4: i32,
}

/// Struct NestedStruct1
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NestedStruct1 {
    pub field1: Struct1,
}

/// Struct NestedStruct2
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NestedStruct2 {
    pub field1: Struct1,
    pub field2: Struct2,
}

/// Struct NestedStruct3
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NestedStruct3 {
    pub field1: Struct1,
    pub field2: Struct2,
    pub field3: Struct3,
}
