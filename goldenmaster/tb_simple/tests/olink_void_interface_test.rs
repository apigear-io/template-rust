mod olink_common;
use tb_simple::api::void_interface::VoidInterfaceTrait;
use tb_simple::implementation::void_interface::VoidInterface;
use tb_simple::olink::void_interface_client::VoidInterfaceOlinkClient;
use tb_simple::olink::void_interface_service::VoidInterfaceOlinkService;
use objectlink_core::traits::ObjectSink;
use objectlink_core::traits::IRemoteNode;
use olink_common::setup_olink_loopback;
use serde_json::json;
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_void_interface() -> (Arc<VoidInterfaceOlinkClient>, Arc<VoidInterface>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new(VoidInterface::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new(VoidInterfaceOlinkService::new(impl_.clone() as Arc<dyn VoidInterfaceTrait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new(VoidInterfaceOlinkClient::default());
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
        let (client, _impl, _loopback) = setup_void_interface();
        let result = client.func_void().await;
        assert!(result.is_ok());
    }

    // -- Signals --

    #[test]
    fn test_olink_sig_void() {
        let (client, _impl, loopback) = setup_void_interface();
        let mut rx = client.publisher().sig_void.subscribe();
        loopback.remote_node.notify_signal("tb.simple.VoidInterface/sigVoid", json!([]));
        assert!(rx.try_recv().is_ok());
    }
}
