mod olink_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
use testbed2::implementation::nested_struct3_interface::NestedStruct3Interface;
use testbed2::olink::nested_struct3_interface_client::NestedStruct3InterfaceOlinkClient;
use testbed2::olink::nested_struct3_interface_service::NestedStruct3InterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_nested_struct3_interface() -> (Arc<NestedStruct3InterfaceOlinkClient>, Arc<NestedStruct3Interface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(NestedStruct3Interface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(NestedStruct3InterfaceOlinkService::new(impl_.clone() as Arc<dyn NestedStruct3InterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(NestedStruct3InterfaceOlinkClient::default());
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
    async fn test_olink_func1() {
        let (client, _impl, _loopback) = setup_nested_struct3_interface();
        let result = client.func1(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func2() {
        let (client, _impl, _loopback) = setup_nested_struct3_interface();
        let result = client.func2(&Default::default(), &Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func3() {
        let (client, _impl, _loopback) = setup_nested_struct3_interface();
        let result = client.func3(&Default::default(), &Default::default(), &Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop1() {
        let (client, impl_, _loopback) = setup_nested_struct3_interface();
        let test_value: NestedStruct1 = Default::default();
        client.set_prop1(&test_value.clone());
        assert_eq!(impl_.prop1(), test_value);
    }

    #[test]
    fn test_olink_set_prop2() {
        let (client, impl_, _loopback) = setup_nested_struct3_interface();
        let test_value: NestedStruct2 = Default::default();
        client.set_prop2(&test_value.clone());
        assert_eq!(impl_.prop2(), test_value);
    }

    #[test]
    fn test_olink_set_prop3() {
        let (client, impl_, _loopback) = setup_nested_struct3_interface();
        let test_value: NestedStruct3 = Default::default();
        client.set_prop3(&test_value.clone());
        assert_eq!(impl_.prop3(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop1_notify() {
        let (client, _impl, loopback) = setup_nested_struct3_interface();
        let rx = client.publisher().prop1_changed.subscribe();
        let expected: NestedStruct1 = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed2.NestedStruct3Interface/prop1", test_value);
        assert_eq!(client.prop1(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop2_notify() {
        let (client, _impl, loopback) = setup_nested_struct3_interface();
        let rx = client.publisher().prop2_changed.subscribe();
        let expected: NestedStruct2 = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed2.NestedStruct3Interface/prop2", test_value);
        assert_eq!(client.prop2(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop3_notify() {
        let (client, _impl, loopback) = setup_nested_struct3_interface();
        let rx = client.publisher().prop3_changed.subscribe();
        let expected: NestedStruct3 = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed2.NestedStruct3Interface/prop3", test_value);
        assert_eq!(client.prop3(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_sig1() {
        let (client, _impl, loopback) = setup_nested_struct3_interface();
        let mut rx = client.publisher().sig1.subscribe();
        loopback.remote_node.notify_signal("testbed2.NestedStruct3Interface/sig1", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig2() {
        let (client, _impl, loopback) = setup_nested_struct3_interface();
        let mut rx = client.publisher().sig2.subscribe();
        loopback.remote_node.notify_signal("testbed2.NestedStruct3Interface/sig2", json!([null, null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
        let _signal_param_1 = received.1.clone();
    }

    #[test]
    fn test_olink_sig3() {
        let (client, _impl, loopback) = setup_nested_struct3_interface();
        let mut rx = client.publisher().sig3.subscribe();
        loopback.remote_node.notify_signal("testbed2.NestedStruct3Interface/sig3", json!([null, null, null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
        let _signal_param_1 = received.1.clone();
        let _signal_param_2 = received.2.clone();
    }
}
