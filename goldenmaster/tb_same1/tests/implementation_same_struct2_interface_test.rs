#[allow(unused_imports)]
use tb_same1::api::data_structs::*;
use tb_same1::api::same_struct2_interface::SameStruct2InterfaceTrait;
use tb_same1::implementation::same_struct2_interface::SameStruct2Interface;

/// tests for SameStruct2Interface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_enum1_enum() {
        assert_eq!(Enum1Enum::try_from(1), Ok(Enum1Enum::Value1));
        assert_eq!(Enum1Enum::try_from(1), Ok(Enum1Enum::default()));
        assert_eq!(Enum1Enum::try_from(2), Ok(Enum1Enum::Value2));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum1Enum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum1_enum() {
        let result: Result<Enum1Enum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum1Enum::Value1));
        let result: Result<Enum1Enum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum1Enum::Value2));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum1Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_to_enum2_enum() {
        assert_eq!(Enum2Enum::try_from(1), Ok(Enum2Enum::Value1));
        assert_eq!(Enum2Enum::try_from(1), Ok(Enum2Enum::default()));
        assert_eq!(Enum2Enum::try_from(2), Ok(Enum2Enum::Value2));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum2Enum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum2_enum() {
        let result: Result<Enum2Enum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value1));
        let result: Result<Enum2Enum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value2));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum2Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[tokio::test]
    async fn test_func1() {
        let test_object = SameStruct2Interface::default();
        let result = test_object.func1(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_func2() {
        let test_object = SameStruct2Interface::default();
        let result = test_object.func2(&Default::default(), &Default::default()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_prop1() {
        let test_object = SameStruct2Interface::default();
        let default_value: Struct2 = Default::default();
        test_object.set_prop1(&default_value.clone());
        assert_eq!(test_object.prop1(), default_value);
    }

    #[test]
    fn test_prop2() {
        let test_object = SameStruct2Interface::default();
        let default_value: Struct2 = Default::default();
        test_object.set_prop2(&default_value.clone());
        assert_eq!(test_object.prop2(), default_value);
    }

    #[test]
    fn test_sig1() {
        let test_object = SameStruct2Interface::default();
        let mut rx = test_object.publisher().sig1.subscribe();
        let default_value_param1: Struct1 = Default::default();
        let _ = test_object.publisher().sig1.send((default_value_param1.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param1);
    }

    #[test]
    fn test_sig2() {
        let test_object = SameStruct2Interface::default();
        let mut rx = test_object.publisher().sig2.subscribe();
        let default_value_param1: Struct1 = Default::default();
        let default_value_param2: Struct2 = Default::default();
        let _ = test_object.publisher().sig2.send((default_value_param1.clone(), default_value_param2.clone()));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_param1);
        assert_eq!(received.1, default_value_param2);
    }
}
