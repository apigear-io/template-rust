use apigear::{ApiError, ApiFuture};
use crate::api::simple_array_interface::SimpleArrayInterfacePublisher;
use crate::api::simple_array_interface::SimpleArrayInterfaceTrait;
use crate::core_types::simple_array_interface_data::SimpleArrayInterfaceData;
use objectlink_core::traits::{IClientNode, ObjectSink};
use objectlink_core::types::Name;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink client adapter for SimpleArrayInterface.
/// Implements the interface trait by forwarding operations over the OLink protocol
/// and caching property values locally.
pub struct SimpleArrayInterfaceOlinkClient {
    data: RwLock<SimpleArrayInterfaceData>,
    node: RwLock<Option<Arc<dyn IClientNode>>>,
    publisher: SimpleArrayInterfacePublisher,
}

impl SimpleArrayInterfaceOlinkClient {
    /// Set the client node used for remote communication.
    /// Must be called before link_remote() to enable operations.
    pub fn set_node(
        &self,
        node: Arc<dyn IClientNode>,
    ) {
        *self.node.write() = Some(node);
    }
}

impl Default for SimpleArrayInterfaceOlinkClient {
    fn default() -> Self {
        Self { data: RwLock::new(SimpleArrayInterfaceData::default()), node: RwLock::new(None), publisher: SimpleArrayInterfacePublisher::default() }
    }
}

impl SimpleArrayInterfaceTrait for SimpleArrayInterfaceOlinkClient {
    fn func_bool(
        &self,
        param_bool: &[bool],
    ) -> ApiFuture<'_, Result<Vec<bool>, ApiError>> {
        let args = json!([param_bool]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleArrayInterface", "funcBool"),
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
        param_int: &[i32],
    ) -> ApiFuture<'_, Result<Vec<i32>, ApiError>> {
        let args = json!([param_int]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleArrayInterface", "funcInt"),
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

    fn func_int32(
        &self,
        param_int32: &[i32],
    ) -> ApiFuture<'_, Result<Vec<i32>, ApiError>> {
        let args = json!([param_int32]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleArrayInterface", "funcInt32"),
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

    fn func_int64(
        &self,
        param_int64: &[i64],
    ) -> ApiFuture<'_, Result<Vec<i64>, ApiError>> {
        let args = json!([param_int64]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleArrayInterface", "funcInt64"),
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
        param_float: &[f32],
    ) -> ApiFuture<'_, Result<Vec<f32>, ApiError>> {
        let args = json!([param_float]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleArrayInterface", "funcFloat"),
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

    fn func_float32(
        &self,
        param_float32: &[f32],
    ) -> ApiFuture<'_, Result<Vec<f32>, ApiError>> {
        let args = json!([param_float32]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleArrayInterface", "funcFloat32"),
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

    fn func_float64(
        &self,
        param_float: &[f64],
    ) -> ApiFuture<'_, Result<Vec<f64>, ApiError>> {
        let args = json!([param_float]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleArrayInterface", "funcFloat64"),
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
        param_string: &[String],
    ) -> ApiFuture<'_, Result<Vec<String>, ApiError>> {
        let args = json!([param_string]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleArrayInterface", "funcString"),
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

    fn prop_bool(&self) -> Vec<bool> {
        self.data.read().prop_bool.clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &[bool],
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleArrayInterface", "propBool"), json!(prop_bool));
        }
    }

    fn prop_int(&self) -> Vec<i32> {
        self.data.read().prop_int.clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &[i32],
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleArrayInterface", "propInt"), json!(prop_int));
        }
    }

    fn prop_int32(&self) -> Vec<i32> {
        self.data.read().prop_int32.clone()
    }
    fn set_prop_int32(
        &self,
        prop_int32: &[i32],
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleArrayInterface", "propInt32"), json!(prop_int32));
        }
    }

    fn prop_int64(&self) -> Vec<i64> {
        self.data.read().prop_int64.clone()
    }
    fn set_prop_int64(
        &self,
        prop_int64: &[i64],
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleArrayInterface", "propInt64"), json!(prop_int64));
        }
    }

    fn prop_float(&self) -> Vec<f32> {
        self.data.read().prop_float.clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &[f32],
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleArrayInterface", "propFloat"), json!(prop_float));
        }
    }

    fn prop_float32(&self) -> Vec<f32> {
        self.data.read().prop_float32.clone()
    }
    fn set_prop_float32(
        &self,
        prop_float32: &[f32],
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleArrayInterface", "propFloat32"), json!(prop_float32));
        }
    }

    fn prop_float64(&self) -> Vec<f64> {
        self.data.read().prop_float64.clone()
    }
    fn set_prop_float64(
        &self,
        prop_float64: &[f64],
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleArrayInterface", "propFloat64"), json!(prop_float64));
        }
    }

    fn prop_string(&self) -> Vec<String> {
        self.data.read().prop_string.clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &[String],
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleArrayInterface", "propString"), json!(prop_string));
        }
    }

    fn prop_read_only_string(&self) -> String {
        self.data.read().prop_read_only_string.clone()
    }

    fn publisher(&self) -> &SimpleArrayInterfacePublisher {
        &self.publisher
    }
}

impl ObjectSink for SimpleArrayInterfaceOlinkClient {
    fn olink_object_name(&self) -> &str {
        "tb.simple.SimpleArrayInterface"
    }

    fn olink_on_init(
        &self,
        _object_id: &str,
        props: Value,
        _node: &dyn IClientNode,
    ) {
        if let Ok(data) = serde_json::from_value::<SimpleArrayInterfaceData>(props) {
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
                if let Ok(v) = serde_json::from_value::<Vec<bool>>(value) {
                    let _ = self.publisher.prop_bool_changed.send(v.clone());
                    self.data.write().prop_bool = v;
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<Vec<i32>>(value) {
                    let _ = self.publisher.prop_int_changed.send(v.clone());
                    self.data.write().prop_int = v;
                }
            }
            "propInt32" => {
                if let Ok(v) = serde_json::from_value::<Vec<i32>>(value) {
                    let _ = self.publisher.prop_int32_changed.send(v.clone());
                    self.data.write().prop_int32 = v;
                }
            }
            "propInt64" => {
                if let Ok(v) = serde_json::from_value::<Vec<i64>>(value) {
                    let _ = self.publisher.prop_int64_changed.send(v.clone());
                    self.data.write().prop_int64 = v;
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<Vec<f32>>(value) {
                    let _ = self.publisher.prop_float_changed.send(v.clone());
                    self.data.write().prop_float = v;
                }
            }
            "propFloat32" => {
                if let Ok(v) = serde_json::from_value::<Vec<f32>>(value) {
                    let _ = self.publisher.prop_float32_changed.send(v.clone());
                    self.data.write().prop_float32 = v;
                }
            }
            "propFloat64" => {
                if let Ok(v) = serde_json::from_value::<Vec<f64>>(value) {
                    let _ = self.publisher.prop_float64_changed.send(v.clone());
                    self.data.write().prop_float64 = v;
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<Vec<String>>(value) {
                    let _ = self.publisher.prop_string_changed.send(v.clone());
                    self.data.write().prop_string = v;
                }
            }
            "propReadOnlyString" => {
                if let Ok(v) = serde_json::from_value::<String>(value) {
                    let _ = self.publisher.prop_read_only_string_changed.send(v.clone());
                    self.data.write().prop_read_only_string = v;
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
            "sigInt32" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_int32.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigInt64" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_int64.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigFloat" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_float.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigFloat32" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_float32.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigFloat64" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_float64.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
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
