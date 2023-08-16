// Enumerations
/// Enumeration Enum1
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum1Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
    Value3 = 3,
    Value4 = 4,
}
// fn toEnum1Enum(v: u8, ok: *mut bool) -> Enum1Enum;
// fn fromEnum1Enum(v: Enum1Enum, ok: *mut bool) -> u8;

/// Enumeration Enum2
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum2Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
    Value3 = 3,
    Value4 = 4,
}
// fn toEnum2Enum(v: u8, ok: *mut bool) -> Enum2Enum;
// fn fromEnum2Enum(v: Enum2Enum, ok: *mut bool) -> u8;

/// Enumeration Enum3
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum3Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
    Value3 = 3,
    Value4 = 4,
}
// fn toEnum3Enum(v: u8, ok: *mut bool) -> Enum3Enum;
// fn fromEnum3Enum(v: Enum3Enum, ok: *mut bool) -> u8;

// Structs
/// Struct Struct1
#[derive(Default, Clone, PartialEq)]
pub struct Struct1 {
    pub field1: i32,
}

/// Struct Struct2
#[derive(Default, Clone, PartialEq)]
pub struct Struct2 {
    pub field1: i32,
    pub field2: i32,
}

/// Struct Struct3
#[derive(Default, Clone, PartialEq)]
pub struct Struct3 {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
}

/// Struct Struct4
#[derive(Default, Clone, PartialEq)]
pub struct Struct4 {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
    pub field4: i32,
}

/// Struct NestedStruct1
#[derive(Default, Clone, PartialEq)]
pub struct NestedStruct1 {
    pub field1: Struct1,
}

/// Struct NestedStruct2
#[derive(Default, Clone, PartialEq)]
pub struct NestedStruct2 {
    pub field1: Struct1,
    pub field2: Struct2,
}

/// Struct NestedStruct3
#[derive(Default, Clone, PartialEq)]
pub struct NestedStruct3 {
    pub field1: Struct1,
    pub field2: Struct2,
    pub field3: Struct3,
}
