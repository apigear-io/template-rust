use apigear::{ApiError, ApiFuture};
use crate::api::void_interface::VoidInterfacePublisher;
use crate::api::void_interface::VoidInterfaceTrait;
use crate::core_types::void_interface_data::VoidInterfaceData;
use objectlink_core::traits::{IClientNode, ObjectSink};
use objectlink_core::types::Name;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink client adapter for VoidInterface.
/// Implements the interface trait by forwarding operations over the OLink protocol
/// and caching property values locally.
pub struct VoidInterfaceOlinkClient {
    data: RwLock<VoidInterfaceData>,
    node: RwLock<Option<Arc<dyn IClientNode>>>,
    publisher: VoidInterfacePublisher,
}

impl VoidInterfaceOlinkClient {
    /// Set the client node used for remote communication.
    /// Must be called before link_remote() to enable operations.
    pub fn set_node(
        &self,
        node: Arc<dyn IClientNode>,
    ) {
        *self.node.write() = Some(node);
    }
}

impl Default for VoidInterfaceOlinkClient {
    fn default() -> Self {
        Self { data: RwLock::new(VoidInterfaceData::default()), node: RwLock::new(None), publisher: VoidInterfacePublisher::default() }
    }
}

impl VoidInterfaceTrait for VoidInterfaceOlinkClient {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.VoidInterface", "funcVoid"),
                    args,
                    Some(Box::new(move |value| {
                        let _ = tx.send(value);
                    })),
                );
                match rx.recv() {
                    Ok(value) => Ok(serde_json::from_value(value).unwrap_or_default()),
                    Err(_) => Err(ApiError::OperationFailed("no reply".to_string())),
                }
            } else {
                Err(ApiError::NotConnected)
            }
        })
    }

    fn publisher(&self) -> &VoidInterfacePublisher {
        &self.publisher
    }
}

impl ObjectSink for VoidInterfaceOlinkClient {
    fn olink_object_name(&self) -> &str {
        "tb.simple.VoidInterface"
    }

    fn olink_on_init(
        &self,
        _object_id: &str,
        props: Value,
        _node: &dyn IClientNode,
    ) {
        if let Ok(data) = serde_json::from_value::<VoidInterfaceData>(props) {
            *self.data.write() = data;
        }
        // Store node reference - we need to upcast to Arc, but we only have &dyn IClientNode.
        // The node is set separately via set_node() before link_remote().
    }

    fn olink_on_property_changed(
        &self,
        property_id: &str,
        value: Value,
    ) {
        let member = Name::member_name(property_id);
        match member {
            _ => {
                tracing::warn!("Unknown property: {}", property_id);
            }
        }
    }

    #[allow(clippy::get_first)]
    fn olink_on_signal(
        &self,
        signal_id: &str,
        args: Value,
    ) {
        let member = Name::member_name(signal_id);
        match member {
            "sigVoid" => {
                let _ = self.publisher.sig_void.send(());
            }
            _ => {
                tracing::warn!("Unknown signal: {}", signal_id);
            }
        }
    }

    fn olink_on_release(&self) {
        *self.node.write() = None;
    }
}
