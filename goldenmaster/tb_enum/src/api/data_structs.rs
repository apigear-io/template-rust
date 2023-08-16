// Enumerations
/// Enumeration Enum0
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum0Enum {
    #[default]
    Value0 = 0,
    Value1 = 1,
    Value2 = 2,
}
// fn toEnum0Enum(v: u8, ok: *mut bool) -> Enum0Enum;
// fn fromEnum0Enum(v: Enum0Enum, ok: *mut bool) -> u8;

/// Enumeration Enum1
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum1Enum {
    #[default]
    Value1 = 1,
    Value2 = 2,
    Value3 = 3,
}
// fn toEnum1Enum(v: u8, ok: *mut bool) -> Enum1Enum;
// fn fromEnum1Enum(v: Enum1Enum, ok: *mut bool) -> u8;

/// Enumeration Enum2
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum2Enum {
    #[default]
    Value2 = 2,
    Value1 = 1,
    Value0 = 0,
}
// fn toEnum2Enum(v: u8, ok: *mut bool) -> Enum2Enum;
// fn fromEnum2Enum(v: Enum2Enum, ok: *mut bool) -> u8;

/// Enumeration Enum3
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Enum3Enum {
    #[default]
    Value3 = 3,
    Value2 = 2,
    Value1 = 1,
}
// fn toEnum3Enum(v: u8, ok: *mut bool) -> Enum3Enum;
// fn fromEnum3Enum(v: Enum3Enum, ok: *mut bool) -> u8;
