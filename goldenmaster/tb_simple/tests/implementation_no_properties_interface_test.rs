use signals2::*;
use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTrait;
use tb_simple::implementation::no_properties_interface::NoPropertiesInterface;

/// tests for NoPropertiesInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_void() {
        let mut test_object: NoPropertiesInterface = Default::default();
        test_object.func_void();
    }

    #[test]
    fn test_func_void_async() {
        let mut test_object: NoPropertiesInterface = Default::default();
        let _ = test_object.func_void_async();
    }

    #[test]
    fn test_func_bool() {
        let mut test_object: NoPropertiesInterface = Default::default();
        test_object.func_bool(Default::default());
    }

    #[test]
    fn test_func_bool_async() {
        let mut test_object: NoPropertiesInterface = Default::default();
        let _ = test_object.func_bool_async(Default::default());
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_void() {
        let mut test_object: NoPropertiesInterface = Default::default();

        test_object._get_signal_handler().sig_void.connect(move || {
        });

        test_object._get_signal_handler().sig_void.emit(
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig_bool() {
        let mut test_object: NoPropertiesInterface = Default::default();

        test_object._get_signal_handler().sig_bool.connect(move |param_bool| {
            let default_value_param_bool: bool = Default::default();
            assert_eq!(param_bool, default_value_param_bool);
        });

        let default_value_param_bool: bool = Default::default();
        test_object._get_signal_handler().sig_bool.emit(
            default_value_param_bool.clone(),
        );
    }
}
