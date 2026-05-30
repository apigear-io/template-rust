#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_array2_interface::StructArray2InterfacePublisher;
use crate::api::struct_array2_interface::StructArray2InterfaceTrait;
use crate::core_types::struct_array2_interface_data::StructArray2InterfaceData;
use parking_lot::RwLock;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/testbed1/StructArray2Interface";

/// MQTT client adapter for StructArray2Interface.
/// Implements the interface trait by forwarding operations over MQTT
/// and caching property values locally.
pub struct StructArray2InterfaceMqttClient {
    data: RwLock<StructArray2InterfaceData>,
    client: Arc<AsyncClient>,
    publisher: StructArray2InterfacePublisher,
}

impl StructArray2InterfaceMqttClient {
    /// Create a new MQTT client adapter with the given MQTT async client.
    pub fn new(client: Arc<AsyncClient>) -> Self {
        Self { data: RwLock::new(StructArray2InterfaceData::default()), client, publisher: StructArray2InterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/prop/propBool", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propFloat", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propString", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propEnum", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigBool", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigInt", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigFloat", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigString", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcBool/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcInt/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcFloat/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcString/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcEnum/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/state", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        Ok(())
    }

    /// Handle an incoming MQTT message by dispatching to the appropriate handler.
    pub fn handle_message(
        &self,
        topic: &str,
        payload: &[u8],
    ) {
        let suffix = topic.strip_prefix(&format!("{}/", TOPIC_PREFIX)).unwrap_or("");
        let value: Value = serde_json::from_slice(payload).unwrap_or_default();

        if suffix == "state" {
            self.handle_state(value);
            return;
        }

        if let Some(prop_name) = suffix.strip_prefix("prop/") {
            self.handle_property_change(prop_name, value);
            return;
        }

        if let Some(sig_name) = suffix.strip_prefix("sig/") {
            self.handle_signal(sig_name, value);
        }
    }

    fn handle_state(
        &self,
        value: Value,
    ) {
        if let Ok(data) = serde_json::from_value::<StructArray2InterfaceData>(value) {
            *self.data.write() = data;
        }
    }

    fn handle_property_change(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
            "propBool" => {
                if let Ok(v) = serde_json::from_value::<StructBoolWithArray>(value) {
                    let _ = self.publisher.prop_bool_changed.send(v.clone());
                    self.data.write().prop_bool = v;
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<StructIntWithArray>(value) {
                    let _ = self.publisher.prop_int_changed.send(v.clone());
                    self.data.write().prop_int = v;
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<StructFloatWithArray>(value) {
                    let _ = self.publisher.prop_float_changed.send(v.clone());
                    self.data.write().prop_float = v;
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<StructStringWithArray>(value) {
                    let _ = self.publisher.prop_string_changed.send(v.clone());
                    self.data.write().prop_string = v;
                }
            }
            "propEnum" => {
                if let Ok(v) = serde_json::from_value::<StructEnumWithArray>(value) {
                    let _ = self.publisher.prop_enum_changed.send(v.clone());
                    self.data.write().prop_enum = v;
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", property_name);
            }
        }
    }

    #[allow(clippy::get_first)]
    fn handle_signal(
        &self,
        signal_name: &str,
        args: Value,
    ) {
        match signal_name {
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
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl StructArray2InterfaceTrait for StructArray2InterfaceMqttClient {
    fn func_bool(
        &self,
        param_bool: &StructBoolWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>> {
        let args = json!([param_bool]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcBool/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_int(
        &self,
        param_int: &StructIntWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>> {
        let args = json!([param_int]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcInt/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_float(
        &self,
        param_float: &StructFloatWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>> {
        let args = json!([param_float]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcFloat/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_string(
        &self,
        param_string: &StructStringWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>> {
        let args = json!([param_string]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcString/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_enum(
        &self,
        param_enum: &StructEnumWithArray,
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>> {
        let args = json!([param_enum]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcEnum/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn prop_bool(&self) -> StructBoolWithArray {
        self.data.read().prop_bool.clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &StructBoolWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_bool)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_int(&self) -> StructIntWithArray {
        self.data.read().prop_int.clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &StructIntWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_int)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_float(&self) -> StructFloatWithArray {
        self.data.read().prop_float.clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &StructFloatWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propFloat", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_float)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_string(&self) -> StructStringWithArray {
        self.data.read().prop_string.clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &StructStringWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propString", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_string)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_enum(&self) -> StructEnumWithArray {
        self.data.read().prop_enum.clone()
    }
    fn set_prop_enum(
        &self,
        prop_enum: &StructEnumWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propEnum", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_enum)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn publisher(&self) -> &StructArray2InterfacePublisher {
        &self.publisher
    }
}
