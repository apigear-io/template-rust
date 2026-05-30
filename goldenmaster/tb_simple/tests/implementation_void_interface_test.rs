use tb_simple::api::void_interface::VoidInterfaceTrait;
use tb_simple::implementation::void_interface::VoidInterface;

/// tests for VoidInterface
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_func_void() {
        let test_object = VoidInterface::default();
        let result = test_object.func_void().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_sig_void() {
        let test_object = VoidInterface::default();
        let mut rx = test_object.publisher().sig_void.subscribe();
        let _ = test_object.publisher().sig_void.send(());
        assert!(rx.try_recv().is_ok());
    }
}
