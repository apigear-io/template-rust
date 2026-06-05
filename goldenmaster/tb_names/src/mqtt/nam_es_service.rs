#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::nam_es::NamEsTrait;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.names/Nam_Es";

/// MQTT service adapter for NamEs.
/// Bridges a local implementation to MQTT using the agreed ApiGear (MQTT 5) wire
/// scheme: operation requests on `rpc/<op>` (answered on the request's
/// `ResponseTopic` with `CorrelationData` echoed), property-change requests on
/// `set/<prop>`, retained change notifications on `prop/<prop>`, signals on
/// `sig/<sig>`.
pub struct NamEsMqttService {
    impl_: Arc<dyn NamEsTrait>,
    client: Arc<AsyncClient>,
}

impl NamEsMqttService {
    pub fn new(
        impl_: Arc<dyn NamEsTrait>,
        client: Arc<AsyncClient>,
    ) -> Self {
        Self { impl_, client }
    }

    /// Subscribe to all relevant MQTT topics for this service.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/rpc/SOME_FUNCTION", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/Some_Function2", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/Switch", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/SOME_PROPERTY", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/Some_Poperty2", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/enum_property", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
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

        if let Some(prop_name) = suffix.strip_prefix("set/") {
            let value: Value = serde_json::from_slice(payload).unwrap_or_default();
            self.handle_set_property(prop_name, value);
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
            "SOME_FUNCTION" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.some_function(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "Some_Function2" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.some_function2(param_0))));
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

    fn handle_set_property(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
            "Switch" => {
                if let Ok(v) = serde_json::from_value::<bool>(value) {
                    self.impl_.set_switch(v);
                }
            }
            "SOME_PROPERTY" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_some_property(v);
                }
            }
            "Some_Poperty2" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_some_poperty2(v);
                }
            }
            "enum_property" => {
                if let Ok(v) = serde_json::from_value::<Enum_With_Under_scoresEnum>(value) {
                    self.impl_.set_enum_property(v);
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", property_name);
            }
        }
    }
    /// Publish Switch property change over MQTT (retained).
    pub async fn publish_switch_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.switch());
        let topic = format!("{}/prop/Switch", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish SOME_PROPERTY property change over MQTT (retained).
    pub async fn publish_some_property_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.some_property());
        let topic = format!("{}/prop/SOME_PROPERTY", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish Some_Poperty2 property change over MQTT (retained).
    pub async fn publish_some_poperty2_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.some_poperty2());
        let topic = format!("{}/prop/Some_Poperty2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish enum_property property change over MQTT (retained).
    pub async fn publish_enum_property_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.enum_property());
        let topic = format!("{}/prop/enum_property", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    pub async fn publish_some_signal(
        &self,
        some_param: bool,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([some_param]);
        let topic = format!("{}/sig/SOME_SIGNAL", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_some_signal2(
        &self,
        some_param: bool,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([some_param]);
        let topic = format!("{}/sig/Some_Signal2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }

    /// Re-publish the current value of every property (retained) so newly
    /// connected clients receive the latest state. The MQTT scheme has no
    /// dedicated state topic; retained `prop/<name>` messages carry the state.
    pub async fn publish_current_state(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.publish_switch_changed().await?;
        self.publish_some_property_changed().await?;
        self.publish_some_poperty2_changed().await?;
        self.publish_enum_property_changed().await?;
        Ok(())
    }
}
