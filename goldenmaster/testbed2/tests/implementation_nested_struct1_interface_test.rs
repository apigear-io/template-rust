// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
use testbed2::implementation::nested_struct1_interface::NestedStruct1Interface;

/// tests for NestedStruct1Interface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func1() {
        let mut test_object: NestedStruct1Interface = Default::default();
        test_object.func1(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func1_async() {
        let mut test_object: NestedStruct1Interface = Default::default();
        let _ = test_object.func1_async(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop1() {
        let mut test_object: NestedStruct1Interface = Default::default();
        let default_value: NestedStruct1 = Default::default();
        test_object.set_prop1(&default_value);
        assert_eq!(test_object.prop1().clone(), default_value);
    }
}
