#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use crate::api::same_enum1_interface::SameEnum1InterfacePublisher;
use crate::api::same_enum1_interface::SameEnum1InterfaceTrait;
use crate::core_types::same_enum1_interface_data::SameEnum1InterfaceData;
use parking_lot::RwLock;
use parking_lot::Mutex;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;

const TOPIC_PREFIX: &str = "tb.same1/SameEnum1Interface";
const RPC_TIMEOUT: Duration = Duration::from_secs(5);

/// MQTT client adapter for SameEnum1Interface.
/// Implements the interface trait using the agreed ApiGear (MQTT 5) wire scheme:
/// operations are published on `rpc/<op>` with an MQTT 5 `ResponseTopic` +
/// `CorrelationData` and the reply is awaited; property writes go to `set/<prop>`;
/// retained `prop/<prop>` notifications and `sig/<sig>` signals update local state.
pub struct SameEnum1InterfaceMqttClient {
    data: RwLock<SameEnum1InterfaceData>,
    client: Arc<AsyncClient>,
    client_id: String,
    next_correlation: std::sync::atomic::AtomicU64,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    publisher: SameEnum1InterfacePublisher,
}

impl SameEnum1InterfaceMqttClient {
    /// Create a new MQTT client adapter. `client_id` must be unique per client and
    /// is used to route RPC replies (`rpc/<op>/<client_id>/result`).
    pub fn new(
        client: Arc<AsyncClient>,
        client_id: impl Into<String>,
    ) -> Self {
        Self { data: RwLock::new(SameEnum1InterfaceData::default()), client, client_id: client_id.into(), next_correlation: std::sync::atomic::AtomicU64::new(1), pending: Arc::new(Mutex::new(HashMap::new())), publisher: SameEnum1InterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/prop/prop1", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sig1", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/func1/{}/result", TOPIC_PREFIX, self.client_id), QoS::AtLeastOnce).await?;
        Ok(())
    }

    /// Handle an incoming MQTT message by dispatching to the appropriate handler.
    /// `correlation_data` (from the MQTT 5 publish properties) routes RPC replies.
    pub fn handle_message(
        &self,
        topic: &str,
        payload: &[u8],
        correlation_data: Option<&[u8]>,
    ) {
        let suffix = topic.strip_prefix(&format!("{}/", TOPIC_PREFIX)).unwrap_or("");

        if suffix.starts_with("rpc/") {
            if let Some(id) = correlation_data.and_then(|b| std::str::from_utf8(b).ok()).and_then(|s| s.parse::<u64>().ok()) {
                if let Some(tx) = self.pending.lock().remove(&id) {
                    let value: Value = serde_json::from_slice(payload).unwrap_or_default();
                    let _ = tx.send(value);
                }
            }
            return;
        }
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
            "prop1" => {
                if let Ok(v) = serde_json::from_value::<Enum1Enum>(value) {
                    let _ = self.publisher.prop1_changed.send(v);
                    self.data.write().prop1 = v;
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
            _ => {
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl SameEnum1InterfaceTrait for SameEnum1InterfaceMqttClient {
    fn func1(
        &self,
        param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        let args = json!([param1]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/func1", TOPIC_PREFIX);
        let id = self.next_correlation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().insert(id, tx);
        let pending = self.pending.clone();
        let response_topic = format!("{}/rpc/func1/{}/result", TOPIC_PREFIX, self.client_id);
        Box::pin(async move {
            let props = PublishProperties { response_topic: Some(response_topic), correlation_data: Some(id.to_string().into()), ..Default::default() };
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            if let Err(e) = client.publish_with_properties(request_topic, QoS::AtLeastOnce, false, payload, props).await {
                pending.lock().remove(&id);
                return Err(ApiError::OperationFailed(e.to_string()));
            }
            match tokio::time::timeout(RPC_TIMEOUT, rx).await {
                Ok(Ok(value)) => Ok(serde_json::from_value(value).unwrap_or_default()),
                _ => {
                    pending.lock().remove(&id);
                    Err(ApiError::OperationFailed("rpc reply timed out".to_string()))
                }
            }
        })
    }

    fn prop1(&self) -> Enum1Enum {
        self.data.read().prop1
    }
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/prop1", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop1)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn publisher(&self) -> &SameEnum1InterfacePublisher {
        &self.publisher
    }
}
