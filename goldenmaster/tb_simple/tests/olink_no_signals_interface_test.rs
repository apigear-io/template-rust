mod olink_common;
use tb_simple::api::no_signals_interface::NoSignalsInterfaceTrait;
use tb_simple::implementation::no_signals_interface::NoSignalsInterface;
use tb_simple::olink::no_signals_interface_client::NoSignalsInterfaceOlinkClient;
use tb_simple::olink::no_signals_interface_service::NoSignalsInterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_no_signals_interface() -> (Arc<NoSignalsInterfaceOlinkClient>, Arc<NoSignalsInterface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(NoSignalsInterface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(NoSignalsInterfaceOlinkService::new(impl_.clone() as Arc<dyn NoSignalsInterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(NoSignalsInterfaceOlinkClient::default());
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
    async fn test_olink_func_void() {
        let (client, _impl, _loopback) = setup_no_signals_interface();
        let result = client.func_void().await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_bool() {
        let (client, _impl, _loopback) = setup_no_signals_interface();
        let result = client.func_bool(Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Properties (set via client -> verify on impl) --

    #[test]
    fn test_olink_set_prop_bool() {
        let (client, impl_, _loopback) = setup_no_signals_interface();
        let test_value: bool = true;
        client.set_prop_bool(test_value);
        assert_eq!(impl_.prop_bool(), test_value);
    }

    #[test]
    fn test_olink_set_prop_int() {
        let (client, impl_, _loopback) = setup_no_signals_interface();
        let test_value: i32 = 1i32;
        client.set_prop_int(test_value);
        assert_eq!(impl_.prop_int(), test_value);
    }

    // -- Properties (notify change from remote -> verify on client) --

    #[test]
    fn test_olink_prop_bool_notify() {
        let (client, _impl, loopback) = setup_no_signals_interface();
        let rx = client.publisher().prop_bool_changed.subscribe();
        let test_value = json!(true);
        let expected: bool = true;
        loopback.remote_node.notify_property_change("tb.simple.NoSignalsInterface/propBool", test_value);
        assert_eq!(client.prop_bool(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }

    #[test]
    fn test_olink_prop_int_notify() {
        let (client, _impl, loopback) = setup_no_signals_interface();
        let rx = client.publisher().prop_int_changed.subscribe();
        let test_value = json!(1);
        let expected: i32 = 1i32;
        loopback.remote_node.notify_property_change("tb.simple.NoSignalsInterface/propInt", test_value);
        assert_eq!(client.prop_int(), expected);
        assert!(rx.has_changed().unwrap_or(false));
    }
}
