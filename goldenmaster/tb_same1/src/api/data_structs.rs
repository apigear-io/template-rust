// Enumerations
/// Enumeration Enum1
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum1Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
}
// fn toEnum1Enum(v: u8, ok: *mut bool) -> Enum1Enum;
// fn fromEnum1Enum(v: Enum1Enum, ok: *mut bool) -> u8;

/// Enumeration Enum2
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum2Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
}
// fn toEnum2Enum(v: u8, ok: *mut bool) -> Enum2Enum;
// fn fromEnum2Enum(v: Enum2Enum, ok: *mut bool) -> u8;

// Structs
/// Struct Struct1
#[derive(Default, Clone, PartialEq)]
pub struct Struct1 {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
}

/// Struct Struct2
#[derive(Default, Clone, PartialEq)]
pub struct Struct2 {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
}
