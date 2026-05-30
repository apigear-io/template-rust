use tb_simple::api::simple_interface::SimpleInterfaceTrait;
use tb_simple::implementation::simple_interface::SimpleInterface;

/// tests for SimpleInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_func_no_return_value() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_no_return_value(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_no_params() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_no_params().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_bool() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_bool(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_int() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_int(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_int32() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_int32(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_int64() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_int64(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_float() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_float(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_float32() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_float32(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_float64() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_float64(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_string() {
        let test_object = SimpleInterface::default();
        let result = test_object.func_string(Default::default()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_prop_bool() {
        let test_object = SimpleInterface::default();
        let default_value: bool = Default::default();
        test_object.set_prop_bool(default_value);
        assert_eq!(test_object.prop_bool(), default_value);
    }

    #[test]
    fn test_prop_int() {
        let test_object = SimpleInterface::default();
        let default_value: i32 = Default::default();
        test_object.set_prop_int(default_value);
        assert_eq!(test_object.prop_int(), default_value);
    }

    #[test]
    fn test_prop_int32() {
        let test_object = SimpleInterface::default();
        let default_value: i32 = Default::default();
        test_object.set_prop_int32(default_value);
        assert_eq!(test_object.prop_int32(), default_value);
    }

    #[test]
    fn test_prop_int64() {
        let test_object = SimpleInterface::default();
        let default_value: i64 = Default::default();
        test_object.set_prop_int64(default_value);
        assert_eq!(test_object.prop_int64(), default_value);
    }

    #[test]
    fn test_prop_float() {
        let test_object = SimpleInterface::default();
        let default_value: f32 = Default::default();
        test_object.set_prop_float(default_value);
        assert_eq!(test_object.prop_float(), default_value);
    }

    #[test]
    fn test_prop_float32() {
        let test_object = SimpleInterface::default();
        let default_value: f32 = Default::default();
        test_object.set_prop_float32(default_value);
        assert_eq!(test_object.prop_float32(), default_value);
    }

    #[test]
    fn test_prop_float64() {
        let test_object = SimpleInterface::default();
        let default_value: f64 = Default::default();
        test_object.set_prop_float64(default_value);
        assert_eq!(test_object.prop_float64(), default_value);
    }

    #[test]
    fn test_prop_string() {
        let test_object = SimpleInterface::default();
        let default_value: String = Default::default();
        test_object.set_prop_string(default_value.as_str());
        assert_eq!(test_object.prop_string(), default_value);
    }

    #[test]
    fn test_sig_bool() {
        let test_object = SimpleInterface::default();
        let mut rx = test_object.publisher().sig_bool.subscribe();
        let default_value_param_bool: bool = Default::default();
        let _ = test_object.publisher().sig_bool.send((default_value_param_bool.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_bool);
    }

    #[test]
    fn test_sig_int() {
        let test_object = SimpleInterface::default();
        let mut rx = test_object.publisher().sig_int.subscribe();
        let default_value_param_int: i32 = Default::default();
        let _ = test_object.publisher().sig_int.send((default_value_param_int.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_int);
    }

    #[test]
    fn test_sig_int32() {
        let test_object = SimpleInterface::default();
        let mut rx = test_object.publisher().sig_int32.subscribe();
        let default_value_param_int32: i32 = Default::default();
        let _ = test_object.publisher().sig_int32.send((default_value_param_int32.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_int32);
    }

    #[test]
    fn test_sig_int64() {
        let test_object = SimpleInterface::default();
        let mut rx = test_object.publisher().sig_int64.subscribe();
        let default_value_param_int64: i64 = Default::default();
        let _ = test_object.publisher().sig_int64.send((default_value_param_int64.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_int64);
    }

    #[test]
    fn test_sig_float() {
        let test_object = SimpleInterface::default();
        let mut rx = test_object.publisher().sig_float.subscribe();
        let default_value_param_float: f32 = Default::default();
        let _ = test_object.publisher().sig_float.send((default_value_param_float.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_float);
    }

    #[test]
    fn test_sig_float32() {
        let test_object = SimpleInterface::default();
        let mut rx = test_object.publisher().sig_float32.subscribe();
        let default_value_param_float32: f32 = Default::default();
        let _ = test_object.publisher().sig_float32.send((default_value_param_float32.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_float32);
    }

    #[test]
    fn test_sig_float64() {
        let test_object = SimpleInterface::default();
        let mut rx = test_object.publisher().sig_float64.subscribe();
        let default_value_param_float64: f64 = Default::default();
        let _ = test_object.publisher().sig_float64.send((default_value_param_float64.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_float64);
    }

    #[test]
    fn test_sig_string() {
        let test_object = SimpleInterface::default();
        let mut rx = test_object.publisher().sig_string.subscribe();
        let default_value_param_string: String = Default::default();
        let _ = test_object.publisher().sig_string.send((default_value_param_string.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_string);
    }
}
