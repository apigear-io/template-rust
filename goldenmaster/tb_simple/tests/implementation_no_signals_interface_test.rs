use tb_simple::api::no_signals_interface::NoSignalsInterfaceTrait;
use tb_simple::implementation::no_signals_interface::NoSignalsInterface;

/// tests for NoSignalsInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func_void() {
        let mut test_object: NoSignalsInterface = Default::default();
        test_object.func_void(
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_void_async() {
        let mut test_object: NoSignalsInterface = Default::default();
        test_object.func_void_async(
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_bool() {
        let mut test_object: NoSignalsInterface = Default::default();
        test_object.func_bool(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_bool_async() {
        let mut test_object: NoSignalsInterface = Default::default();
        test_object.func_bool_async(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop_bool() {
        let mut test_object: NoSignalsInterface = Default::default();
        let default_value: bool = Default::default();
        test_object.set_prop_bool(default_value);
        assert_eq!(test_object.prop_bool().clone(), default_value);
    }

    #[test]
    #[rustfmt::skip]
    fn test_prop_int() {
        let mut test_object: NoSignalsInterface = Default::default();
        let default_value: i32 = Default::default();
        test_object.set_prop_int(default_value);
        assert_eq!(test_object.prop_int().clone(), default_value);
    }
}
