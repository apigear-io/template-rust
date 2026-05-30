#[allow(unused_imports)]
use crate::api::data_structs::*;

/// Creates a Struct1 populated with test values.
pub fn fill_test_struct1() -> Struct1 {
    Struct1 { field1: 1 }
}

/// Creates a Struct2 populated with test values.
pub fn fill_test_struct2() -> Struct2 {
    Struct2 { field1: 1, field2: 1 }
}

/// Creates a Struct3 populated with test values.
pub fn fill_test_struct3() -> Struct3 {
    Struct3 { field1: 1, field2: 1, field3: 1 }
}

/// Creates a Struct4 populated with test values.
pub fn fill_test_struct4() -> Struct4 {
    Struct4 { field1: 1, field2: 1, field3: 1, field4: 1 }
}

/// Creates a NestedStruct1 populated with test values.
pub fn fill_test_nested_struct1() -> NestedStruct1 {
    NestedStruct1 { field1: Default::default() }
}

/// Creates a NestedStruct2 populated with test values.
pub fn fill_test_nested_struct2() -> NestedStruct2 {
    NestedStruct2 { field1: Default::default(), field2: Default::default() }
}

/// Creates a NestedStruct3 populated with test values.
pub fn fill_test_nested_struct3() -> NestedStruct3 {
    NestedStruct3 { field1: Default::default(), field2: Default::default(), field3: Default::default() }
}
