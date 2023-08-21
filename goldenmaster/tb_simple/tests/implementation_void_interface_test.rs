use tb_simple::api::void_interface::VoidInterfaceTrait;
use tb_simple::implementation::void_interface::VoidInterface;

/// tests for VoidInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_func_void() {
        let mut test_object: VoidInterface = Default::default();
        test_object.func_void(
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_func_void_async() {
        let mut test_object: VoidInterface = Default::default();
        let _ = test_object.func_void_async(
        );
    }
}
