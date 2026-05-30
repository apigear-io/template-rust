#![allow(unused_imports, dead_code, clippy::never_loop)]
use crate::api::empty_interface::EmptyInterfaceTrait;
use crate::core_types::empty_interface_data::EmptyInterfaceData;
use objectlink_core::traits::{IClientNode, ObjectSink};
use objectlink_core::types::Name;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink client adapter for EmptyInterface.
/// Implements the interface trait by forwarding operations over the OLink protocol
/// and caching property values locally.
pub struct EmptyInterfaceOlinkClient {
    data: RwLock<EmptyInterfaceData>,
    node: RwLock<Option<Arc<dyn IClientNode>>>,
}

impl EmptyInterfaceOlinkClient {
    /// Set the client node used for remote communication.
    /// Must be called before link_remote() to enable operations.
    pub fn set_node(
        &self,
        node: Arc<dyn IClientNode>,
    ) {
        *self.node.write() = Some(node);
    }
}

impl Default for EmptyInterfaceOlinkClient {
    fn default() -> Self {
        Self { data: RwLock::new(EmptyInterfaceData::default()), node: RwLock::new(None) }
    }
}

impl EmptyInterfaceTrait for EmptyInterfaceOlinkClient {}

impl ObjectSink for EmptyInterfaceOlinkClient {
    fn olink_object_name(&self) -> &str {
        "tb.simple.EmptyInterface"
    }

    fn olink_on_init(
        &self,
        _object_id: &str,
        props: Value,
        _node: &dyn IClientNode,
    ) {
        if let Ok(data) = serde_json::from_value::<EmptyInterfaceData>(props) {
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
            _ => {
                tracing::warn!("Unknown signal: {}", signal_id);
            }
        }
    }

    fn olink_on_release(&self) {
        *self.node.write() = None;
    }
}
