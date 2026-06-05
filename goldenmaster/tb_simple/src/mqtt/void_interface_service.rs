use crate::api::void_interface::VoidInterfaceTrait;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple/VoidInterface";

/// MQTT service adapter for VoidInterface.
/// Bridges a local implementation to MQTT using the agreed ApiGear (MQTT 5) wire
/// scheme: operation requests on `rpc/<op>` (answered on the request's
/// `ResponseTopic` with `CorrelationData` echoed), property-change requests on
/// `set/<prop>`, retained change notifications on `prop/<prop>`, signals on
/// `sig/<sig>`.
pub struct VoidInterfaceMqttService {
    impl_: Arc<dyn VoidInterfaceTrait>,
    client: Arc<AsyncClient>,
}

impl VoidInterfaceMqttService {
    pub fn new(
        impl_: Arc<dyn VoidInterfaceTrait>,
        client: Arc<AsyncClient>,
    ) -> Self {
        Self { impl_, client }
    }

    /// Subscribe to all relevant MQTT topics for this service.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/rpc/funcVoid", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        Ok(())
    }

    /// Handle an incoming MQTT message by dispatching to the appropriate handler.
    /// `response_topic` and `correlation_data` come from the MQTT 5 publish
    /// properties and route RPC replies back to the caller.
    pub fn handle_message(
        &self,
        topic: &str,
        payload: &[u8],
        response_topic: Option<&str>,
        correlation_data: Option<&[u8]>,
    ) {
        let suffix = topic.strip_prefix(&format!("{}/", TOPIC_PREFIX)).unwrap_or("");

        if let Some(op_name) = suffix.strip_prefix("rpc/") {
            let value: Value = serde_json::from_slice(payload).unwrap_or_default();
            self.handle_invoke(op_name, value, response_topic, correlation_data);
            return;
        }
    }

    #[allow(clippy::get_first)]
    fn handle_invoke(
        &self,
        method_name: &str,
        args: Value,
        response_topic: Option<&str>,
        correlation_data: Option<&[u8]>,
    ) {
        #[allow(unused_variables)]
        let arr = args.as_array();
        let client = self.client.clone();
        let response_topic = response_topic.map(|s| s.to_string());
        let correlation_data = correlation_data.map(|b| b.to_vec());
        match method_name {
            "funcVoid" => {
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_void())));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            _ => {
                tracing::warn!("Unknown method: {}", method_name);
            }
        }
    }

    /// Publish an RPC result back to the caller's `ResponseTopic`, echoing its
    /// `CorrelationData`. No reply is sent when the caller did not request one
    /// (e.g. void operations).
    fn send_reply(
        &self,
        client: Arc<AsyncClient>,
        response_topic: Option<String>,
        correlation_data: Option<Vec<u8>>,
        result: Value,
    ) {
        let Some(response_topic) = response_topic else {
            return;
        };
        let props = PublishProperties { correlation_data: correlation_data.map(Into::into), ..Default::default() };
        let payload = serde_json::to_vec(&result).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish_with_properties(response_topic, QoS::AtLeastOnce, false, payload, props).await;
        });
    }
    pub async fn publish_sig_void(&self) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([]);
        let topic = format!("{}/sig/sigVoid", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }

    /// Re-publish the current value of every property (retained) so newly
    /// connected clients receive the latest state. The MQTT scheme has no
    /// dedicated state topic; retained `prop/<name>` messages carry the state.
    pub async fn publish_current_state(&self) -> Result<(), rumqttc::v5::ClientError> {
        Ok(())
    }
}
