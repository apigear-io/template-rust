mod olink_common;
#[allow(unused_imports)]
use tb_enum::api::data_structs::*;
use tb_enum::api::enum_interface::EnumInterfaceTrait;
use tb_enum::implementation::enum_interface::EnumInterface;
use tb_enum::olink::enum_interface_client::EnumInterfaceOlinkClient;
use tb_enum::olink::enum_interface_service::EnumInterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_enum_interface() -> (Arc<EnumInterfaceOlinkClient>, Arc<EnumInterface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(EnumInterface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(EnumInterfaceOlinkService::new(impl_.clone() as Arc<dyn EnumInterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(EnumInterfaceOlinkClient::default());
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
    async fn test_olink_func0() {
        let (client, _impl, _loopback) = setup_enum_interface();
        let result = client.func0(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func1() {
        let (client, _impl, _loopback) = setup_enum_interface();
        let result = client.func1(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func2() {
        let (client, _impl, _loopback) = setup_enum_interface();
        let result = client.func2(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func3() {
        let (client, _impl, _loopback) = setup_enum_interface();
        let result = client.func3(Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop0() {
        let (client, impl_, _loopback) = setup_enum_interface();
        let test_value: Enum0Enum = Default::default();
        client.set_prop0(test_value);
        assert_eq!(impl_.prop0(), test_value);
    }

    #[test]
    fn test_olink_set_prop1() {
        let (client, impl_, _loopback) = setup_enum_interface();
        let test_value: Enum1Enum = Default::default();
        client.set_prop1(test_value);
        assert_eq!(impl_.prop1(), test_value);
    }

    #[test]
    fn test_olink_set_prop2() {
        let (client, impl_, _loopback) = setup_enum_interface();
        let test_value: Enum2Enum = Default::default();
        client.set_prop2(test_value);
        assert_eq!(impl_.prop2(), test_value);
    }

    #[test]
    fn test_olink_set_prop3() {
        let (client, impl_, _loopback) = setup_enum_interface();
        let test_value: Enum3Enum = Default::default();
        client.set_prop3(test_value);
        assert_eq!(impl_.prop3(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop0_notify() {
        let (client, _impl, loopback) = setup_enum_interface();
        let rx = client.publisher().prop0_changed.subscribe();
        let expected: Enum0Enum = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("tb.enum.EnumInterface/prop0", test_value);
        assert_eq!(client.prop0(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop1_notify() {
        let (client, _impl, loopback) = setup_enum_interface();
        let rx = client.publisher().prop1_changed.subscribe();
        let expected: Enum1Enum = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("tb.enum.EnumInterface/prop1", test_value);
        assert_eq!(client.prop1(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop2_notify() {
        let (client, _impl, loopback) = setup_enum_interface();
        let rx = client.publisher().prop2_changed.subscribe();
        let expected: Enum2Enum = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("tb.enum.EnumInterface/prop2", test_value);
        assert_eq!(client.prop2(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop3_notify() {
        let (client, _impl, loopback) = setup_enum_interface();
        let rx = client.publisher().prop3_changed.subscribe();
        let expected: Enum3Enum = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("tb.enum.EnumInterface/prop3", test_value);
        assert_eq!(client.prop3(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_sig0() {
        let (client, _impl, loopback) = setup_enum_interface();
        let mut rx = client.publisher().sig0.subscribe();
        loopback.remote_node.notify_signal("tb.enum.EnumInterface/sig0", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig1() {
        let (client, _impl, loopback) = setup_enum_interface();
        let mut rx = client.publisher().sig1.subscribe();
        loopback.remote_node.notify_signal("tb.enum.EnumInterface/sig1", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig2() {
        let (client, _impl, loopback) = setup_enum_interface();
        let mut rx = client.publisher().sig2.subscribe();
        loopback.remote_node.notify_signal("tb.enum.EnumInterface/sig2", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }

    #[test]
    fn test_olink_sig3() {
        let (client, _impl, loopback) = setup_enum_interface();
        let mut rx = client.publisher().sig3.subscribe();
        loopback.remote_node.notify_signal("tb.enum.EnumInterface/sig3", json!([null]));
        let received = rx.try_recv().unwrap();
        let _signal_param_0 = received.0.clone();
    }
}
