// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use tb_same2::api::data_structs::*;
use tb_same2::api::same_struct1_interface::SameStruct1InterfaceTrait;
use tb_same2::implementation::same_struct1_interface::SameStruct1Interface;

/// tests for SameStruct1Interface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func1() {
        let mut test_object: SameStruct1Interface = Default::default();
        test_object.func1(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func1_async() {
        let mut test_object: SameStruct1Interface = Default::default();
        test_object.func1_async(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop1() {
        let mut test_object: SameStruct1Interface = Default::default();
        let default_value: Struct1 = Default::default();
        test_object.set_prop1(&default_value);
        assert_eq!(test_object.prop1().clone(), default_value);
    }
}
