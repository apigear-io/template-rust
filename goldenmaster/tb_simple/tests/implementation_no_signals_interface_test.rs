use tb_simple::api::no_signals_interface::NoSignalsInterfaceTrait;
use tb_simple::implementation::no_signals_interface::NoSignalsInterface;

/// tests for NoSignalsInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_func_void() {
        let test_object = NoSignalsInterface::default();
        let result = test_object.func_void().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_bool() {
        let test_object = NoSignalsInterface::default();
        let result = test_object.func_bool(Default::default()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_prop_bool() {
        let test_object = NoSignalsInterface::default();
        let default_value: bool = Default::default();
        test_object.set_prop_bool(default_value);
        assert_eq!(test_object.prop_bool(), default_value);
    }

    #[test]
    fn test_prop_int() {
        let test_object = NoSignalsInterface::default();
        let default_value: i32 = Default::default();
        test_object.set_prop_int(default_value);
        assert_eq!(test_object.prop_int(), default_value);
    }
}
