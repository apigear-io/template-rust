use tb_simple::api::no_operations_interface::NoOperationsInterfaceTrait;
use tb_simple::implementation::no_operations_interface::NoOperationsInterface;

/// tests for NoOperationsInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prop_bool() {
        let test_object = NoOperationsInterface::default();
        let default_value: bool = Default::default();
        test_object.set_prop_bool(default_value);
        assert_eq!(test_object.prop_bool(), default_value);
    }

    #[test]
    fn test_prop_int() {
        let test_object = NoOperationsInterface::default();
        let default_value: i32 = Default::default();
        test_object.set_prop_int(default_value);
        assert_eq!(test_object.prop_int(), default_value);
    }

    #[test]
    fn test_sig_void() {
        let test_object = NoOperationsInterface::default();
        let mut rx = test_object.publisher().sig_void.subscribe();
        let _ = test_object.publisher().sig_void.send(());
        assert!(rx.try_recv().is_ok());
    }

    #[test]
    fn test_sig_bool() {
        let test_object = NoOperationsInterface::default();
        let mut rx = test_object.publisher().sig_bool.subscribe();
        let default_value_param_bool: bool = Default::default();
        let _ = test_object.publisher().sig_bool.send((default_value_param_bool.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_bool);
    }
}
