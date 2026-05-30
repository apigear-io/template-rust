#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::enum_interface::EnumInterfacePublisher;
use crate::api::enum_interface::EnumInterfaceTrait;
use crate::core_types::enum_interface_data::EnumInterfaceData;
use parking_lot::RwLock;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/tb.enum/EnumInterface";

/// MQTT client adapter for EnumInterface.
/// Implements the interface trait by forwarding operations over MQTT
/// and caching property values locally.
pub struct EnumInterfaceMqttClient {
    data: RwLock<EnumInterfaceData>,
    client: Arc<AsyncClient>,
    publisher: EnumInterfacePublisher,
}

impl EnumInterfaceMqttClient {
    /// Create a new MQTT client adapter with the given MQTT async client.
    pub fn new(client: Arc<AsyncClient>) -> Self {
        Self { data: RwLock::new(EnumInterfaceData::default()), client, publisher: EnumInterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/prop/prop0", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/prop1", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/prop2", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/prop3", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sig0", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sig1", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sig2", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sig3", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func0/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func1/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func2/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func3/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
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
        if let Ok(data) = serde_json::from_value::<EnumInterfaceData>(value) {
            *self.data.write() = data;
        }
    }

    fn handle_property_change(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
            "prop0" => {
                if let Ok(v) = serde_json::from_value::<Enum0Enum>(value) {
                    let _ = self.publisher.prop0_changed.send(v);
                    self.data.write().prop0 = v;
                }
            }
            "prop1" => {
                if let Ok(v) = serde_json::from_value::<Enum1Enum>(value) {
                    let _ = self.publisher.prop1_changed.send(v);
                    self.data.write().prop1 = v;
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_value::<Enum2Enum>(value) {
                    let _ = self.publisher.prop2_changed.send(v);
                    self.data.write().prop2 = v;
                }
            }
            "prop3" => {
                if let Ok(v) = serde_json::from_value::<Enum3Enum>(value) {
                    let _ = self.publisher.prop3_changed.send(v);
                    self.data.write().prop3 = v;
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
            "sig0" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig0.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sig1" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig1.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sig2" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig2.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sig3" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig3.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            _ => {
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl EnumInterfaceTrait for EnumInterfaceMqttClient {
    fn func0(
        &self,
        param0: Enum0Enum,
    ) -> ApiFuture<'_, Result<Enum0Enum, ApiError>> {
        let args = json!([param0]);
        let client = self.client.clone();
        let topic = format!("{}/op/func0/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func1(
        &self,
        param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        let args = json!([param1]);
        let client = self.client.clone();
        let topic = format!("{}/op/func1/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func2(
        &self,
        param2: Enum2Enum,
    ) -> ApiFuture<'_, Result<Enum2Enum, ApiError>> {
        let args = json!([param2]);
        let client = self.client.clone();
        let topic = format!("{}/op/func2/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn func3(
        &self,
        param3: Enum3Enum,
    ) -> ApiFuture<'_, Result<Enum3Enum, ApiError>> {
        let args = json!([param3]);
        let client = self.client.clone();
        let topic = format!("{}/op/func3/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn prop0(&self) -> Enum0Enum {
        self.data.read().prop0
    }
    fn set_prop0(
        &self,
        prop0: Enum0Enum,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/prop0", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop0)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop1(&self) -> Enum1Enum {
        self.data.read().prop1
    }
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/prop1", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop1)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop2(&self) -> Enum2Enum {
        self.data.read().prop2
    }
    fn set_prop2(
        &self,
        prop2: Enum2Enum,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/prop2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop2)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop3(&self) -> Enum3Enum {
        self.data.read().prop3
    }
    fn set_prop3(
        &self,
        prop3: Enum3Enum,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/prop3", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop3)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn publisher(&self) -> &EnumInterfacePublisher {
        &self.publisher
    }
}
