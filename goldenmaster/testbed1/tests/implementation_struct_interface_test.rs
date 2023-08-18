// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use testbed1::api::data_structs::*;
use testbed1::api::struct_interface::StructInterfaceTrait;
use testbed1::implementation::struct_interface::StructInterface;

/// tests for StructInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func_bool() {
        let mut test_object: StructInterface = Default::default();
        test_object.func_bool(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_bool_async() {
        let mut test_object: StructInterface = Default::default();
        test_object.func_bool_async(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_int() {
        let mut test_object: StructInterface = Default::default();
        test_object.func_int(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_int_async() {
        let mut test_object: StructInterface = Default::default();
        test_object.func_int_async(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_float() {
        let mut test_object: StructInterface = Default::default();
        test_object.func_float(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_float_async() {
        let mut test_object: StructInterface = Default::default();
        test_object.func_float_async(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_string() {
        let mut test_object: StructInterface = Default::default();
        test_object.func_string(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_string_async() {
        let mut test_object: StructInterface = Default::default();
        test_object.func_string_async(
        &Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop_bool() {
        let mut test_object: StructInterface = Default::default();
        let default_value: StructBool = Default::default();
        test_object.set_prop_bool(&default_value);
        assert_eq!(test_object.prop_bool().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop_int() {
        let mut test_object: StructInterface = Default::default();
        let default_value: StructInt = Default::default();
        test_object.set_prop_int(&default_value);
        assert_eq!(test_object.prop_int().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop_float() {
        let mut test_object: StructInterface = Default::default();
        let default_value: StructFloat = Default::default();
        test_object.set_prop_float(&default_value);
        assert_eq!(test_object.prop_float().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop_string() {
        let mut test_object: StructInterface = Default::default();
        let default_value: StructString = Default::default();
        test_object.set_prop_string(&default_value);
        assert_eq!(test_object.prop_string().clone(), default_value);
    }
}
