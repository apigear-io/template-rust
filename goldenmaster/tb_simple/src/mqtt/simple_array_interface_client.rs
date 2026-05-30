use apigear::{ApiError, ApiFuture};
use crate::api::simple_array_interface::SimpleArrayInterfacePublisher;
use crate::api::simple_array_interface::SimpleArrayInterfaceTrait;
use crate::core_types::simple_array_interface_data::SimpleArrayInterfaceData;
use parking_lot::RwLock;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/tb.simple/SimpleArrayInterface";

/// MQTT client adapter for SimpleArrayInterface.
/// Implements the interface trait by forwarding operations over MQTT
/// and caching property values locally.
pub struct SimpleArrayInterfaceMqttClient {
    data: RwLock<SimpleArrayInterfaceData>,
    client: Arc<AsyncClient>,
    publisher: SimpleArrayInterfacePublisher,
}

impl SimpleArrayInterfaceMqttClient {
    /// Create a new MQTT client adapter with the given MQTT async client.
    pub fn new(client: Arc<AsyncClient>) -> Self {
        Self { data: RwLock::new(SimpleArrayInterfaceData::default()), client, publisher: SimpleArrayInterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/prop/propBool", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt32", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt64", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propFloat", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propFloat32", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propFloat64", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propString", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propReadOnlyString", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigBool", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigInt", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigInt32", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigInt64", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigFloat", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigFloat32", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigFloat64", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigString", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcBool/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcInt/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcInt32/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcInt64/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcFloat/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcFloat32/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcFloat64/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcString/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
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
        if let Ok(data) = serde_json::from_value::<SimpleArrayInterfaceData>(value) {
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
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl SimpleArrayInterfaceTrait for SimpleArrayInterfaceMqttClient {
    fn func_bool(
        &self,
        param_bool: &[bool],
    ) -> ApiFuture<'_, Result<Vec<bool>, ApiError>> {
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
        param_int: &[i32],
    ) -> ApiFuture<'_, Result<Vec<i32>, ApiError>> {
        let args = json!([param_int]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcInt/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_int32(
        &self,
        param_int32: &[i32],
    ) -> ApiFuture<'_, Result<Vec<i32>, ApiError>> {
        let args = json!([param_int32]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcInt32/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_int64(
        &self,
        param_int64: &[i64],
    ) -> ApiFuture<'_, Result<Vec<i64>, ApiError>> {
        let args = json!([param_int64]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcInt64/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_float(
        &self,
        param_float: &[f32],
    ) -> ApiFuture<'_, Result<Vec<f32>, ApiError>> {
        let args = json!([param_float]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcFloat/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_float32(
        &self,
        param_float32: &[f32],
    ) -> ApiFuture<'_, Result<Vec<f32>, ApiError>> {
        let args = json!([param_float32]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcFloat32/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_float64(
        &self,
        param_float: &[f64],
    ) -> ApiFuture<'_, Result<Vec<f64>, ApiError>> {
        let args = json!([param_float]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcFloat64/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func_string(
        &self,
        param_string: &[String],
    ) -> ApiFuture<'_, Result<Vec<String>, ApiError>> {
        let args = json!([param_string]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcString/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn prop_bool(&self) -> Vec<bool> {
        self.data.read().prop_bool.clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &[bool],
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_bool)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_int(&self) -> Vec<i32> {
        self.data.read().prop_int.clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &[i32],
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_int)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_int32(&self) -> Vec<i32> {
        self.data.read().prop_int32.clone()
    }
    fn set_prop_int32(
        &self,
        prop_int32: &[i32],
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propInt32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_int32)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_int64(&self) -> Vec<i64> {
        self.data.read().prop_int64.clone()
    }
    fn set_prop_int64(
        &self,
        prop_int64: &[i64],
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propInt64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_int64)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_float(&self) -> Vec<f32> {
        self.data.read().prop_float.clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &[f32],
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propFloat", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_float)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_float32(&self) -> Vec<f32> {
        self.data.read().prop_float32.clone()
    }
    fn set_prop_float32(
        &self,
        prop_float32: &[f32],
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propFloat32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_float32)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_float64(&self) -> Vec<f64> {
        self.data.read().prop_float64.clone()
    }
    fn set_prop_float64(
        &self,
        prop_float64: &[f64],
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propFloat64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_float64)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_string(&self) -> Vec<String> {
        self.data.read().prop_string.clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &[String],
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/propString", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_string)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop_read_only_string(&self) -> String {
        self.data.read().prop_read_only_string.clone()
    }

    fn publisher(&self) -> &SimpleArrayInterfacePublisher {
        &self.publisher
    }
}
