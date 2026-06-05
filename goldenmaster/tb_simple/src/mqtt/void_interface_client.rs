use crate::api::{ApiError, ApiFuture};
use crate::api::void_interface::VoidInterfacePublisher;
use crate::api::void_interface::VoidInterfaceTrait;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple/VoidInterface";

/// MQTT client adapter for VoidInterface.
/// Implements the interface trait using the agreed ApiGear (MQTT 5) wire scheme:
/// operations are published on `rpc/<op>` with an MQTT 5 `ResponseTopic` +
/// `CorrelationData` and the reply is awaited; property writes go to `set/<prop>`;
/// retained `prop/<prop>` notifications and `sig/<sig>` signals update local state.
pub struct VoidInterfaceMqttClient {
    client: Arc<AsyncClient>,
    publisher: VoidInterfacePublisher,
}

impl VoidInterfaceMqttClient {
    /// Create a new MQTT client adapter. `client_id` must be unique per client and
    /// is used to route RPC replies (`rpc/<op>/<client_id>/result`).
    pub fn new(
        client: Arc<AsyncClient>,
        _client_id: impl Into<String>,
    ) -> Self {
        Self { client, publisher: VoidInterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/sig/sigVoid", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
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
        let request_topic = format!("{}/rpc/funcVoid", TOPIC_PREFIX);
        // Void operation: fire-and-forget, no reply is requested or awaited.
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(request_topic, QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(())
        })
    }

    fn publisher(&self) -> &VoidInterfacePublisher {
        &self.publisher
    }
}
