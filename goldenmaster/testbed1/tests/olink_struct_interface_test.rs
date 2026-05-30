mod olink_common;
#[allow(unused_imports)]
use testbed1::api::data_structs::*;
use testbed1::api::struct_interface::StructInterfaceTrait;
use testbed1::implementation::struct_interface::StructInterface;
use testbed1::olink::struct_interface_client::StructInterfaceOlinkClient;
use testbed1::olink::struct_interface_service::StructInterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_struct_interface() -> (Arc<StructInterfaceOlinkClient>, Arc<StructInterface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(StructInterface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(StructInterfaceOlinkService::new(impl_.clone() as Arc<dyn StructInterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(StructInterfaceOlinkClient::default());
    client.set_node(loopback.client_node.clone());
    let sink: Arc<dyn ObjectSink> = client.clone();
    loopback.client_node.link_remote(&sink);

    (client, impl_, loopback)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Operations --

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_bool() {
        let (client, _impl, _loopback) = setup_struct_interface();
        let result = client.func_bool(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_int() {
        let (client, _impl, _loopback) = setup_struct_interface();
        let result = client.func_int(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_float() {
        let (client, _impl, _loopback) = setup_struct_interface();
        let result = client.func_float(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_string() {
        let (client, _impl, _loopback) = setup_struct_interface();
        let result = client.func_string(&Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop_bool() {
        let (client, impl_, _loopback) = setup_struct_interface();
        let test_value: StructBool = Default::default();
        client.set_prop_bool(&test_value.clone());
        assert_eq!(impl_.prop_bool(), test_value);
    }

    #[test]
    fn test_olink_set_prop_int() {
        let (client, impl_, _loopback) = setup_struct_interface();
        let test_value: StructInt = Default::default();
        client.set_prop_int(&test_value.clone());
        assert_eq!(impl_.prop_int(), test_value);
    }

    #[test]
    fn test_olink_set_prop_float() {
        let (client, impl_, _loopback) = setup_struct_interface();
        let test_value: StructFloat = Default::default();
        client.set_prop_float(&test_value.clone());
        assert_eq!(impl_.prop_float(), test_value);
    }

    #[test]
    fn test_olink_set_prop_string() {
        let (client, impl_, _loopback) = setup_struct_interface();
        let test_value: StructString = Default::default();
        client.set_prop_string(&test_value.clone());
        assert_eq!(impl_.prop_string(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop_bool_notify() {
        let (client, _impl, loopback) = setup_struct_interface();
        let rx = client.publisher().prop_bool_changed.subscribe();
        let expected: StructBool = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructInterface/propBool", test_value);
        assert_eq!(client.prop_bool(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_int_notify() {
        let (client, _impl, loopback) = setup_struct_interface();
        let rx = client.publisher().prop_int_changed.subscribe();
        let expected: StructInt = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructInterface/propInt", test_value);
        assert_eq!(client.prop_int(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_float_notify() {
        let (client, _impl, loopback) = setup_struct_interface();
        let rx = client.publisher().prop_float_changed.subscribe();
        let expected: StructFloat = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructInterface/propFloat", test_value);
        assert_eq!(client.prop_float(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_string_notify() {
        let (client, _impl, loopback) = setup_struct_interface();
        let rx = client.publisher().prop_string_changed.subscribe();
        let expected: StructString = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructInterface/propString", test_value);
        assert_eq!(client.prop_string(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_sig_bool() {
        let (client, _impl, loopback) = setup_struct_interface();
        let mut rx = client.publisher().sig_bool.subscribe();
        loopback.remote_node.notify_signal("testbed1.StructInterface/sigBool", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_int() {
        let (client, _impl, loopback) = setup_struct_interface();
        let mut rx = client.publisher().sig_int.subscribe();
        loopback.remote_node.notify_signal("testbed1.StructInterface/sigInt", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_float() {
        let (client, _impl, loopback) = setup_struct_interface();
        let mut rx = client.publisher().sig_float.subscribe();
        loopback.remote_node.notify_signal("testbed1.StructInterface/sigFloat", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_string() {
        let (client, _impl, loopback) = setup_struct_interface();
        let mut rx = client.publisher().sig_string.subscribe();
        loopback.remote_node.notify_signal("testbed1.StructInterface/sigString", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }
}
