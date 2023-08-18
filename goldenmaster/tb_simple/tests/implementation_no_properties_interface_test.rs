use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTrait;
use tb_simple::implementation::no_properties_interface::NoPropertiesInterface;

/// tests for NoPropertiesInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func_void() {
        let mut test_object: NoPropertiesInterface = Default::default();
        test_object.func_void(
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_void_async() {
        let mut test_object: NoPropertiesInterface = Default::default();
        test_object.func_void_async(
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_bool() {
        let mut test_object: NoPropertiesInterface = Default::default();
        test_object.func_bool(
        Default::default(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_bool_async() {
        let mut test_object: NoPropertiesInterface = Default::default();
        test_object.func_bool_async(
        Default::default(),
        );
    }
}
