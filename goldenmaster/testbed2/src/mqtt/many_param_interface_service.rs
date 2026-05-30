#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::many_param_interface::ManyParamInterfaceTrait;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/testbed2/ManyParamInterface";

/// MQTT service adapter for ManyParamInterface.
/// Bridges a local implementation to MQTT by subscribing to operation requests
/// and publishing property changes and signals.
pub struct ManyParamInterfaceMqttService {
    impl_: Arc<dyn ManyParamInterfaceTrait>,
    client: Arc<AsyncClient>,
}

impl ManyParamInterfaceMqttService {
    pub fn new(
        impl_: Arc<dyn ManyParamInterfaceTrait>,
        client: Arc<AsyncClient>,
    ) -> Self {
        Self { impl_, client }
    }

    /// Subscribe to all relevant MQTT topics for this service.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/op/func1/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func2/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func3/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/func4/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/prop1", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/prop2", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/prop3", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/prop4", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
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
            "func1" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func1(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/func1/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "func2" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func2(param_0, param_1))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/func2/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "func3" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_2: i32 = serde_json::from_value(arr.and_then(|a| a.get(2).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func3(param_0, param_1, param_2))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/func3/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "func4" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_2: i32 = serde_json::from_value(arr.and_then(|a| a.get(2).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_3: i32 = serde_json::from_value(arr.and_then(|a| a.get(3).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func4(param_0, param_1, param_2, param_3))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/func4/resp", TOPIC_PREFIX);
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
            "prop1" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop1(v);
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop2(v);
                }
            }
            "prop3" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop3(v);
                }
            }
            "prop4" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop4(v);
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", property_name);
            }
        }
    }
    /// Publish prop1 property change over MQTT (retained).
    pub async fn publish_prop1_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop1());
        let topic = format!("{}/prop/prop1", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish prop2 property change over MQTT (retained).
    pub async fn publish_prop2_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop2());
        let topic = format!("{}/prop/prop2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish prop3 property change over MQTT (retained).
    pub async fn publish_prop3_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop3());
        let topic = format!("{}/prop/prop3", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish prop4 property change over MQTT (retained).
    pub async fn publish_prop4_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop4());
        let topic = format!("{}/prop/prop4", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    pub async fn publish_sig1(
        &self,
        param1: i32,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param1]);
        let topic = format!("{}/sig/sig1", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig2(
        &self,
        param1: i32,
        param2: i32,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param1, param2]);
        let topic = format!("{}/sig/sig2", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig3(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param1, param2, param3]);
        let topic = format!("{}/sig/sig3", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig4(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
        param4: i32,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param1, param2, param3, param4]);
        let topic = format!("{}/sig/sig4", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }

    /// Publish the full initial state (retained).
    pub async fn publish_state(&self) -> Result<(), rumqttc::ClientError> {
        let state = json!({
            "prop1": self.impl_.prop1(),
            "prop2": self.impl_.prop2(),
            "prop3": self.impl_.prop3(),
            "prop4": self.impl_.prop4()
        });
        let topic = format!("{}/state", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
}
