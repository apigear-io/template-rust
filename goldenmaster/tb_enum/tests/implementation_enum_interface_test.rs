// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use tb_enum::api::data_structs::*;
use tb_enum::api::enum_interface::EnumInterfaceTrait;
use tb_enum::implementation::enum_interface::EnumInterface;

/// tests for EnumInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func0() {
        let mut test_object: EnumInterface = Default::default();
        test_object.func0(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func0_async() {
        let mut test_object: EnumInterface = Default::default();
        let _ = test_object.func0_async(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func1() {
        let mut test_object: EnumInterface = Default::default();
        test_object.func1(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func1_async() {
        let mut test_object: EnumInterface = Default::default();
        let _ = test_object.func1_async(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func2() {
        let mut test_object: EnumInterface = Default::default();
        test_object.func2(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func2_async() {
        let mut test_object: EnumInterface = Default::default();
        let _ = test_object.func2_async(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func3() {
        let mut test_object: EnumInterface = Default::default();
        test_object.func3(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func3_async() {
        let mut test_object: EnumInterface = Default::default();
        let _ = test_object.func3_async(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop0() {
        let mut test_object: EnumInterface = Default::default();
        let default_value: Enum0Enum = Default::default();
        test_object.set_prop0(default_value);
        assert_eq!(test_object.prop0().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop1() {
        let mut test_object: EnumInterface = Default::default();
        let default_value: Enum1Enum = Default::default();
        test_object.set_prop1(default_value);
        assert_eq!(test_object.prop1().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop2() {
        let mut test_object: EnumInterface = Default::default();
        let default_value: Enum2Enum = Default::default();
        test_object.set_prop2(default_value);
        assert_eq!(test_object.prop2().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop3() {
        let mut test_object: EnumInterface = Default::default();
        let default_value: Enum3Enum = Default::default();
        test_object.set_prop3(default_value);
        assert_eq!(test_object.prop3().clone(), default_value);
    }
}
