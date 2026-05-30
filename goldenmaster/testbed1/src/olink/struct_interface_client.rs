#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_interface::StructInterfacePublisher;
use crate::api::struct_interface::StructInterfaceTrait;
use crate::core_types::struct_interface_data::StructInterfaceData;
use objectlink_core::traits::{IClientNode, ObjectSink};
use objectlink_core::types::Name;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink client adapter for StructInterface.
/// Implements the interface trait by forwarding operations over the OLink protocol
/// and caching property values locally.
pub struct StructInterfaceOlinkClient {
    data: RwLock<StructInterfaceData>,
    node: RwLock<Option<Arc<dyn IClientNode>>>,
    publisher: StructInterfacePublisher,
}

impl StructInterfaceOlinkClient {
    /// Set the client node used for remote communication.
    /// Must be called before link_remote() to enable operations.
    pub fn set_node(
        &self,
        node: Arc<dyn IClientNode>,
    ) {
        *self.node.write() = Some(node);
    }
}

impl Default for StructInterfaceOlinkClient {
    fn default() -> Self {
        Self { data: RwLock::new(StructInterfaceData::default()), node: RwLock::new(None), publisher: StructInterfacePublisher::default() }
    }
}

impl StructInterfaceTrait for StructInterfaceOlinkClient {
    fn func_bool(
        &self,
        param_bool: &StructBool,
    ) -> ApiFuture<'_, Result<StructBool, ApiError>> {
        let args = json!([param_bool]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("testbed1.StructInterface", "funcBool"),
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

    fn func_int(
        &self,
        param_int: &StructInt,
    ) -> ApiFuture<'_, Result<StructInt, ApiError>> {
        let args = json!([param_int]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("testbed1.StructInterface", "funcInt"),
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

    fn func_float(
        &self,
        param_float: &StructFloat,
    ) -> ApiFuture<'_, Result<StructFloat, ApiError>> {
        let args = json!([param_float]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("testbed1.StructInterface", "funcFloat"),
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

    fn func_string(
        &self,
        param_string: &StructString,
    ) -> ApiFuture<'_, Result<StructString, ApiError>> {
        let args = json!([param_string]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("testbed1.StructInterface", "funcString"),
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

    fn prop_bool(&self) -> StructBool {
        self.data.read().prop_bool.clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &StructBool,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("testbed1.StructInterface", "propBool"), json!(prop_bool));
        }
    }

    fn prop_int(&self) -> StructInt {
        self.data.read().prop_int.clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &StructInt,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("testbed1.StructInterface", "propInt"), json!(prop_int));
        }
    }

    fn prop_float(&self) -> StructFloat {
        self.data.read().prop_float.clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &StructFloat,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("testbed1.StructInterface", "propFloat"), json!(prop_float));
        }
    }

    fn prop_string(&self) -> StructString {
        self.data.read().prop_string.clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &StructString,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("testbed1.StructInterface", "propString"), json!(prop_string));
        }
    }

    fn publisher(&self) -> &StructInterfacePublisher {
        &self.publisher
    }
}

impl ObjectSink for StructInterfaceOlinkClient {
    fn olink_object_name(&self) -> &str {
        "testbed1.StructInterface"
    }

    fn olink_on_init(
        &self,
        _object_id: &str,
        props: Value,
        _node: &dyn IClientNode,
    ) {
        if let Ok(data) = serde_json::from_value::<StructInterfaceData>(props) {
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
            "propBool" => {
                if let Ok(v) = serde_json::from_value::<StructBool>(value) {
                    let _ = self.publisher.prop_bool_changed.send(v.clone());
                    self.data.write().prop_bool = v;
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<StructInt>(value) {
                    let _ = self.publisher.prop_int_changed.send(v.clone());
                    self.data.write().prop_int = v;
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<StructFloat>(value) {
                    let _ = self.publisher.prop_float_changed.send(v.clone());
                    self.data.write().prop_float = v;
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<StructString>(value) {
                    let _ = self.publisher.prop_string_changed.send(v.clone());
                    self.data.write().prop_string = v;
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
            "sigBool" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_bool.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigInt" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_int.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigFloat" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_float.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigString" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_string.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
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
