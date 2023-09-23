use signals2::*;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use tb_same2::api::data_structs::*;
use tb_same2::api::same_enum1_interface::SameEnum1InterfaceTrait;
use tb_same2::implementation::same_enum1_interface::SameEnum1Interface;

/// tests for SameEnum1Interface
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

    #[test]
    fn test_func1() {
        let mut test_object: SameEnum1Interface = Default::default();
        test_object.func1(Default::default());
    }

    #[test]
    fn test_func1_async() {
        let mut test_object: SameEnum1Interface = Default::default();
        let _ = test_object.func1_async(Default::default());
    }

    #[test]
    fn test_prop1() {
        let mut test_object: SameEnum1Interface = Default::default();
        let default_value: Enum1Enum = Default::default();
        test_object.set_prop1(default_value);
        assert_eq!(test_object.prop1().clone(), default_value);
    }

    #[rustfmt::skip]
    #[test]
    fn test_sig1() {
        let mut test_object: SameEnum1Interface = Default::default();

        test_object._get_signal_handler().sig1.connect(move |param1| {
            let default_value_param1: Enum1Enum = Default::default();
            assert_eq!(param1, default_value_param1);
        });

        let default_value_param1: Enum1Enum = Default::default();
        test_object._get_signal_handler().sig1.emit(
            default_value_param1.clone(),
        );
    }
}
