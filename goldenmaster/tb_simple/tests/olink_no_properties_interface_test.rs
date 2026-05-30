mod olink_common;
use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTrait;
use tb_simple::implementation::no_properties_interface::NoPropertiesInterface;
use tb_simple::olink::no_properties_interface_client::NoPropertiesInterfaceOlinkClient;
use tb_simple::olink::no_properties_interface_service::NoPropertiesInterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_no_properties_interface() -> (Arc<NoPropertiesInterfaceOlinkClient>, Arc<NoPropertiesInterface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(NoPropertiesInterface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(NoPropertiesInterfaceOlinkService::new(impl_.clone() as Arc<dyn NoPropertiesInterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(NoPropertiesInterfaceOlinkClient::default());
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
        let (client, _impl, _loopback) = setup_no_properties_interface();
        let result = client.func_void().await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_func_bool() {
        let (client, _impl, _loopback) = setup_no_properties_interface();
        let result = client.func_bool(Default::default()).await;
        assert!(result.is_ok());
    }

    // -- Signals --

    #[test]
    fn test_olink_sig_void() {
        let (client, _impl, loopback) = setup_no_properties_interface();
        let mut rx = client.publisher().sig_void.subscribe();
        loopback.remote_node.notify_signal("tb.simple.NoPropertiesInterface/sigVoid", json!([]));
        assert!(rx.try_recv().is_ok());
    }

    #[test]
    fn test_olink_sig_bool() {
        let (client, _impl, loopback) = setup_no_properties_interface();
        let mut rx = client.publisher().sig_bool.subscribe();
        loopback.remote_node.notify_signal("tb.simple.NoPropertiesInterface/sigBool", json!([true]));
        let received = rx.try_recv().unwrap();
        assert_eq!(received.0, true);
    }
}
