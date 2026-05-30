#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_struct2_interface::SameStruct2InterfacePublisher;
use crate::api::same_struct2_interface::SameStruct2InterfaceTrait;
use crate::core_types::same_struct2_interface_data::SameStruct2InterfaceData;
use parking_lot::RwLock;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/tb.same1/SameStruct2Interface";

/// MQTT client adapter for SameStruct2Interface.
/// Implements the interface trait by forwarding operations over MQTT
/// and caching property values locally.
pub struct SameStruct2InterfaceMqttClient {
    data: RwLock<SameStruct2InterfaceData>,
    client: Arc<AsyncClient>,
    publisher: SameStruct2InterfacePublisher,
}

impl SameStruct2InterfaceMqttClient {
    /// Create a new MQTT client adapter with the given MQTT async client.
    pub fn new(client: Arc<AsyncClient>) -> Self {
        Self { data: RwLock::new(SameStruct2InterfaceData::default()), client, publisher: SameStruct2InterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/prop/prop1", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/prop2", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sig1", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sig2", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func1/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func2/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
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
        if let Ok(data) = serde_json::from_value::<SameStruct2InterfaceData>(value) {
            *self.data.write() = data;
        }
    }

    fn handle_property_change(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
            "prop1" => {
                if let Ok(v) = serde_json::from_value::<Struct2>(value) {
                    let _ = self.publisher.prop1_changed.send(v.clone());
                    self.data.write().prop1 = v;
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_value::<Struct2>(value) {
                    let _ = self.publisher.prop2_changed.send(v.clone());
                    self.data.write().prop2 = v;
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
            "sig1" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig1.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sig2" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig2.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(), serde_json::from_value(arr.get(1).cloned().unwrap_or_default()).unwrap_or_default()));
                }
            }
            _ => {
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl SameStruct2InterfaceTrait for SameStruct2InterfaceMqttClient {
    fn func1(
        &self,
        param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
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
        param1: &Struct1,
        param2: &Struct2,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
        let args = json!([param1, param2]);
        let client = self.client.clone();
        let topic = format!("{}/op/func2/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn prop1(&self) -> Struct2 {
        self.data.read().prop1.clone()
    }
    fn set_prop1(
        &self,
        prop1: &Struct2,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/prop1", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop1)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn prop2(&self) -> Struct2 {
        self.data.read().prop2.clone()
    }
    fn set_prop2(
        &self,
        prop2: &Struct2,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/prop/prop2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop2)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
        });
    }

    fn publisher(&self) -> &SameStruct2InterfacePublisher {
        &self.publisher
    }
}
