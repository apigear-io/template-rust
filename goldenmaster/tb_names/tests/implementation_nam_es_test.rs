#[allow(unused_imports)]
use tb_names::api::data_structs::*;
use tb_names::api::nam_es::NamEsTrait;
use tb_names::implementation::nam_es::NamEs;

/// tests for NamEs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_enum_with_under_scores_enum() {
        assert_eq!(Enum_With_Under_scoresEnum::try_from(0), Ok(Enum_With_Under_scoresEnum::First_Value));
        assert_eq!(Enum_With_Under_scoresEnum::try_from(0), Ok(Enum_With_Under_scoresEnum::default()));
        assert_eq!(Enum_With_Under_scoresEnum::try_from(1), Ok(Enum_With_Under_scoresEnum::Second_value));
        assert_eq!(Enum_With_Under_scoresEnum::try_from(2), Ok(Enum_With_Under_scoresEnum::Third_Value));
        // test error case assuming 254 is not defined in IDL
        assert_eq!(Enum_With_Under_scoresEnum::try_from(254), Err(()));
    }

    #[test]
    fn test_from_enum_with_under_scores_enum() {
        let result: Result<Enum_With_Under_scoresEnum, ()> = 0u8.try_into();
        assert_eq!(result, Ok(Enum_With_Under_scoresEnum::First_Value));
        let result: Result<Enum_With_Under_scoresEnum, ()> = 1u8.try_into();
        assert_eq!(result, Ok(Enum_With_Under_scoresEnum::Second_value));
        let result: Result<Enum_With_Under_scoresEnum, ()> = 2u8.try_into();
        assert_eq!(result, Ok(Enum_With_Under_scoresEnum::Third_Value));
        // test error case assuming 254 is not defined in IDL
        let result: Result<Enum_With_Under_scoresEnum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
    }

    #[tokio::test]
    async fn test_some_function() {
        let test_object = NamEs::default();
        let result = test_object.some_function(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_some_function2() {
        let test_object = NamEs::default();
        let result = test_object.some_function2(Default::default()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_switch() {
        let test_object = NamEs::default();
        let default_value: bool = Default::default();
        test_object.set_switch(default_value);
        assert_eq!(test_object.switch(), default_value);
    }

    #[test]
    fn test_some_property() {
        let test_object = NamEs::default();
        let default_value: i32 = Default::default();
        test_object.set_some_property(default_value);
        assert_eq!(test_object.some_property(), default_value);
    }

    #[test]
    fn test_some_poperty2() {
        let test_object = NamEs::default();
        let default_value: i32 = Default::default();
        test_object.set_some_poperty2(default_value);
        assert_eq!(test_object.some_poperty2(), default_value);
    }

    #[test]
    fn test_enum_property() {
        let test_object = NamEs::default();
        let default_value: Enum_With_Under_scoresEnum = Default::default();
        test_object.set_enum_property(default_value);
        assert_eq!(test_object.enum_property(), default_value);
    }

    #[test]
    fn test_some_signal() {
        let test_object = NamEs::default();
        let mut rx = test_object.publisher().some_signal.subscribe();
        let default_value_some_param: bool = Default::default();
        let _ = test_object.publisher().some_signal.send((default_value_some_param.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_some_param);
    }

    #[test]
    fn test_some_signal2() {
        let test_object = NamEs::default();
        let mut rx = test_object.publisher().some_signal2.subscribe();
        let default_value_some_param: bool = Default::default();
        let _ = test_object.publisher().some_signal2.send((default_value_some_param.clone(),));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, default_value_some_param);
    }
}
