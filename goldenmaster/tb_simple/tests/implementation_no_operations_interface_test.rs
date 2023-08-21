use tb_simple::api::no_operations_interface::NoOperationsInterfaceTrait;
use tb_simple::implementation::no_operations_interface::NoOperationsInterface;

/// tests for NoOperationsInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prop_bool() {
        let mut test_object: NoOperationsInterface = Default::default();
        let default_value: bool = Default::default();
        test_object.set_prop_bool(default_value);
        assert_eq!(test_object.prop_bool().clone(), default_value);
    }

    #[test]
    fn test_prop_int() {
        let mut test_object: NoOperationsInterface = Default::default();
        let default_value: i32 = Default::default();
        test_object.set_prop_int(default_value);
        assert_eq!(test_object.prop_int().clone(), default_value);
    }
}
