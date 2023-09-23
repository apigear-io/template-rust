use signals2::*;
use tb_simple::api::simple_array_interface::SimpleArrayInterfaceTrait;
use tb_simple::implementation::simple_array_interface::SimpleArrayInterface;

/// tests for SimpleArrayInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_bool() {
        let mut test_object: SimpleArrayInterface = Default::default();
        test_object.func_bool(Default::default());
    }

    #[test]
    fn test_func_bool_async() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let _ = test_object.func_bool_async(Default::default());
    }

    #[test]
    fn test_func_int() {
        let mut test_object: SimpleArrayInterface = Default::default();
        test_object.func_int(Default::default());
    }

    #[test]
    fn test_func_int_async() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let _ = test_object.func_int_async(Default::default());
    }

    #[test]
    fn test_func_int32() {
        let mut test_object: SimpleArrayInterface = Default::default();
        test_object.func_int32(Default::default());
    }

    #[test]
    fn test_func_int32_async() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let _ = test_object.func_int32_async(Default::default());
    }

    #[test]
    fn test_func_int64() {
        let mut test_object: SimpleArrayInterface = Default::default();
        test_object.func_int64(Default::default());
    }

    #[test]
    fn test_func_int64_async() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let _ = test_object.func_int64_async(Default::default());
    }

    #[test]
    fn test_func_float() {
        let mut test_object: SimpleArrayInterface = Default::default();
        test_object.func_float(Default::default());
    }

    #[test]
    fn test_func_float_async() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let _ = test_object.func_float_async(Default::default());
    }

    #[test]
    fn test_func_float32() {
        let mut test_object: SimpleArrayInterface = Default::default();
        test_object.func_float32(Default::default());
    }

    #[test]
    fn test_func_float32_async() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let _ = test_object.func_float32_async(Default::default());
    }

    #[test]
    fn test_func_float64() {
        let mut test_object: SimpleArrayInterface = Default::default();
        test_object.func_float64(Default::default());
    }

    #[test]
    fn test_func_float64_async() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let _ = test_object.func_float64_async(Default::default());
    }

    #[test]
    fn test_func_string() {
        let mut test_object: SimpleArrayInterface = Default::default();
        test_object.func_string(Default::default());
    }

    #[test]
    fn test_func_string_async() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let _ = test_object.func_string_async(Default::default());
    }

    #[test]
    fn test_prop_bool() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let default_value: Vec<bool> = Default::default();
        test_object.set_prop_bool(&default_value);
        assert_eq!(test_object.prop_bool().clone(), default_value);
    }

    #[test]
    fn test_prop_int() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let default_value: Vec<i32> = Default::default();
        test_object.set_prop_int(&default_value);
        assert_eq!(test_object.prop_int().clone(), default_value);
    }

    #[test]
    fn test_prop_int32() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let default_value: Vec<i32> = Default::default();
        test_object.set_prop_int32(&default_value);
        assert_eq!(test_object.prop_int32().clone(), default_value);
    }

    #[test]
    fn test_prop_int64() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let default_value: Vec<i64> = Default::default();
        test_object.set_prop_int64(&default_value);
        assert_eq!(test_object.prop_int64().clone(), default_value);
    }

    #[test]
    fn test_prop_float() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let default_value: Vec<f32> = Default::default();
        test_object.set_prop_float(&default_value);
        assert_eq!(test_object.prop_float().clone(), default_value);
    }

    #[test]
    fn test_prop_float32() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let default_value: Vec<f32> = Default::default();
        test_object.set_prop_float32(&default_value);
        assert_eq!(test_object.prop_float32().clone(), default_value);
    }

    #[test]
    fn test_prop_float64() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let default_value: Vec<f64> = Default::default();
        test_object.set_prop_float64(&default_value);
        assert_eq!(test_object.prop_float64().clone(), default_value);
    }

    #[test]
    fn test_prop_string() {
        let mut test_object: SimpleArrayInterface = Default::default();
        let default_value: Vec<String> = Default::default();
        test_object.set_prop_string(&default_value);
        assert_eq!(test_object.prop_string().clone(), default_value);
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_bool() {
        let mut test_object: SimpleArrayInterface = Default::default();

        test_object._get_signal_handler().sig_bool.connect(move |param_bool| {
            let default_value_param_bool: Vec<bool> = Default::default();
            assert_eq!(param_bool, default_value_param_bool);
        });

        let default_value_param_bool: Vec<bool> = Default::default();
        test_object._get_signal_handler().sig_bool.emit(
            default_value_param_bool.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_int() {
        let mut test_object: SimpleArrayInterface = Default::default();

        test_object._get_signal_handler().sig_int.connect(move |param_int| {
            let default_value_param_int: Vec<i32> = Default::default();
            assert_eq!(param_int, default_value_param_int);
        });

        let default_value_param_int: Vec<i32> = Default::default();
        test_object._get_signal_handler().sig_int.emit(
            default_value_param_int.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_int32() {
        let mut test_object: SimpleArrayInterface = Default::default();

        test_object._get_signal_handler().sig_int32.connect(move |param_int32| {
            let default_value_param_int32: Vec<i32> = Default::default();
            assert_eq!(param_int32, default_value_param_int32);
        });

        let default_value_param_int32: Vec<i32> = Default::default();
        test_object._get_signal_handler().sig_int32.emit(
            default_value_param_int32.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_int64() {
        let mut test_object: SimpleArrayInterface = Default::default();

        test_object._get_signal_handler().sig_int64.connect(move |param_int64| {
            let default_value_param_int64: Vec<i64> = Default::default();
            assert_eq!(param_int64, default_value_param_int64);
        });

        let default_value_param_int64: Vec<i64> = Default::default();
        test_object._get_signal_handler().sig_int64.emit(
            default_value_param_int64.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_float() {
        let mut test_object: SimpleArrayInterface = Default::default();

        test_object._get_signal_handler().sig_float.connect(move |param_float| {
            let default_value_param_float: Vec<f32> = Default::default();
            assert_eq!(param_float, default_value_param_float);
        });

        let default_value_param_float: Vec<f32> = Default::default();
        test_object._get_signal_handler().sig_float.emit(
            default_value_param_float.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_float32() {
        let mut test_object: SimpleArrayInterface = Default::default();

        test_object._get_signal_handler().sig_float32.connect(move |param_floa32| {
            let default_value_param_floa32: Vec<f32> = Default::default();
            assert_eq!(param_floa32, default_value_param_floa32);
        });

        let default_value_param_floa32: Vec<f32> = Default::default();
        test_object._get_signal_handler().sig_float32.emit(
            default_value_param_floa32.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_float64() {
        let mut test_object: SimpleArrayInterface = Default::default();

        test_object._get_signal_handler().sig_float64.connect(move |param_float64| {
            let default_value_param_float64: Vec<f64> = Default::default();
            assert_eq!(param_float64, default_value_param_float64);
        });

        let default_value_param_float64: Vec<f64> = Default::default();
        test_object._get_signal_handler().sig_float64.emit(
            default_value_param_float64.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_string() {
        let mut test_object: SimpleArrayInterface = Default::default();

        test_object._get_signal_handler().sig_string.connect(move |param_string| {
            let default_value_param_string: Vec<String> = Default::default();
            assert_eq!(param_string, default_value_param_string);
        });

        let default_value_param_string: Vec<String> = Default::default();
        test_object._get_signal_handler().sig_string.emit(
            default_value_param_string.clone(),
        );
    }
}
