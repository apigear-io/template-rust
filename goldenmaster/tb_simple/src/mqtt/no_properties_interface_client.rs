use crate::api::{ApiError, ApiFuture};
use crate::api::no_properties_interface::NoPropertiesInterfacePublisher;
use crate::api::no_properties_interface::NoPropertiesInterfaceTrait;
use parking_lot::Mutex;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;

const TOPIC_PREFIX: &str = "tb.simple/NoPropertiesInterface";
const RPC_TIMEOUT: Duration = Duration::from_secs(5);

/// MQTT client adapter for NoPropertiesInterface.
/// Implements the interface trait using the agreed ApiGear (MQTT 5) wire scheme:
/// operations are published on `rpc/<op>` with an MQTT 5 `ResponseTopic` +
/// `CorrelationData` and the reply is awaited; property writes go to `set/<prop>`;
/// retained `prop/<prop>` notifications and `sig/<sig>` signals update local state.
pub struct NoPropertiesInterfaceMqttClient {
    client: Arc<AsyncClient>,
    client_id: String,
    next_correlation: std::sync::atomic::AtomicU64,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    publisher: NoPropertiesInterfacePublisher,
}

impl NoPropertiesInterfaceMqttClient {
    /// Create a new MQTT client adapter. `client_id` must be unique per client and
    /// is used to route RPC replies (`rpc/<op>/<client_id>/result`).
    pub fn new(
        client: Arc<AsyncClient>,
        client_id: impl Into<String>,
    ) -> Self {
        Self { client, client_id: client_id.into(), next_correlation: std::sync::atomic::AtomicU64::new(1), pending: Arc::new(Mutex::new(HashMap::new())), publisher: NoPropertiesInterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/sig/sigVoid", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcBool/{}/result", TOPIC_PREFIX, self.client_id), QoS::AtLeastOnce).await?;
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
        if let Some(sig_name) = suffix.strip_prefix("sig/") {
            self.handle_signal(sig_name, value);
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

impl NoPropertiesInterfaceTrait for NoPropertiesInterfaceMqttClient {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/funcVoid", TOPIC_PREFIX);
        // Void operation: fire-and-forget, no reply is requested or awaited.
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(request_topic, QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(())
        })
    }

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
        let args = json!([param_bool]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/funcBool", TOPIC_PREFIX);
        let id = self.next_correlation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().insert(id, tx);
        let pending = self.pending.clone();
        let response_topic = format!("{}/rpc/funcBool/{}/result", TOPIC_PREFIX, self.client_id);
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

    fn publisher(&self) -> &NoPropertiesInterfacePublisher {
        &self.publisher
    }
}
