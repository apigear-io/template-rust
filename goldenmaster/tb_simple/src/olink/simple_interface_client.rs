use apigear::{ApiError, ApiFuture};
use crate::api::simple_interface::SimpleInterfacePublisher;
use crate::api::simple_interface::SimpleInterfaceTrait;
use crate::core_types::simple_interface_data::SimpleInterfaceData;
use objectlink_core::traits::{IClientNode, ObjectSink};
use objectlink_core::types::Name;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink client adapter for SimpleInterface.
/// Implements the interface trait by forwarding operations over the OLink protocol
/// and caching property values locally.
pub struct SimpleInterfaceOlinkClient {
    data: RwLock<SimpleInterfaceData>,
    node: RwLock<Option<Arc<dyn IClientNode>>>,
    publisher: SimpleInterfacePublisher,
}

impl SimpleInterfaceOlinkClient {
    /// Set the client node used for remote communication.
    /// Must be called before link_remote() to enable operations.
    pub fn set_node(
        &self,
        node: Arc<dyn IClientNode>,
    ) {
        *self.node.write() = Some(node);
    }
}

impl Default for SimpleInterfaceOlinkClient {
    fn default() -> Self {
        Self { data: RwLock::new(SimpleInterfaceData::default()), node: RwLock::new(None), publisher: SimpleInterfacePublisher::default() }
    }
}

impl SimpleInterfaceTrait for SimpleInterfaceOlinkClient {
    fn func_no_return_value(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([param_bool]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcNoReturnValue"),
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

    fn func_no_params(&self) -> ApiFuture<'_, Result<bool, ApiError>> {
        let args = json!([]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcNoParams"),
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

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
        let args = json!([param_bool]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcBool"),
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
        param_int: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        let args = json!([param_int]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcInt"),
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
        param_int32: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        let args = json!([param_int32]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcInt32"),
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
        param_int64: i64,
    ) -> ApiFuture<'_, Result<i64, ApiError>> {
        let args = json!([param_int64]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcInt64"),
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
        param_float: f32,
    ) -> ApiFuture<'_, Result<f32, ApiError>> {
        let args = json!([param_float]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcFloat"),
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
        param_float32: f32,
    ) -> ApiFuture<'_, Result<f32, ApiError>> {
        let args = json!([param_float32]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcFloat32"),
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
        param_float: f64,
    ) -> ApiFuture<'_, Result<f64, ApiError>> {
        let args = json!([param_float]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcFloat64"),
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
        param_string: &str,
    ) -> ApiFuture<'_, Result<String, ApiError>> {
        let args = json!([param_string]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.simple.SimpleInterface", "funcString"),
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

    fn prop_bool(&self) -> bool {
        self.data.read().prop_bool
    }
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleInterface", "propBool"), json!(prop_bool));
        }
    }

    fn prop_int(&self) -> i32 {
        self.data.read().prop_int
    }
    fn set_prop_int(
        &self,
        prop_int: i32,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleInterface", "propInt"), json!(prop_int));
        }
    }

    fn prop_int32(&self) -> i32 {
        self.data.read().prop_int32
    }
    fn set_prop_int32(
        &self,
        prop_int32: i32,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleInterface", "propInt32"), json!(prop_int32));
        }
    }

    fn prop_int64(&self) -> i64 {
        self.data.read().prop_int64
    }
    fn set_prop_int64(
        &self,
        prop_int64: i64,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleInterface", "propInt64"), json!(prop_int64));
        }
    }

    fn prop_float(&self) -> f32 {
        self.data.read().prop_float
    }
    fn set_prop_float(
        &self,
        prop_float: f32,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleInterface", "propFloat"), json!(prop_float));
        }
    }

    fn prop_float32(&self) -> f32 {
        self.data.read().prop_float32
    }
    fn set_prop_float32(
        &self,
        prop_float32: f32,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleInterface", "propFloat32"), json!(prop_float32));
        }
    }

    fn prop_float64(&self) -> f64 {
        self.data.read().prop_float64
    }
    fn set_prop_float64(
        &self,
        prop_float64: f64,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleInterface", "propFloat64"), json!(prop_float64));
        }
    }

    fn prop_string(&self) -> String {
        self.data.read().prop_string.clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &str,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.simple.SimpleInterface", "propString"), json!(prop_string));
        }
    }

    fn publisher(&self) -> &SimpleInterfacePublisher {
        &self.publisher
    }
}

impl ObjectSink for SimpleInterfaceOlinkClient {
    fn olink_object_name(&self) -> &str {
        "tb.simple.SimpleInterface"
    }

    fn olink_on_init(
        &self,
        _object_id: &str,
        props: Value,
        _node: &dyn IClientNode,
    ) {
        if let Ok(data) = serde_json::from_value::<SimpleInterfaceData>(props) {
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
                if let Ok(v) = serde_json::from_value::<bool>(value) {
                    let _ = self.publisher.prop_bool_changed.send(v);
                    self.data.write().prop_bool = v;
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    let _ = self.publisher.prop_int_changed.send(v);
                    self.data.write().prop_int = v;
                }
            }
            "propInt32" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    let _ = self.publisher.prop_int32_changed.send(v);
                    self.data.write().prop_int32 = v;
                }
            }
            "propInt64" => {
                if let Ok(v) = serde_json::from_value::<i64>(value) {
                    let _ = self.publisher.prop_int64_changed.send(v);
                    self.data.write().prop_int64 = v;
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<f32>(value) {
                    let _ = self.publisher.prop_float_changed.send(v);
                    self.data.write().prop_float = v;
                }
            }
            "propFloat32" => {
                if let Ok(v) = serde_json::from_value::<f32>(value) {
                    let _ = self.publisher.prop_float32_changed.send(v);
                    self.data.write().prop_float32 = v;
                }
            }
            "propFloat64" => {
                if let Ok(v) = serde_json::from_value::<f64>(value) {
                    let _ = self.publisher.prop_float64_changed.send(v);
                    self.data.write().prop_float64 = v;
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<String>(value) {
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
