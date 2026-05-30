#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nam_es::NamEsPublisher;
use crate::api::nam_es::NamEsTrait;
use crate::core_types::nam_es_data::NamEsData;
use objectlink_core::traits::{IClientNode, ObjectSink};
use objectlink_core::types::Name;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink client adapter for NamEs.
/// Implements the interface trait by forwarding operations over the OLink protocol
/// and caching property values locally.
pub struct NamEsOlinkClient {
    data: RwLock<NamEsData>,
    node: RwLock<Option<Arc<dyn IClientNode>>>,
    publisher: NamEsPublisher,
}

impl NamEsOlinkClient {
    /// Set the client node used for remote communication.
    /// Must be called before link_remote() to enable operations.
    pub fn set_node(
        &self,
        node: Arc<dyn IClientNode>,
    ) {
        *self.node.write() = Some(node);
    }
}

impl Default for NamEsOlinkClient {
    fn default() -> Self {
        Self { data: RwLock::new(NamEsData::default()), node: RwLock::new(None), publisher: NamEsPublisher::default() }
    }
}

impl NamEsTrait for NamEsOlinkClient {
    fn some_function(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([some_param]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.names.Nam_Es", "SOME_FUNCTION"),
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

    fn some_function2(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([some_param]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("tb.names.Nam_Es", "Some_Function2"),
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

    fn switch(&self) -> bool {
        self.data.read().switch
    }
    fn set_switch(
        &self,
        switch: bool,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.names.Nam_Es", "Switch"), json!(switch));
        }
    }

    fn some_property(&self) -> i32 {
        self.data.read().some_property
    }
    fn set_some_property(
        &self,
        some_property: i32,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.names.Nam_Es", "SOME_PROPERTY"), json!(some_property));
        }
    }

    fn some_poperty2(&self) -> i32 {
        self.data.read().some_poperty2
    }
    fn set_some_poperty2(
        &self,
        some_poperty2: i32,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.names.Nam_Es", "Some_Poperty2"), json!(some_poperty2));
        }
    }

    fn enum_property(&self) -> Enum_With_Under_scoresEnum {
        self.data.read().enum_property
    }
    fn set_enum_property(
        &self,
        enum_property: Enum_With_Under_scoresEnum,
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("tb.names.Nam_Es", "enum_property"), json!(enum_property));
        }
    }

    fn publisher(&self) -> &NamEsPublisher {
        &self.publisher
    }
}

impl ObjectSink for NamEsOlinkClient {
    fn olink_object_name(&self) -> &str {
        "tb.names.Nam_Es"
    }

    fn olink_on_init(
        &self,
        _object_id: &str,
        props: Value,
        _node: &dyn IClientNode,
    ) {
        if let Ok(data) = serde_json::from_value::<NamEsData>(props) {
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
            "Switch" => {
                if let Ok(v) = serde_json::from_value::<bool>(value) {
                    let _ = self.publisher.switch_changed.send(v);
                    self.data.write().switch = v;
                }
            }
            "SOME_PROPERTY" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    let _ = self.publisher.some_property_changed.send(v);
                    self.data.write().some_property = v;
                }
            }
            "Some_Poperty2" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    let _ = self.publisher.some_poperty2_changed.send(v);
                    self.data.write().some_poperty2 = v;
                }
            }
            "enum_property" => {
                if let Ok(v) = serde_json::from_value::<Enum_With_Under_scoresEnum>(value) {
                    let _ = self.publisher.enum_property_changed.send(v);
                    self.data.write().enum_property = v;
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
            "SOME_SIGNAL" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.some_signal.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "Some_Signal2" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.some_signal2.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
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
