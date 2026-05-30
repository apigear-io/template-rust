mod olink_common;
#[allow(unused_imports)]
use tb_names::api::data_structs::*;
use tb_names::api::nam_es::NamEsTrait;
use tb_names::implementation::nam_es::NamEs;
use tb_names::olink::nam_es_client::NamEsOlinkClient;
use tb_names::olink::nam_es_service::NamEsOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_nam_es() -> (Arc<NamEsOlinkClient>, Arc<NamEs>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(NamEs::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(NamEsOlinkService::new(impl_.clone() as Arc<dyn NamEsTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(NamEsOlinkClient::default());
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
    async fn test_olink_some_function() {
        let (client, _impl, _loopback) = setup_nam_es();
        let result = client.some_function(Default::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_some_function2() {
        let (client, _impl, _loopback) = setup_nam_es();
        let result = client.some_function2(Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_switch() {
        let (client, impl_, _loopback) = setup_nam_es();
        let test_value: bool = true;
        client.set_switch(test_value);
        assert_eq!(impl_.switch(), test_value);
    }

    #[test]
    fn test_olink_set_some_property() {
        let (client, impl_, _loopback) = setup_nam_es();
        let test_value: i32 = 1i32;
        client.set_some_property(test_value);
        assert_eq!(impl_.some_property(), test_value);
    }

    #[test]
    fn test_olink_set_some_poperty2() {
        let (client, impl_, _loopback) = setup_nam_es();
        let test_value: i32 = 1i32;
        client.set_some_poperty2(test_value);
        assert_eq!(impl_.some_poperty2(), test_value);
    }

    #[test]
    fn test_olink_set_enum_property() {
        let (client, impl_, _loopback) = setup_nam_es();
        let test_value: Enum_With_Under_scoresEnum = Default::default();
        client.set_enum_property(test_value);
        assert_eq!(impl_.enum_property(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_switch_notify() {
        let (client, _impl, loopback) = setup_nam_es();
        let rx = client.publisher().switch_changed.subscribe();
        let test_value = json!(true);
        let expected: bool = true;
        loopback.remote_node.notify_property_change("tb.names.Nam_Es/Switch", test_value);
        assert_eq!(client.switch(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_some_property_notify() {
        let (client, _impl, loopback) = setup_nam_es();
        let rx = client.publisher().some_property_changed.subscribe();
        let test_value = json!(1);
        let expected: i32 = 1i32;
        loopback.remote_node.notify_property_change("tb.names.Nam_Es/SOME_PROPERTY", test_value);
        assert_eq!(client.some_property(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_some_poperty2_notify() {
        let (client, _impl, loopback) = setup_nam_es();
        let rx = client.publisher().some_poperty2_changed.subscribe();
        let test_value = json!(1);
        let expected: i32 = 1i32;
        loopback.remote_node.notify_property_change("tb.names.Nam_Es/Some_Poperty2", test_value);
        assert_eq!(client.some_poperty2(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_enum_property_notify() {
        let (client, _impl, loopback) = setup_nam_es();
        let rx = client.publisher().enum_property_changed.subscribe();
        let expected: Enum_With_Under_scoresEnum = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        loopback.remote_node.notify_property_change("tb.names.Nam_Es/enum_property", test_value);
        assert_eq!(client.enum_property(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    // -- Signals --

    #[test]
    fn test_olink_some_signal() {
        let (client, _impl, loopback) = setup_nam_es();
        let mut rx = client.publisher().some_signal.subscribe();
        loopback.remote_node.notify_signal("tb.names.Nam_Es/SOME_SIGNAL", json!([true]));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, true);
    }

    #[test]
    fn test_olink_some_signal2() {
        let (client, _impl, loopback) = setup_nam_es();
        let mut rx = client.publisher().some_signal2.subscribe();
        loopback.remote_node.notify_signal("tb.names.Nam_Es/Some_Signal2", json!([true]));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, true);
    }
}
