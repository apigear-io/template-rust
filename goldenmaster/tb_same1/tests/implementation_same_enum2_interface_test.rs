// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use tb_same1::api::data_structs::*;
use tb_same1::api::same_enum2_interface::SameEnum2InterfaceTrait;
use tb_same1::implementation::same_enum2_interface::SameEnum2Interface;

/// tests for SameEnum2Interface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func1() {
        let mut test_object: SameEnum2Interface = Default::default();
        test_object.func1(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func1_async() {
        let mut test_object: SameEnum2Interface = Default::default();
        test_object.func1_async(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func2() {
        let mut test_object: SameEnum2Interface = Default::default();
        test_object.func2(
        Default::default(),
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func2_async() {
        let mut test_object: SameEnum2Interface = Default::default();
        test_object.func2_async(
        Default::default(),
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop1() {
        let mut test_object: SameEnum2Interface = Default::default();
        let default_value: Enum1Enum = Default::default();
        test_object.set_prop1(default_value);
        assert_eq!(test_object.prop1().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop2() {
        let mut test_object: SameEnum2Interface = Default::default();
        let default_value: Enum2Enum = Default::default();
        test_object.set_prop2(default_value);
        assert_eq!(test_object.prop2().clone(), default_value);
    }
}
