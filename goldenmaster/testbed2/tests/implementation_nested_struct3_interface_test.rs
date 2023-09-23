use signals2::*;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
use testbed2::implementation::nested_struct3_interface::NestedStruct3Interface;

/// tests for NestedStruct3Interface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_enum1_enum() {
        assert_eq!(Enum1Enum::try_from(1), Ok(Enum1Enum::Value1));
        assert_eq!(Enum1Enum::try_from(1), Ok(Enum1Enum::default()));
        assert_eq!(Enum1Enum::try_from(2), Ok(Enum1Enum::Value2));
        assert_eq!(Enum1Enum::try_from(3), Ok(Enum1Enum::Value3));
        assert_eq!(Enum1Enum::try_from(4), Ok(Enum1Enum::Value4));
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
        let result: Result<Enum1Enum, ()> = 4u8.try_into();
        assert_eq!(result, Ok(Enum1Enum::Value4));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum1Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_to_enum2_enum() {
        assert_eq!(Enum2Enum::try_from(1), Ok(Enum2Enum::Value1));
        assert_eq!(Enum2Enum::try_from(1), Ok(Enum2Enum::default()));
        assert_eq!(Enum2Enum::try_from(2), Ok(Enum2Enum::Value2));
        assert_eq!(Enum2Enum::try_from(3), Ok(Enum2Enum::Value3));
        assert_eq!(Enum2Enum::try_from(4), Ok(Enum2Enum::Value4));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum2Enum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum2_enum() {
        let result: Result<Enum2Enum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value1));
        let result: Result<Enum2Enum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value2));
        let result: Result<Enum2Enum, ()> = 3u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value3));
        let result: Result<Enum2Enum, ()> = 4u8.try_into();
        assert_eq!(result, Ok(Enum2Enum::Value4));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum2Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_to_enum3_enum() {
        assert_eq!(Enum3Enum::try_from(1), Ok(Enum3Enum::Value1));
        assert_eq!(Enum3Enum::try_from(1), Ok(Enum3Enum::default()));
        assert_eq!(Enum3Enum::try_from(2), Ok(Enum3Enum::Value2));
        assert_eq!(Enum3Enum::try_from(3), Ok(Enum3Enum::Value3));
        assert_eq!(Enum3Enum::try_from(4), Ok(Enum3Enum::Value4));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum3Enum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum3_enum() {
        let result: Result<Enum3Enum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum3Enum::Value1));
        let result: Result<Enum3Enum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum3Enum::Value2));
        let result: Result<Enum3Enum, ()> = 3u8.try_into();
        assert_eq!(result, Ok(Enum3Enum::Value3));
        let result: Result<Enum3Enum, ()> = 4u8.try_into();
        assert_eq!(result, Ok(Enum3Enum::Value4));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum3Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_func1() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func1(&Default::default());
    }

    #[test]
    fn test_func1_async() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let _ = test_object.func1_async(&Default::default());
    }

    #[test]
    fn test_func2() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func2(&Default::default(), &Default::default());
    }

    #[test]
    fn test_func2_async() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let _ = test_object.func2_async(&Default::default(), &Default::default());
    }

    #[test]
    fn test_func3() {
        let mut test_object: NestedStruct3Interface = Default::default();
        test_object.func3(&Default::default(), &Default::default(), &Default::default());
    }

    #[test]
    fn test_func3_async() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let _ = test_object.func3_async(&Default::default(), &Default::default(), &Default::default());
    }

    #[test]
    fn test_prop1() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let default_value: NestedStruct1 = Default::default();
        test_object.set_prop1(&default_value);
        assert_eq!(test_object.prop1().clone(), default_value);
    }

    #[test]
    fn test_prop2() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let default_value: NestedStruct2 = Default::default();
        test_object.set_prop2(&default_value);
        assert_eq!(test_object.prop2().clone(), default_value);
    }

    #[test]
    fn test_prop3() {
        let mut test_object: NestedStruct3Interface = Default::default();
        let default_value: NestedStruct3 = Default::default();
        test_object.set_prop3(&default_value);
        assert_eq!(test_object.prop3().clone(), default_value);
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig1() {
        let mut test_object: NestedStruct3Interface = Default::default();

        test_object._get_signal_handler().sig1.connect(move |param1| {
            let default_value_param1: NestedStruct1 = Default::default();
            assert_eq!(param1, default_value_param1);
        });

        let default_value_param1: NestedStruct1 = Default::default();
        test_object._get_signal_handler().sig1.emit(
            default_value_param1.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig2() {
        let mut test_object: NestedStruct3Interface = Default::default();

        test_object._get_signal_handler().sig2.connect(move |param1, param2| {
            let default_value_param1: NestedStruct1 = Default::default();
            assert_eq!(param1, default_value_param1);
            let default_value_param2: NestedStruct2 = Default::default();
            assert_eq!(param2, default_value_param2);
        });

        let default_value_param1: NestedStruct1 = Default::default();
        let default_value_param2: NestedStruct2 = Default::default();
        test_object._get_signal_handler().sig2.emit(
            default_value_param1.clone(),
            default_value_param2.clone(),
        );
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig3() {
        let mut test_object: NestedStruct3Interface = Default::default();

        test_object._get_signal_handler().sig3.connect(move |param1, param2, param3| {
            let default_value_param1: NestedStruct1 = Default::default();
            assert_eq!(param1, default_value_param1);
            let default_value_param2: NestedStruct2 = Default::default();
            assert_eq!(param2, default_value_param2);
            let default_value_param3: NestedStruct3 = Default::default();
            assert_eq!(param3, default_value_param3);
        });

        let default_value_param1: NestedStruct1 = Default::default();
        let default_value_param2: NestedStruct2 = Default::default();
        let default_value_param3: NestedStruct3 = Default::default();
        test_object._get_signal_handler().sig3.emit(
            default_value_param1.clone(),
            default_value_param2.clone(),
            default_value_param3.clone(),
        );
    }
}
