mod olink_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
use testbed2::implementation::nested_struct1_interface::NestedStruct1Interface;
use testbed2::olink::nested_struct1_interface_client::NestedStruct1InterfaceOlinkClient;
use testbed2::olink::nested_struct1_interface_service::NestedStruct1InterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_nested_struct1_interface() -> (Arc<NestedStruct1InterfaceOlinkClient>, Arc<NestedStruct1Interface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(NestedStruct1Interface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(NestedStruct1InterfaceOlinkService::new(impl_.clone() as Arc<dyn NestedStruct1InterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(NestedStruct1InterfaceOlinkClient::default());
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
    async fn test_olink_func_no_return_value() {
        let (client, _impl, _loopback) = setup_nested_struct1_interface();
        let result = client.func_no_return_value(&Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_no_params() {
        let (client, _impl, _loopback) = setup_nested_struct1_interface();
        let result = client.func_no_params().await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func1() {
        let (client, _impl, _loopback) = setup_nested_struct1_interface();
        let result = client.func1(&Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop1() {
        let (client, impl_, _loopback) = setup_nested_struct1_interface();
        let test_value: NestedStruct1 = Default::default();
        client.set_prop1(&test_value.clone());
        assert_eq!(impl_.prop1(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop1_notify() {
        let (client, _impl, loopback) = setup_nested_struct1_interface();
        let rx = client.publisher().prop1_changed.subscribe();
        let expected: NestedStruct1 = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("testbed2.NestedStruct1Interface/prop1", test_value);
        assert_eq!(client.prop1(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_sig1() {
        let (client, _impl, loopback) = setup_nested_struct1_interface();
        let mut rx = client.publisher().sig1.subscribe();
        loopback.remote_node.notify_signal("testbed2.NestedStruct1Interface/sig1", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }
}
