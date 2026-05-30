use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/tb.simple/NoOperationsInterface";

/// MQTT service adapter for NoOperationsInterface.
/// Bridges a local implementation to MQTT by subscribing to operation requests
/// and publishing property changes and signals.
pub struct NoOperationsInterfaceMqttService {
    impl_: Arc<dyn NoOperationsInterfaceTrait>,
    client: Arc<AsyncClient>,
}

impl NoOperationsInterfaceMqttService {
    pub fn new(
        impl_: Arc<dyn NoOperationsInterfaceTrait>,
        client: Arc<AsyncClient>,
    ) -> Self {
        Self { impl_, client }
    }

    /// Subscribe to all relevant MQTT topics for this service.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/prop/propBool", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
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

        if let Some(rest) = suffix.strip_prefix("op/") {
            if let Some(op_name) = rest.strip_suffix("/req") {
                self.handle_invoke(op_name, value);
            }
            return;
        }

        if let Some(prop_name) = suffix.strip_prefix("prop/") {
            self.handle_set_property(prop_name, value);
        }
    }

    #[allow(clippy::get_first)]
    fn handle_invoke(
        &self,
        method_name: &str,
        args: Value,
    ) {
        #[allow(unused_variables)]
        let arr = args.as_array();
        let client = self.client.clone();
        match method_name {
            _ => {
                tracing::warn!("Unknown method: {}", method_name);
            }
        }
    }

    fn handle_set_property(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
            "propBool" => {
                if let Ok(v) = serde_json::from_value::<bool>(value) {
                    self.impl_.set_prop_bool(v);
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop_int(v);
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", property_name);
            }
        }
    }
    /// Publish propBool property change over MQTT (retained).
    pub async fn publish_prop_bool_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_bool());
        let topic = format!("{}/prop/propBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propInt property change over MQTT (retained).
    pub async fn publish_prop_int_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_int());
        let topic = format!("{}/prop/propInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    pub async fn publish_sig_void(&self) -> Result<(), rumqttc::ClientError> {
        let args = json!([]);
        let topic = format!("{}/sig/sigVoid", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_bool(
        &self,
        param_bool: bool,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_bool]);
        let topic = format!("{}/sig/sigBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }

    /// Publish the full initial state (retained).
    pub async fn publish_state(&self) -> Result<(), rumqttc::ClientError> {
        let state = json!({
            "propBool": self.impl_.prop_bool(),
            "propInt": self.impl_.prop_int()
        });
        let topic = format!("{}/state", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
}
