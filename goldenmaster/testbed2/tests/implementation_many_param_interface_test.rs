// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
use testbed2::implementation::many_param_interface::ManyParamInterface;

/// tests for ManyParamInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func1() {
        let mut test_object: ManyParamInterface = Default::default();
        test_object.func1(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func1_async() {
        let mut test_object: ManyParamInterface = Default::default();
        test_object.func1_async(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func2() {
        let mut test_object: ManyParamInterface = Default::default();
        test_object.func2(
        Default::default(),
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func2_async() {
        let mut test_object: ManyParamInterface = Default::default();
        test_object.func2_async(
        Default::default(),
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func3() {
        let mut test_object: ManyParamInterface = Default::default();
        test_object.func3(
        Default::default(),
        Default::default(),
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func3_async() {
        let mut test_object: ManyParamInterface = Default::default();
        test_object.func3_async(
        Default::default(),
        Default::default(),
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func4() {
        let mut test_object: ManyParamInterface = Default::default();
        test_object.func4(
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func4_async() {
        let mut test_object: ManyParamInterface = Default::default();
        test_object.func4_async(
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop1() {
        let mut test_object: ManyParamInterface = Default::default();
        let default_value: i32 = Default::default();
        test_object.set_prop1(default_value);
        assert_eq!(test_object.prop1().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop2() {
        let mut test_object: ManyParamInterface = Default::default();
        let default_value: i32 = Default::default();
        test_object.set_prop2(default_value);
        assert_eq!(test_object.prop2().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop3() {
        let mut test_object: ManyParamInterface = Default::default();
        let default_value: i32 = Default::default();
        test_object.set_prop3(default_value);
        assert_eq!(test_object.prop3().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop4() {
        let mut test_object: ManyParamInterface = Default::default();
        let default_value: i32 = Default::default();
        test_object.set_prop4(default_value);
        assert_eq!(test_object.prop4().clone(), default_value);
    }
}
