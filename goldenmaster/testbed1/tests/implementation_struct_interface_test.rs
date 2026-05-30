#[allow(unused_imports)]
use testbed1::api::data_structs::*;
use testbed1::api::struct_interface::StructInterfaceTrait;
use testbed1::implementation::struct_interface::StructInterface;

/// tests for StructInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_enum0_enum() {
        assert_eq!(Enum0Enum::try_from(0), Ok(Enum0Enum::Value0));
        assert_eq!(Enum0Enum::try_from(0), Ok(Enum0Enum::default()));
        assert_eq!(Enum0Enum::try_from(1), Ok(Enum0Enum::Value1));
        assert_eq!(Enum0Enum::try_from(2), Ok(Enum0Enum::Value2));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum0Enum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum0_enum() {
        let result: Result<Enum0Enum, ()> = 0u8.try_into();
        assert_eq!(result, Ok(Enum0Enum::Value0));
        let result: Result<Enum0Enum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum0Enum::Value1));
        let result: Result<Enum0Enum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum0Enum::Value2));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum0Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[tokio::test]
    async fn test_func_bool() {
        let test_object = StructInterface::default();
        let result = test_object.func_bool(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_int() {
        let test_object = StructInterface::default();
        let result = test_object.func_int(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_float() {
        let test_object = StructInterface::default();
        let result = test_object.func_float(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func_string() {
        let test_object = StructInterface::default();
        let result = test_object.func_string(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_prop_bool() {
        let test_object = StructInterface::default();
        let default_value: StructBool = Default::default();
        test_object.set_prop_bool(&default_value.clone());
        assert_eq!(test_object.prop_bool(), default_value);
    }

    #[test]
    fn test_prop_int() {
        let test_object = StructInterface::default();
        let default_value: StructInt = Default::default();
        test_object.set_prop_int(&default_value.clone());
        assert_eq!(test_object.prop_int(), default_value);
    }

    #[test]
    fn test_prop_float() {
        let test_object = StructInterface::default();
        let default_value: StructFloat = Default::default();
        test_object.set_prop_float(&default_value.clone());
        assert_eq!(test_object.prop_float(), default_value);
    }

    #[test]
    fn test_prop_string() {
        let test_object = StructInterface::default();
        let default_value: StructString = Default::default();
        test_object.set_prop_string(&default_value.clone());
        assert_eq!(test_object.prop_string(), default_value);
    }

    #[test]
    fn test_sig_bool() {
        let test_object = StructInterface::default();
        let mut rx = test_object.publisher().sig_bool.subscribe();
        let default_value_param_bool: StructBool = Default::default();
        let _ = test_object.publisher().sig_bool.send((default_value_param_bool.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_bool);
    }

    #[test]
    fn test_sig_int() {
        let test_object = StructInterface::default();
        let mut rx = test_object.publisher().sig_int.subscribe();
        let default_value_param_int: StructInt = Default::default();
        let _ = test_object.publisher().sig_int.send((default_value_param_int.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_int);
    }

    #[test]
    fn test_sig_float() {
        let test_object = StructInterface::default();
        let mut rx = test_object.publisher().sig_float.subscribe();
        let default_value_param_float: StructFloat = Default::default();
        let _ = test_object.publisher().sig_float.send((default_value_param_float.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_float);
    }

    #[test]
    fn test_sig_string() {
        let test_object = StructInterface::default();
        let mut rx = test_object.publisher().sig_string.subscribe();
        let default_value_param_string: StructString = Default::default();
        let _ = test_object.publisher().sig_string.send((default_value_param_string.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param_string);
    }
}
