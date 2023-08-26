use serde::{Deserialize, Serialize};

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
