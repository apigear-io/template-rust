#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::struct_interface::StructInterfaceTrait;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "testbed1/StructInterface";

/// MQTT service adapter for StructInterface.
/// Bridges a local implementation to MQTT using the agreed ApiGear (MQTT 5) wire
/// scheme: operation requests on `rpc/<op>` (answered on the request's
/// `ResponseTopic` with `CorrelationData` echoed), property-change requests on
/// `set/<prop>`, retained change notifications on `prop/<prop>`, signals on
/// `sig/<sig>`.
pub struct StructInterfaceMqttService {
    impl_: Arc<dyn StructInterfaceTrait>,
    client: Arc<AsyncClient>,
}

impl StructInterfaceMqttService {
    pub fn new(
        impl_: Arc<dyn StructInterfaceTrait>,
        client: Arc<AsyncClient>,
    ) -> Self {
        Self { impl_, client }
    }

    /// Subscribe to all relevant MQTT topics for this service.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/rpc/funcBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcInt", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcFloat", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcString", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propInt", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/set/propFloat", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
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
            "funcBool" => {
                let param_0: StructBool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_bool(&param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcInt" => {
                let param_0: StructInt = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int(&param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcFloat" => {
                let param_0: StructFloat = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float(&param_0))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                self.send_reply(client, response_topic, correlation_data, result);
            }
            "funcString" => {
                let param_0: StructString = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_string(&param_0))));
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
                if let Ok(v) = serde_json::from_value::<StructBool>(value) {
                    self.impl_.set_prop_bool(&v);
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<StructInt>(value) {
                    self.impl_.set_prop_int(&v);
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<StructFloat>(value) {
                    self.impl_.set_prop_float(&v);
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<StructString>(value) {
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
    /// Publish propFloat property change over MQTT (retained).
    pub async fn publish_prop_float_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.prop_float());
        let topic = format!("{}/prop/propFloat", TOPIC_PREFIX);
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
        param_bool: &StructBool,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_bool]);
        let topic = format!("{}/sig/sigBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_int(
        &self,
        param_int: &StructInt,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_int]);
        let topic = format!("{}/sig/sigInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_float(
        &self,
        param_float: &StructFloat,
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([param_float]);
        let topic = format!("{}/sig/sigFloat", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
    pub async fn publish_sig_string(
        &self,
        param_string: &StructString,
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
        self.publish_prop_float_changed().await?;
        self.publish_prop_string_changed().await?;
        Ok(())
    }
}
