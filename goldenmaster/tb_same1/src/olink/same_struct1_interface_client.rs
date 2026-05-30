#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_struct1_interface::SameStruct1InterfacePublisher;
use crate::api::same_struct1_interface::SameStruct1InterfaceTrait;
use crate::core_types::same_struct1_interface_data::SameStruct1InterfaceData;
use objectlink_core::traits::{IClientNode, ObjectSink};
use objectlink_core::types::Name;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink client adapter for SameStruct1Interface.
/// Implements the interface trait by forwarding operations over the OLink protocol
/// and caching property values locally.
pub struct SameStruct1InterfaceOlinkClient {
    data: RwLock<SameStruct1InterfaceData>,
    node: RwLock<Option<Arc<dyn IClientNode>>>,
    publisher: SameStruct1InterfacePublisher,
}

impl SameStruct1InterfaceOlinkClient {
    /// Set the client node used for remote communication.
    /// Must be called before link_remote() to enable operations.
    pub fn set_node(
        &self,
        node: Arc<dyn IClientNode>,
    ) {
        *self.node.write() = Some(node);
    }
}

impl Default for SameStruct1InterfaceOlinkClient {
    fn default() -> Self {
        Self { data: RwLock::new(SameStruct1InterfaceData::default()), node: RwLock::new(None), publisher: SameStruct1InterfacePublisher::default() }
    }
}

impl SameStruct1InterfaceTrait for SameStruct1InterfaceOlinkClient {
    fn func1(
        &self,
        param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
        let args = json!([param1]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.same1.SameStruct1Interface", "func1"),
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

    fn prop1(&self) -> Struct1 {
        self.data.read().prop1.clone()
    }
    fn set_prop1(
        &self,
        prop1: &Struct1,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.same1.SameStruct1Interface", "prop1"), json!(prop1));
        }
    }

    fn publisher(&self) -> &SameStruct1InterfacePublisher {
        &self.publisher
    }
}

impl ObjectSink for SameStruct1InterfaceOlinkClient {
    fn olink_object_name(&self) -> &str {
        "tb.same1.SameStruct1Interface"
    }

    fn olink_on_init(
        &self,
        _object_id: &str,
        props: Value,
        _node: &dyn IClientNode,
    ) {
        if let Ok(data) = serde_json::from_value::<SameStruct1InterfaceData>(props) {
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
            "prop1" => {
                if let Ok(v) = serde_json::from_value::<Struct1>(value) {
                    let _ = self.publisher.prop1_changed.send(v.clone());
                    self.data.write().prop1 = v;
                }
            }
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
            "sig1" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig1.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
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
