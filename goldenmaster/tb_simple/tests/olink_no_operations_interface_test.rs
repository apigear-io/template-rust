mod olink_common;
use tb_simple::api::no_operations_interface::NoOperationsInterfaceTrait;
use tb_simple::implementation::no_operations_interface::NoOperationsInterface;
use tb_simple::olink::no_operations_interface_client::NoOperationsInterfaceOlinkClient;
use tb_simple::olink::no_operations_interface_service::NoOperationsInterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_no_operations_interface() -> (Arc<NoOperationsInterfaceOlinkClient>, Arc<NoOperationsInterface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(NoOperationsInterface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(NoOperationsInterfaceOlinkService::new(impl_.clone() as Arc<dyn NoOperationsInterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(NoOperationsInterfaceOlinkClient::default());
    client.set_node(loopback.client_node.clone());
    let sink: Arc<dyn ObjectSink> = client.clone();
    loopback.client_node.link_remote(&sink);

    (client, impl_, loopback)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop_bool() {
        let (client, impl_, _loopback) = setup_no_operations_interface();
        let test_value: bool = true;
        client.set_prop_bool(test_value);
        assert_eq!(impl_.prop_bool(), test_value);
    }

    #[test]
    fn test_olink_set_prop_int() {
        let (client, impl_, _loopback) = setup_no_operations_interface();
        let test_value: i32 = 1i32;
        client.set_prop_int(test_value);
        assert_eq!(impl_.prop_int(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop_bool_notify() {
        let (client, _impl, loopback) = setup_no_operations_interface();
        let rx = client.publisher().prop_bool_changed.subscribe();
        let test_value = json!(true);
        let expected: bool = true;
        loopback.remote_node.notify_property_change("tb.simple.NoOperationsInterface/propBool", test_value);
        assert_eq!(client.prop_bool(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_int_notify() {
        let (client, _impl, loopback) = setup_no_operations_interface();
        let rx = client.publisher().prop_int_changed.subscribe();
        let test_value = json!(1);
        let expected: i32 = 1i32;
        loopback.remote_node.notify_property_change("tb.simple.NoOperationsInterface/propInt", test_value);
        assert_eq!(client.prop_int(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_sig_void() {
        let (client, _impl, loopback) = setup_no_operations_interface();
        let mut rx = client.publisher().sig_void.subscribe();
        loopback.remote_node.notify_signal("tb.simple.NoOperationsInterface/sigVoid", json!([]));
        assert!(rx.try_recv().is_ok());
    }

    #[test]
    fn test_olink_sig_bool() {
        let (client, _impl, loopback) = setup_no_operations_interface();
        let mut rx = client.publisher().sig_bool.subscribe();
        loopback.remote_node.notify_signal("tb.simple.NoOperationsInterface/sigBool", json!([true]));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, true);
    }
}
