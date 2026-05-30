mod olink_common;
#[allow(unused_imports)]
use testbed1::api::data_structs::*;
use testbed1::api::struct_array2_interface::StructArray2InterfaceTrait;
use testbed1::implementation::struct_array2_interface::StructArray2Interface;
use testbed1::olink::struct_array2_interface_client::StructArray2InterfaceOlinkClient;
use testbed1::olink::struct_array2_interface_service::StructArray2InterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_struct_array2_interface() -> (Arc<StructArray2InterfaceOlinkClient>, Arc<StructArray2Interface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(StructArray2Interface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(StructArray2InterfaceOlinkService::new(impl_.clone() as Arc<dyn StructArray2InterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(StructArray2InterfaceOlinkClient::default());
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
        let (client, _impl, _loopback) = setup_struct_array2_interface();
        let result = client.func_bool(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_int() {
        let (client, _impl, _loopback) = setup_struct_array2_interface();
        let result = client.func_int(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_float() {
        let (client, _impl, _loopback) = setup_struct_array2_interface();
        let result = client.func_float(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_string() {
        let (client, _impl, _loopback) = setup_struct_array2_interface();
        let result = client.func_string(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_enum() {
        let (client, _impl, _loopback) = setup_struct_array2_interface();
        let result = client.func_enum(&Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop_bool() {
        let (client, impl_, _loopback) = setup_struct_array2_interface();
        let test_value: StructBoolWithArray = Default::default();
        client.set_prop_bool(&test_value.clone());
        assert_eq!(impl_.prop_bool(), test_value);
    }

    #[test]
    fn test_olink_set_prop_int() {
        let (client, impl_, _loopback) = setup_struct_array2_interface();
        let test_value: StructIntWithArray = Default::default();
        client.set_prop_int(&test_value.clone());
        assert_eq!(impl_.prop_int(), test_value);
    }

    #[test]
    fn test_olink_set_prop_float() {
        let (client, impl_, _loopback) = setup_struct_array2_interface();
        let test_value: StructFloatWithArray = Default::default();
        client.set_prop_float(&test_value.clone());
        assert_eq!(impl_.prop_float(), test_value);
    }

    #[test]
    fn test_olink_set_prop_string() {
        let (client, impl_, _loopback) = setup_struct_array2_interface();
        let test_value: StructStringWithArray = Default::default();
        client.set_prop_string(&test_value.clone());
        assert_eq!(impl_.prop_string(), test_value);
    }

    #[test]
    fn test_olink_set_prop_enum() {
        let (client, impl_, _loopback) = setup_struct_array2_interface();
        let test_value: StructEnumWithArray = Default::default();
        client.set_prop_enum(&test_value.clone());
        assert_eq!(impl_.prop_enum(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop_bool_notify() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let rx = client.publisher().prop_bool_changed.subscribe();
        let expected: StructBoolWithArray = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructArray2Interface/propBool", test_value);
        assert_eq!(client.prop_bool(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_int_notify() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let rx = client.publisher().prop_int_changed.subscribe();
        let expected: StructIntWithArray = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructArray2Interface/propInt", test_value);
        assert_eq!(client.prop_int(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_float_notify() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let rx = client.publisher().prop_float_changed.subscribe();
        let expected: StructFloatWithArray = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructArray2Interface/propFloat", test_value);
        assert_eq!(client.prop_float(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_string_notify() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let rx = client.publisher().prop_string_changed.subscribe();
        let expected: StructStringWithArray = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructArray2Interface/propString", test_value);
        assert_eq!(client.prop_string(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_enum_notify() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let rx = client.publisher().prop_enum_changed.subscribe();
        let expected: StructEnumWithArray = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed1.StructArray2Interface/propEnum", test_value);
        assert_eq!(client.prop_enum(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_sig_bool() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let mut rx = client.publisher().sig_bool.subscribe();
        loopback.remote_node.notify_signal("testbed1.StructArray2Interface/sigBool", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_int() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let mut rx = client.publisher().sig_int.subscribe();
        loopback.remote_node.notify_signal("testbed1.StructArray2Interface/sigInt", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_float() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let mut rx = client.publisher().sig_float.subscribe();
        loopback.remote_node.notify_signal("testbed1.StructArray2Interface/sigFloat", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_string() {
        let (client, _impl, loopback) = setup_struct_array2_interface();
        let mut rx = client.publisher().sig_string.subscribe();
        loopback.remote_node.notify_signal("testbed1.StructArray2Interface/sigString", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }
}
