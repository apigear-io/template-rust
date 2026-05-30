#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::nam_es::NamEsTrait;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/tb.names/Nam_Es";

/// MQTT service adapter for NamEs.
/// Bridges a local implementation to MQTT by subscribing to operation requests
/// and publishing property changes and signals.
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
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/op/SOME_FUNCTION/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/Some_Function2/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/Switch", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/SOME_PROPERTY", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/Some_Poperty2", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/enum_property", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
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
            "SOME_FUNCTION" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.some_function(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/SOME_FUNCTION/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "Some_Function2" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.some_function2(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/Some_Function2/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
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
    pub async fn publish_switch_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.switch());
        let topic = format!("{}/prop/Switch", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish SOME_PROPERTY property change over MQTT (retained).
    pub async fn publish_some_property_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.some_property());
        let topic = format!("{}/prop/SOME_PROPERTY", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish Some_Poperty2 property change over MQTT (retained).
    pub async fn publish_some_poperty2_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.some_poperty2());
        let topic = format!("{}/prop/Some_Poperty2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish enum_property property change over MQTT (retained).
    pub async fn publish_enum_property_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.enum_property());
        let topic = format!("{}/prop/enum_property", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    pub async fn publish_some_signal(
        &self,
        some_param: bool,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([some_param]);
        let topic = format!("{}/sig/SOME_SIGNAL", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_some_signal2(
        &self,
        some_param: bool,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([some_param]);
        let topic = format!("{}/sig/Some_Signal2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }

    /// Publish the full initial state (retained).
    pub async fn publish_state(&self) -> Result<(), rumqttc::ClientError> {
        let state = json!({
            "Switch": self.impl_.switch(),
            "SOME_PROPERTY": self.impl_.some_property(),
            "Some_Poperty2": self.impl_.some_poperty2(),
            "enum_property": self.impl_.enum_property()
        });
        let topic = format!("{}/state", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
}
