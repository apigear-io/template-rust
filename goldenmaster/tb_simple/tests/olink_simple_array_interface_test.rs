mod olink_common;
use tb_simple::api::simple_array_interface::SimpleArrayInterfaceTrait;
use tb_simple::implementation::simple_array_interface::SimpleArrayInterface;
use tb_simple::olink::simple_array_interface_client::SimpleArrayInterfaceOlinkClient;
use tb_simple::olink::simple_array_interface_service::SimpleArrayInterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_simple_array_interface() -> (Arc<SimpleArrayInterfaceOlinkClient>, Arc<SimpleArrayInterface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(SimpleArrayInterface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(SimpleArrayInterfaceOlinkService::new(impl_.clone() as Arc<dyn SimpleArrayInterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(SimpleArrayInterfaceOlinkClient::default());
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
        let (client, _impl, _loopback) = setup_simple_array_interface();
        let result = client.func_bool(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_int() {
        let (client, _impl, _loopback) = setup_simple_array_interface();
        let result = client.func_int(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_int32() {
        let (client, _impl, _loopback) = setup_simple_array_interface();
        let result = client.func_int32(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_int64() {
        let (client, _impl, _loopback) = setup_simple_array_interface();
        let result = client.func_int64(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_float() {
        let (client, _impl, _loopback) = setup_simple_array_interface();
        let result = client.func_float(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_float32() {
        let (client, _impl, _loopback) = setup_simple_array_interface();
        let result = client.func_float32(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_float64() {
        let (client, _impl, _loopback) = setup_simple_array_interface();
        let result = client.func_float64(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_string() {
        let (client, _impl, _loopback) = setup_simple_array_interface();
        let result = client.func_string(Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop_bool() {
        let (client, impl_, _loopback) = setup_simple_array_interface();
        let test_value: Vec<bool> = vec![Default::default()];
        client.set_prop_bool(test_value.as_slice());
        assert_eq!(impl_.prop_bool(), test_value);
    }

    #[test]
    fn test_olink_set_prop_int() {
        let (client, impl_, _loopback) = setup_simple_array_interface();
        let test_value: Vec<i32> = vec![Default::default()];
        client.set_prop_int(test_value.as_slice());
        assert_eq!(impl_.prop_int(), test_value);
    }

    #[test]
    fn test_olink_set_prop_int32() {
        let (client, impl_, _loopback) = setup_simple_array_interface();
        let test_value: Vec<i32> = vec![Default::default()];
        client.set_prop_int32(test_value.as_slice());
        assert_eq!(impl_.prop_int32(), test_value);
    }

    #[test]
    fn test_olink_set_prop_int64() {
        let (client, impl_, _loopback) = setup_simple_array_interface();
        let test_value: Vec<i64> = vec![Default::default()];
        client.set_prop_int64(test_value.as_slice());
        assert_eq!(impl_.prop_int64(), test_value);
    }

    #[test]
    fn test_olink_set_prop_float() {
        let (client, impl_, _loopback) = setup_simple_array_interface();
        let test_value: Vec<f32> = vec![Default::default()];
        client.set_prop_float(test_value.as_slice());
        assert_eq!(impl_.prop_float(), test_value);
    }

    #[test]
    fn test_olink_set_prop_float32() {
        let (client, impl_, _loopback) = setup_simple_array_interface();
        let test_value: Vec<f32> = vec![Default::default()];
        client.set_prop_float32(test_value.as_slice());
        assert_eq!(impl_.prop_float32(), test_value);
    }

    #[test]
    fn test_olink_set_prop_float64() {
        let (client, impl_, _loopback) = setup_simple_array_interface();
        let test_value: Vec<f64> = vec![Default::default()];
        client.set_prop_float64(test_value.as_slice());
        assert_eq!(impl_.prop_float64(), test_value);
    }

    #[test]
    fn test_olink_set_prop_string() {
        let (client, impl_, _loopback) = setup_simple_array_interface();
        let test_value: Vec<String> = vec![Default::default()];
        client.set_prop_string(test_value.as_slice());
        assert_eq!(impl_.prop_string(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop_bool_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_bool_changed.subscribe();
        let test_value = json!([]);
        let expected: Vec<bool> = vec![];
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propBool", test_value);
        assert_eq!(client.prop_bool(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_int_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_int_changed.subscribe();
        let test_value = json!([]);
        let expected: Vec<i32> = vec![];
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propInt", test_value);
        assert_eq!(client.prop_int(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_int32_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_int32_changed.subscribe();
        let test_value = json!([]);
        let expected: Vec<i32> = vec![];
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propInt32", test_value);
        assert_eq!(client.prop_int32(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_int64_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_int64_changed.subscribe();
        let test_value = json!([]);
        let expected: Vec<i64> = vec![];
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propInt64", test_value);
        assert_eq!(client.prop_int64(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_float_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_float_changed.subscribe();
        let test_value = json!([]);
        let expected: Vec<f32> = vec![];
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propFloat", test_value);
        assert_eq!(client.prop_float(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_float32_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_float32_changed.subscribe();
        let test_value = json!([]);
        let expected: Vec<f32> = vec![];
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propFloat32", test_value);
        assert_eq!(client.prop_float32(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_float64_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_float64_changed.subscribe();
        let test_value = json!([]);
        let expected: Vec<f64> = vec![];
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propFloat64", test_value);
        assert_eq!(client.prop_float64(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_string_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_string_changed.subscribe();
        let test_value = json!([]);
        let expected: Vec<String> = vec![];
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propString", test_value);
        assert_eq!(client.prop_string(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_read_only_string_notify() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let rx = client.publisher().prop_read_only_string_changed.subscribe();
        let test_value = json!("test");
        let expected: String = String::from("test");
        loopback.remote_node.notify_property_change("tb.simple.SimpleArrayInterface/propReadOnlyString", test_value);
        assert_eq!(client.prop_read_only_string(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_sig_bool() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let mut rx = client.publisher().sig_bool.subscribe();
        loopback.remote_node.notify_signal("tb.simple.SimpleArrayInterface/sigBool", json!([[]]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_int() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let mut rx = client.publisher().sig_int.subscribe();
        loopback.remote_node.notify_signal("tb.simple.SimpleArrayInterface/sigInt", json!([[]]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_int32() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let mut rx = client.publisher().sig_int32.subscribe();
        loopback.remote_node.notify_signal("tb.simple.SimpleArrayInterface/sigInt32", json!([[]]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_int64() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let mut rx = client.publisher().sig_int64.subscribe();
        loopback.remote_node.notify_signal("tb.simple.SimpleArrayInterface/sigInt64", json!([[]]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_float() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let mut rx = client.publisher().sig_float.subscribe();
        loopback.remote_node.notify_signal("tb.simple.SimpleArrayInterface/sigFloat", json!([[]]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_float32() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let mut rx = client.publisher().sig_float32.subscribe();
        loopback.remote_node.notify_signal("tb.simple.SimpleArrayInterface/sigFloat32", json!([[]]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_float64() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let mut rx = client.publisher().sig_float64.subscribe();
        loopback.remote_node.notify_signal("tb.simple.SimpleArrayInterface/sigFloat64", json!([[]]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig_string() {
        let (client, _impl, loopback) = setup_simple_array_interface();
        let mut rx = client.publisher().sig_string.subscribe();
        loopback.remote_node.notify_signal("tb.simple.SimpleArrayInterface/sigString", json!([[]]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }
}
