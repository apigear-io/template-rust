use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTrait;
use tb_simple::implementation::no_properties_interface::NoPropertiesInterface;

/// tests for NoPropertiesInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_func_void() {
        let test_object = NoPropertiesInterface::default();
        let result = test_object.func_void().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_bool() {
        let test_object = NoPropertiesInterface::default();
        let result = test_object.func_bool(Default::default()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_sig_void() {
        let test_object = NoPropertiesInterface::default();
        let mut rx = test_object.publisher().sig_void.subscribe();
        let _ = test_object.publisher().sig_void.send(());
        assert!(rx.try_recv().is_ok());
    }

    #[test]
    fn test_sig_bool() {
        let test_object = NoPropertiesInterface::default();
        let mut rx = test_object.publisher().sig_bool.subscribe();
        let default_value_param_bool: bool = Default::default();
        let _ = test_object.publisher().sig_bool.send((default_value_param_bool.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_bool);
    }
}
