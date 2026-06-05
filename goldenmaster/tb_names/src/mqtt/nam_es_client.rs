#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use crate::api::nam_es::NamEsPublisher;
use crate::api::nam_es::NamEsTrait;
use crate::core_types::nam_es_data::NamEsData;
use parking_lot::RwLock;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.names/Nam_Es";

/// MQTT client adapter for NamEs.
/// Implements the interface trait using the agreed ApiGear (MQTT 5) wire scheme:
/// operations are published on `rpc/<op>` with an MQTT 5 `ResponseTopic` +
/// `CorrelationData` and the reply is awaited; property writes go to `set/<prop>`;
/// retained `prop/<prop>` notifications and `sig/<sig>` signals update local state.
pub struct NamEsMqttClient {
    data: RwLock<NamEsData>,
    client: Arc<AsyncClient>,
    publisher: NamEsPublisher,
}

impl NamEsMqttClient {
    /// Create a new MQTT client adapter. `client_id` must be unique per client and
    /// is used to route RPC replies (`rpc/<op>/<client_id>/result`).
    pub fn new(
        client: Arc<AsyncClient>,
        _client_id: impl Into<String>,
    ) -> Self {
        Self { data: RwLock::new(NamEsData::default()), client, publisher: NamEsPublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/prop/Switch", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/SOME_PROPERTY", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/Some_Poperty2", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/enum_property", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/SOME_SIGNAL", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/Some_Signal2", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
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
            "Switch" => {
                if let Ok(v) = serde_json::from_value::<bool>(value) {
                    let _ = self.publisher.switch_changed.send(v);
                    self.data.write().switch = v;
                }
            }
            "SOME_PROPERTY" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    let _ = self.publisher.some_property_changed.send(v);
                    self.data.write().some_property = v;
                }
            }
            "Some_Poperty2" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    let _ = self.publisher.some_poperty2_changed.send(v);
                    self.data.write().some_poperty2 = v;
                }
            }
            "enum_property" => {
                if let Ok(v) = serde_json::from_value::<Enum_With_Under_scoresEnum>(value) {
                    let _ = self.publisher.enum_property_changed.send(v);
                    self.data.write().enum_property = v;
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
            "SOME_SIGNAL" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.some_signal.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "Some_Signal2" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.some_signal2.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            _ => {
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl NamEsTrait for NamEsMqttClient {
    fn some_function(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([some_param]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/SOME_FUNCTION", TOPIC_PREFIX);
        // Void operation: fire-and-forget, no reply is requested or awaited.
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(request_topic, QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(())
        })
    }

    fn some_function2(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([some_param]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/Some_Function2", TOPIC_PREFIX);
        // Void operation: fire-and-forget, no reply is requested or awaited.
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(request_topic, QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(())
        })
    }

    fn switch(&self) -> bool {
        self.data.read().switch
    }
    fn set_switch(
        &self,
        switch: bool,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/Switch", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(switch)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn some_property(&self) -> i32 {
        self.data.read().some_property
    }
    fn set_some_property(
        &self,
        some_property: i32,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/SOME_PROPERTY", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(some_property)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn some_poperty2(&self) -> i32 {
        self.data.read().some_poperty2
    }
    fn set_some_poperty2(
        &self,
        some_poperty2: i32,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/Some_Poperty2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(some_poperty2)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn enum_property(&self) -> Enum_With_Under_scoresEnum {
        self.data.read().enum_property
    }
    fn set_enum_property(
        &self,
        enum_property: Enum_With_Under_scoresEnum,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/enum_property", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(enum_property)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn publisher(&self) -> &NamEsPublisher {
        &self.publisher
    }
}
