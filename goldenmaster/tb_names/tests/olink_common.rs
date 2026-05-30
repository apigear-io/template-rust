use objectlink_core::client_node::ClientNode;
use objectlink_core::client_registry::ClientRegistry;
use objectlink_core::remote_node::RemoteNode;
use objectlink_core::remote_registry::RemoteRegistry;
use objectlink_core::traits::ObjectSource;
use std::sync::Arc;

/// Holds the OLink loopback wiring for tests.
/// The `_service` field keeps the `Arc<dyn ObjectSource>` alive because
/// `RemoteRegistry` only holds a `Weak` reference.
pub struct OlinkLoopback {
    pub client_node: Arc<ClientNode>,
    pub remote_node: Arc<RemoteNode>,
    _client_registry: Arc<ClientRegistry>,
    _remote_registry: Arc<RemoteRegistry>,
    _service: Arc<dyn ObjectSource>,
}

/// Create a loopback OLink setup with the given service wired in.
///
/// `ClientNode.on_write` feeds into `RemoteNode.handle_message` and vice versa,
/// so messages round-trip without any network.
pub fn setup_olink_loopback(service: Arc<dyn ObjectSource>) -> OlinkLoopback {
    let client_registry = Arc::new(ClientRegistry::new());
    let remote_registry = Arc::new(RemoteRegistry::new());
    let client_node = Arc::new(ClientNode::new(client_registry.clone()));
    let remote_node = Arc::new(RemoteNode::new(remote_registry.clone()));

    // Client writes -> Remote reads
    let rn = remote_node.clone();
    client_node.on_write(Box::new(move |msg: &str| {
        rn.handle_message(msg);
    }));

    // Remote writes -> Client reads
    let cn = client_node.clone();
    remote_node.on_write(Box::new(move |msg: &str| {
        cn.handle_message(msg);
    }));

    remote_registry.add_source(Arc::downgrade(&service));

    OlinkLoopback { client_node, remote_node, _client_registry: client_registry, _remote_registry: remote_registry, _service: service }
}
