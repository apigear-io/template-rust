#[allow(unused_imports)]
use tb_enum::api::data_structs::*;
use tb_enum::api::enum_interface::EnumInterfaceTrait;
use tb_enum::implementation::enum_interface::EnumInterface;

/// tests for EnumInterface
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

    #[test]
    fn test_to_enum1_enum() {
        assert_eq!(Enum1Enum::try_from(1), Ok(Enum1Enum::Value1));
        assert_eq!(Enum1Enum::try_from(1), Ok(Enum1Enum::default()));
        assert_eq!(Enum1Enum::try_from(2), Ok(Enum1Enum::Value2));
        assert_eq!(Enum1Enum::try_from(3), Ok(Enum1Enum::Value3));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum1Enum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum1_enum() {
        let result: Result<Enum1Enum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum1Enum::Value1));
        let result: Result<Enum1Enum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum1Enum::Value2));
        let result: Result<Enum1Enum, ()> = 3u8.try_into();
        assert_eq!(result, Ok(Enum1Enum::Value3));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum1Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_to_enum2_enum() {
        assert_eq!(Enum2Enum::try_from(2), Ok(Enum2Enum::Value2));
        assert_eq!(Enum2Enum::try_from(2), Ok(Enum2Enum::default()));
        assert_eq!(Enum2Enum::try_from(1), Ok(Enum2Enum::Value1));
        assert_eq!(Enum2Enum::try_from(0), Ok(Enum2Enum::Value0));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum2Enum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum2_enum() {
        let result: Result<Enum2Enum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value2));
        let result: Result<Enum2Enum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value1));
        let result: Result<Enum2Enum, ()> = 0u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value0));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum2Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_to_enum3_enum() {
        assert_eq!(Enum3Enum::try_from(3), Ok(Enum3Enum::Value3));
        assert_eq!(Enum3Enum::try_from(3), Ok(Enum3Enum::default()));
        assert_eq!(Enum3Enum::try_from(2), Ok(Enum3Enum::Value2));
        assert_eq!(Enum3Enum::try_from(1), Ok(Enum3Enum::Value1));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum3Enum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum3_enum() {
        let result: Result<Enum3Enum, ()> = 3u8.try_into();
        assert_eq!(result, Ok(Enum3Enum::Value3));
        let result: Result<Enum3Enum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum3Enum::Value2));
        let result: Result<Enum3Enum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum3Enum::Value1));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum3Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[tokio::test]
    async fn test_func0() {
        let test_object = EnumInterface::default();
        let result = test_object.func0(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func1() {
        let test_object = EnumInterface::default();
        let result = test_object.func1(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func2() {
        let test_object = EnumInterface::default();
        let result = test_object.func2(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func3() {
        let test_object = EnumInterface::default();
        let result = test_object.func3(Default::default()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_prop0() {
        let test_object = EnumInterface::default();
        let default_value: Enum0Enum = Default::default();
        test_object.set_prop0(default_value);
        assert_eq!(test_object.prop0(), default_value);
    }

    #[test]
    fn test_prop1() {
        let test_object = EnumInterface::default();
        let default_value: Enum1Enum = Default::default();
        test_object.set_prop1(default_value);
        assert_eq!(test_object.prop1(), default_value);
    }

    #[test]
    fn test_prop2() {
        let test_object = EnumInterface::default();
        let default_value: Enum2Enum = Default::default();
        test_object.set_prop2(default_value);
        assert_eq!(test_object.prop2(), default_value);
    }

    #[test]
    fn test_prop3() {
        let test_object = EnumInterface::default();
        let default_value: Enum3Enum = Default::default();
        test_object.set_prop3(default_value);
        assert_eq!(test_object.prop3(), default_value);
    }

    #[test]
    fn test_sig0() {
        let test_object = EnumInterface::default();
        let mut rx = test_object.publisher().sig0.subscribe();
        let default_value_param0: Enum0Enum = Default::default();
        let _ = test_object.publisher().sig0.send((default_value_param0.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param0);
    }

    #[test]
    fn test_sig1() {
        let test_object = EnumInterface::default();
        let mut rx = test_object.publisher().sig1.subscribe();
        let default_value_param1: Enum1Enum = Default::default();
        let _ = test_object.publisher().sig1.send((default_value_param1.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param1);
    }

    #[test]
    fn test_sig2() {
        let test_object = EnumInterface::default();
        let mut rx = test_object.publisher().sig2.subscribe();
        let default_value_param2: Enum2Enum = Default::default();
        let _ = test_object.publisher().sig2.send((default_value_param2.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param2);
    }

    #[test]
    fn test_sig3() {
        let test_object = EnumInterface::default();
        let mut rx = test_object.publisher().sig3.subscribe();
        let default_value_param3: Enum3Enum = Default::default();
        let _ = test_object.publisher().sig3.send((default_value_param3.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param3);
    }
}
