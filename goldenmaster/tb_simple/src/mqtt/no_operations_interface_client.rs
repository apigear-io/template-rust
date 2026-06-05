use crate::api::no_operations_interface::NoOperationsInterfacePublisher;
use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use crate::core_types::no_operations_interface_data::NoOperationsInterfaceData;
use parking_lot::RwLock;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple/NoOperationsInterface";

/// MQTT client adapter for NoOperationsInterface.
/// Implements the interface trait using the agreed ApiGear (MQTT 5) wire scheme:
/// operations are published on `rpc/<op>` with an MQTT 5 `ResponseTopic` +
/// `CorrelationData` and the reply is awaited; property writes go to `set/<prop>`;
/// retained `prop/<prop>` notifications and `sig/<sig>` signals update local state.
pub struct NoOperationsInterfaceMqttClient {
    data: RwLock<NoOperationsInterfaceData>,
    client: Arc<AsyncClient>,
    publisher: NoOperationsInterfacePublisher,
}

impl NoOperationsInterfaceMqttClient {
    /// Create a new MQTT client adapter. `client_id` must be unique per client and
    /// is used to route RPC replies (`rpc/<op>/<client_id>/result`).
    pub fn new(
        client: Arc<AsyncClient>,
        _client_id: impl Into<String>,
    ) -> Self {
        Self { data: RwLock::new(NoOperationsInterfaceData::default()), client, publisher: NoOperationsInterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/prop/propBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigVoid", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        Ok(())
    }

    /// Handle an incoming MQTT message by dispatching to the appropriate handler.
    /// `correlation_data` (from the MQTT 5 publish properties) routes RPC replies.
    pub fn handle_message(
        &self,
        topic: &str,
        payload: &[u8],
        _correlation_data: Option<&[u8]>,
    ) {
        let suffix = topic.strip_prefix(&format!("{}/", TOPIC_PREFIX)).unwrap_or("");
        let value: Value = serde_json::from_slice(payload).unwrap_or_default();
        if let Some(prop_name) = suffix.strip_prefix("prop/") {
            self.handle_property_change(prop_name, value);
            return;
        }
        if let Some(sig_name) = suffix.strip_prefix("sig/") {
            self.handle_signal(sig_name, value);
        }
    }

    fn handle_property_change(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
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
            "sigBool" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_bool.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            _ => {
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl NoOperationsInterfaceTrait for NoOperationsInterfaceMqttClient {
    fn prop_bool(&self) -> bool {
        self.data.read().prop_bool
    }
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/propBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_bool)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn prop_int(&self) -> i32 {
        self.data.read().prop_int
    }
    fn set_prop_int(
        &self,
        prop_int: i32,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/propInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_int)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn publisher(&self) -> &NoOperationsInterfacePublisher {
        &self.publisher
    }
}
