use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple/NoOperationsInterface";

/// MQTT service adapter for NoOperationsInterface.
/// Bridges a local implementation to MQTT using the agreed ApiGear (MQTT 5) wire
/// scheme: operation requests on `rpc/<op>` (answered on the request's
/// `ResponseTopic` with `CorrelationData` echoed), property-change requests on
/// `set/<prop>`, retained change notifications on `prop/<prop>`, signals on
/// `sig/<sig>`.
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
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/set/propBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propInt", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        Ok(())
    }

    /// Handle an incoming MQTT message by dispatching to the appropriate handler.
    /// `response_topic` and `correlation_data` come from the MQTT 5 publish
    /// properties and route RPC replies back to the caller.
    pub fn handle_message(
        &self,
        topic: &str,
        payload: &[u8],
        _response_topic: Option<&str>,
        _correlation_data: Option<&[u8]>,
    ) {
        let suffix = topic.strip_prefix(&format!("{}/", TOPIC_PREFIX)).unwrap_or("");

        if let Some(prop_name) = suffix.strip_prefix("set/") {
            let value: Value = serde_json::from_slice(payload).unwrap_or_default();
            self.handle_set_property(prop_name, value);
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
    pub async fn publish_prop_bool_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_bool());
        let topic = format!("{}/prop/propBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propInt property change over MQTT (retained).
    pub async fn publish_prop_int_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_int());
        let topic = format!("{}/prop/propInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    pub async fn publish_sig_void(&self) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([]);
        let topic = format!("{}/sig/sigVoid", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_bool(
        &self,
        param_bool: bool,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_bool]);
        let topic = format!("{}/sig/sigBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }

    /// Re-publish the current value of every property (retained) so newly
    /// connected clients receive the latest state. The MQTT scheme has no
    /// dedicated state topic; retained `prop/<name>` messages carry the state.
    pub async fn publish_current_state(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.publish_prop_bool_changed().await?;
        self.publish_prop_int_changed().await?;
        Ok(())
    }
}
