mod olink_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
use testbed2::implementation::many_param_interface::ManyParamInterface;
use testbed2::olink::many_param_interface_client::ManyParamInterfaceOlinkClient;
use testbed2::olink::many_param_interface_service::ManyParamInterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_many_param_interface() -> (Arc<ManyParamInterfaceOlinkClient>, Arc<ManyParamInterface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(ManyParamInterface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(ManyParamInterfaceOlinkService::new(impl_.clone() as Arc<dyn ManyParamInterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(ManyParamInterfaceOlinkClient::default());
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
        let (client, _impl, _loopback) = setup_many_param_interface();
        let result = client.func1(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func2() {
        let (client, _impl, _loopback) = setup_many_param_interface();
        let result = client.func2(Default::default(), Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func3() {
        let (client, _impl, _loopback) = setup_many_param_interface();
        let result = client.func3(Default::default(), Default::default(), Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func4() {
        let (client, _impl, _loopback) = setup_many_param_interface();
        let result = client.func4(Default::default(), Default::default(), Default::default(), Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop1() {
        let (client, impl_, _loopback) = setup_many_param_interface();
        let test_value: i32 = 1i32;
        client.set_prop1(test_value);
        assert_eq!(impl_.prop1(), test_value);
    }

    #[test]
    fn test_olink_set_prop2() {
        let (client, impl_, _loopback) = setup_many_param_interface();
        let test_value: i32 = 1i32;
        client.set_prop2(test_value);
        assert_eq!(impl_.prop2(), test_value);
    }

    #[test]
    fn test_olink_set_prop3() {
        let (client, impl_, _loopback) = setup_many_param_interface();
        let test_value: i32 = 1i32;
        client.set_prop3(test_value);
        assert_eq!(impl_.prop3(), test_value);
    }

    #[test]
    fn test_olink_set_prop4() {
        let (client, impl_, _loopback) = setup_many_param_interface();
        let test_value: i32 = 1i32;
        client.set_prop4(test_value);
        assert_eq!(impl_.prop4(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop1_notify() {
        let (client, _impl, loopback) = setup_many_param_interface();
        let rx = client.publisher().prop1_changed.subscribe();
        let test_value = json!(1);
        let expected: i32 = 1i32;
        loopback.remote_node.notify_property_change("testbed2.ManyParamInterface/prop1", test_value);
        assert_eq!(client.prop1(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop2_notify() {
        let (client, _impl, loopback) = setup_many_param_interface();
        let rx = client.publisher().prop2_changed.subscribe();
        let test_value = json!(1);
        let expected: i32 = 1i32;
        loopback.remote_node.notify_property_change("testbed2.ManyParamInterface/prop2", test_value);
        assert_eq!(client.prop2(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop3_notify() {
        let (client, _impl, loopback) = setup_many_param_interface();
        let rx = client.publisher().prop3_changed.subscribe();
        let test_value = json!(1);
        let expected: i32 = 1i32;
        loopback.remote_node.notify_property_change("testbed2.ManyParamInterface/prop3", test_value);
        assert_eq!(client.prop3(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop4_notify() {
        let (client, _impl, loopback) = setup_many_param_interface();
        let rx = client.publisher().prop4_changed.subscribe();
        let test_value = json!(1);
        let expected: i32 = 1i32;
        loopback.remote_node.notify_property_change("testbed2.ManyParamInterface/prop4", test_value);
        assert_eq!(client.prop4(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_sig1() {
        let (client, _impl, loopback) = setup_many_param_interface();
        let mut rx = client.publisher().sig1.subscribe();
        loopback.remote_node.notify_signal("testbed2.ManyParamInterface/sig1", json!([1]));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, 1i32);
    }

    #[test]
    fn test_olink_sig2() {
        let (client, _impl, loopback) = setup_many_param_interface();
        let mut rx = client.publisher().sig2.subscribe();
        loopback.remote_node.notify_signal("testbed2.ManyParamInterface/sig2", json!([1, 1]));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, 1i32);
        assert_eq!(received.1, 1i32);
    }

    #[test]
    fn test_olink_sig3() {
        let (client, _impl, loopback) = setup_many_param_interface();
        let mut rx = client.publisher().sig3.subscribe();
        loopback.remote_node.notify_signal("testbed2.ManyParamInterface/sig3", json!([1, 1, 1]));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, 1i32);
        assert_eq!(received.1, 1i32);
        assert_eq!(received.2, 1i32);
    }

    #[test]
    fn test_olink_sig4() {
        let (client, _impl, loopback) = setup_many_param_interface();
        let mut rx = client.publisher().sig4.subscribe();
        loopback.remote_node.notify_signal("testbed2.ManyParamInterface/sig4", json!([1, 1, 1, 1]));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, 1i32);
        assert_eq!(received.1, 1i32);
        assert_eq!(received.2, 1i32);
        assert_eq!(received.3, 1i32);
    }
}
