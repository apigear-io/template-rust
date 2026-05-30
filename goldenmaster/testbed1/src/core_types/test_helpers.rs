#[allow(unused_imports)]
use crate::api::data_structs::*;

/// Creates a StructBool populated with test values.
pub fn fill_test_struct_bool() -> StructBool {
    StructBool { field_bool: true }
}

/// Creates a StructInt populated with test values.
pub fn fill_test_struct_int() -> StructInt {
    StructInt { field_int: 1 }
}

/// Creates a StructFloat populated with test values.
pub fn fill_test_struct_float() -> StructFloat {
    StructFloat { field_float: 1.0 }
}

/// Creates a StructString populated with test values.
pub fn fill_test_struct_string() -> StructString {
    StructString { field_string: String::from("test") }
}

/// Creates a StructStruct populated with test values.
pub fn fill_test_struct_struct() -> StructStruct {
    StructStruct { field_string: Default::default() }
}

/// Creates a StructEnum populated with test values.
pub fn fill_test_struct_enum() -> StructEnum {
    StructEnum { field_enum: Default::default() }
}

/// Creates a StructBoolWithArray populated with test values.
pub fn fill_test_struct_bool_with_array() -> StructBoolWithArray {
    StructBoolWithArray { field_bool: vec![Default::default()] }
}

/// Creates a StructIntWithArray populated with test values.
pub fn fill_test_struct_int_with_array() -> StructIntWithArray {
    StructIntWithArray { field_int: vec![Default::default()] }
}

/// Creates a StructFloatWithArray populated with test values.
pub fn fill_test_struct_float_with_array() -> StructFloatWithArray {
    StructFloatWithArray { field_float: vec![Default::default()] }
}

/// Creates a StructStringWithArray populated with test values.
pub fn fill_test_struct_string_with_array() -> StructStringWithArray {
    StructStringWithArray { field_string: vec![Default::default()] }
}

/// Creates a StructStructWithArray populated with test values.
pub fn fill_test_struct_struct_with_array() -> StructStructWithArray {
    StructStructWithArray { field_struct: vec![Default::default()] }
}

/// Creates a StructEnumWithArray populated with test values.
pub fn fill_test_struct_enum_with_array() -> StructEnumWithArray {
    StructEnumWithArray { field_enum: vec![Default::default()] }
}
