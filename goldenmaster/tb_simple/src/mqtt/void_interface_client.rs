use apigear::{ApiError, ApiFuture};
use crate::api::void_interface::VoidInterfacePublisher;
use crate::api::void_interface::VoidInterfaceTrait;
use crate::core_types::void_interface_data::VoidInterfaceData;
use parking_lot::RwLock;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/tb.simple/VoidInterface";

/// MQTT client adapter for VoidInterface.
/// Implements the interface trait by forwarding operations over MQTT
/// and caching property values locally.
pub struct VoidInterfaceMqttClient {
    data: RwLock<VoidInterfaceData>,
    client: Arc<AsyncClient>,
    publisher: VoidInterfacePublisher,
}

impl VoidInterfaceMqttClient {
    /// Create a new MQTT client adapter with the given MQTT async client.
    pub fn new(client: Arc<AsyncClient>) -> Self {
        Self { data: RwLock::new(VoidInterfaceData::default()), client, publisher: VoidInterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/sig/sigVoid", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcVoid/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
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
        if let Ok(data) = serde_json::from_value::<VoidInterfaceData>(value) {
            *self.data.write() = data;
        }
    }

    fn handle_property_change(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
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
            "sigVoid" => {
                let _ = self.publisher.sig_void.send(());
            }
            _ => {
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl VoidInterfaceTrait for VoidInterfaceMqttClient {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([]);
        let client = self.client.clone();
        let topic = format!("{}/op/funcVoid/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }

    fn publisher(&self) -> &VoidInterfacePublisher {
        &self.publisher
    }
}
