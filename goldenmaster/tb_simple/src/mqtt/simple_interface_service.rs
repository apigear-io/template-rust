use crate::api::simple_interface::SimpleInterfaceTrait;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/tb.simple/SimpleInterface";

/// MQTT service adapter for SimpleInterface.
/// Bridges a local implementation to MQTT by subscribing to operation requests
/// and publishing property changes and signals.
pub struct SimpleInterfaceMqttService {
    impl_: Arc<dyn SimpleInterfaceTrait>,
    client: Arc<AsyncClient>,
}

impl SimpleInterfaceMqttService {
    pub fn new(
        impl_: Arc<dyn SimpleInterfaceTrait>,
        client: Arc<AsyncClient>,
    ) -> Self {
        Self { impl_, client }
    }

    /// Subscribe to all relevant MQTT topics for this service.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(format!("{}/op/funcNoReturnValue/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcNoParams/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcBool/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcInt/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcInt32/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcInt64/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcFloat/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcFloat32/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcFloat64/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/op/funcString/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propBool", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt32", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt64", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propFloat", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propFloat32", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propFloat64", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propString", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
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
            "funcNoReturnValue" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_no_return_value(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcNoReturnValue/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcNoParams" => {
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_no_params())));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcNoParams/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcBool" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_bool(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcBool/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcInt" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcInt/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcInt32" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int32(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcInt32/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcInt64" => {
                let param_0: i64 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int64(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcInt64/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcFloat" => {
                let param_0: f32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcFloat/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcFloat32" => {
                let param_0: f32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float32(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcFloat32/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcFloat64" => {
                let param_0: f64 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float64(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcFloat64/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
            "funcString" => {
                let param_0: String = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_string(param_0.as_str()))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/funcString/resp", TOPIC_PREFIX);
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
            "propInt32" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop_int32(v);
                }
            }
            "propInt64" => {
                if let Ok(v) = serde_json::from_value::<i64>(value) {
                    self.impl_.set_prop_int64(v);
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<f32>(value) {
                    self.impl_.set_prop_float(v);
                }
            }
            "propFloat32" => {
                if let Ok(v) = serde_json::from_value::<f32>(value) {
                    self.impl_.set_prop_float32(v);
                }
            }
            "propFloat64" => {
                if let Ok(v) = serde_json::from_value::<f64>(value) {
                    self.impl_.set_prop_float64(v);
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<String>(value) {
                    self.impl_.set_prop_string(&v);
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", property_name);
            }
        }
    }
    /// Publish propBool property change over MQTT (retained).
    pub async fn publish_prop_bool_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_bool());
        let topic = format!("{}/prop/propBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propInt property change over MQTT (retained).
    pub async fn publish_prop_int_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_int());
        let topic = format!("{}/prop/propInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propInt32 property change over MQTT (retained).
    pub async fn publish_prop_int32_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_int32());
        let topic = format!("{}/prop/propInt32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propInt64 property change over MQTT (retained).
    pub async fn publish_prop_int64_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_int64());
        let topic = format!("{}/prop/propInt64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propFloat property change over MQTT (retained).
    pub async fn publish_prop_float_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_float());
        let topic = format!("{}/prop/propFloat", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propFloat32 property change over MQTT (retained).
    pub async fn publish_prop_float32_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_float32());
        let topic = format!("{}/prop/propFloat32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propFloat64 property change over MQTT (retained).
    pub async fn publish_prop_float64_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_float64());
        let topic = format!("{}/prop/propFloat64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propString property change over MQTT (retained).
    pub async fn publish_prop_string_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.prop_string());
        let topic = format!("{}/prop/propString", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
    pub async fn publish_sig_bool(
        &self,
        param_bool: bool,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_bool]);
        let topic = format!("{}/sig/sigBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_int(
        &self,
        param_int: i32,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_int]);
        let topic = format!("{}/sig/sigInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_int32(
        &self,
        param_int32: i32,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_int32]);
        let topic = format!("{}/sig/sigInt32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_int64(
        &self,
        param_int64: i64,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_int64]);
        let topic = format!("{}/sig/sigInt64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_float(
        &self,
        param_float: f32,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_float]);
        let topic = format!("{}/sig/sigFloat", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_float32(
        &self,
        param_float32: f32,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_float32]);
        let topic = format!("{}/sig/sigFloat32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_float64(
        &self,
        param_float64: f64,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_float64]);
        let topic = format!("{}/sig/sigFloat64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_string(
        &self,
        param_string: &str,
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([param_string]);
        let topic = format!("{}/sig/sigString", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }

    /// Publish the full initial state (retained).
    pub async fn publish_state(&self) -> Result<(), rumqttc::ClientError> {
        let state = json!({
            "propBool": self.impl_.prop_bool(),
            "propInt": self.impl_.prop_int(),
            "propInt32": self.impl_.prop_int32(),
            "propInt64": self.impl_.prop_int64(),
            "propFloat": self.impl_.prop_float(),
            "propFloat32": self.impl_.prop_float32(),
            "propFloat64": self.impl_.prop_float64(),
            "propString": self.impl_.prop_string()
        });
        let topic = format!("{}/state", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
}
