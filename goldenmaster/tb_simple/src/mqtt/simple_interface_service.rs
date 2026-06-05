use crate::api::simple_interface::SimpleInterfaceTrait;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple/SimpleInterface";

/// MQTT service adapter for SimpleInterface.
/// Bridges a local implementation to MQTT using the agreed ApiGear (MQTT 5) wire
/// scheme: operation requests on `rpc/<op>` (answered on the request's
/// `ResponseTopic` with `CorrelationData` echoed), property-change requests on
/// `set/<prop>`, retained change notifications on `prop/<prop>`, signals on
/// `sig/<sig>`.
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
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/rpc/funcNoReturnValue", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcNoParams", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcInt", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcInt32", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcInt64", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcFloat", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcFloat32", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcFloat64", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcString", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propInt", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propInt32", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propInt64", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propFloat", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propFloat32", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propFloat64", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propString", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
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
            "funcNoReturnValue" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_no_return_value(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcNoParams" => {
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_no_params())));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcBool" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_bool(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcInt" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcInt32" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int32(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcInt64" => {
                let param_0: i64 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int64(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcFloat" => {
                let param_0: f32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcFloat32" => {
                let param_0: f32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float32(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcFloat64" => {
                let param_0: f64 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float64(param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcString" => {
                let param_0: String = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_string(param_0.as_str()))));
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
    pub async fn publish_prop_bool_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_bool());
        let topic = format!("{}/prop/propBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propInt property change over MQTT (retained).
    pub async fn publish_prop_int_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_int());
        let topic = format!("{}/prop/propInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propInt32 property change over MQTT (retained).
    pub async fn publish_prop_int32_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_int32());
        let topic = format!("{}/prop/propInt32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propInt64 property change over MQTT (retained).
    pub async fn publish_prop_int64_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_int64());
        let topic = format!("{}/prop/propInt64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propFloat property change over MQTT (retained).
    pub async fn publish_prop_float_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_float());
        let topic = format!("{}/prop/propFloat", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propFloat32 property change over MQTT (retained).
    pub async fn publish_prop_float32_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_float32());
        let topic = format!("{}/prop/propFloat32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propFloat64 property change over MQTT (retained).
    pub async fn publish_prop_float64_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_float64());
        let topic = format!("{}/prop/propFloat64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    /// Publish propString property change over MQTT (retained).
    pub async fn publish_prop_string_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_string());
        let topic = format!("{}/prop/propString", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
    pub async fn publish_sig_bool(
        &self,
        param_bool: bool,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_bool]);
        let topic = format!("{}/sig/sigBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_int(
        &self,
        param_int: i32,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_int]);
        let topic = format!("{}/sig/sigInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_int32(
        &self,
        param_int32: i32,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_int32]);
        let topic = format!("{}/sig/sigInt32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_int64(
        &self,
        param_int64: i64,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_int64]);
        let topic = format!("{}/sig/sigInt64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_float(
        &self,
        param_float: f32,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_float]);
        let topic = format!("{}/sig/sigFloat", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_float32(
        &self,
        param_float32: f32,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_float32]);
        let topic = format!("{}/sig/sigFloat32", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_float64(
        &self,
        param_float64: f64,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_float64]);
        let topic = format!("{}/sig/sigFloat64", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_string(
        &self,
        param_string: &str,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_string]);
        let topic = format!("{}/sig/sigString", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }

    /// Re-publish the current value of every property (retained) so newly
    /// connected clients receive the latest state. The MQTT scheme has no
    /// dedicated state topic; retained `prop/<name>` messages carry the state.
    pub async fn publish_current_state(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.publish_prop_bool_changed().await?;
        self.publish_prop_int_changed().await?;
        self.publish_prop_int32_changed().await?;
        self.publish_prop_int64_changed().await?;
        self.publish_prop_float_changed().await?;
        self.publish_prop_float32_changed().await?;
        self.publish_prop_float64_changed().await?;
        self.publish_prop_string_changed().await?;
        Ok(())
    }
}
