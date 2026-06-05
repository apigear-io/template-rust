#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use crate::api::struct_array2_interface::StructArray2InterfacePublisher;
use crate::api::struct_array2_interface::StructArray2InterfaceTrait;
use crate::core_types::struct_array2_interface_data::StructArray2InterfaceData;
use parking_lot::RwLock;
use parking_lot::Mutex;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;

const TOPIC_PREFIX: &str = "testbed1/StructArray2Interface";
const RPC_TIMEOUT: Duration = Duration::from_secs(5);

/// MQTT client adapter for StructArray2Interface.
/// Implements the interface trait using the agreed ApiGear (MQTT 5) wire scheme:
/// operations are published on `rpc/<op>` with an MQTT 5 `ResponseTopic` +
/// `CorrelationData` and the reply is awaited; property writes go to `set/<prop>`;
/// retained `prop/<prop>` notifications and `sig/<sig>` signals update local state.
pub struct StructArray2InterfaceMqttClient {
    data: RwLock<StructArray2InterfaceData>,
    client: Arc<AsyncClient>,
    client_id: String,
    next_correlation: std::sync::atomic::AtomicU64,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    publisher: StructArray2InterfacePublisher,
}

impl StructArray2InterfaceMqttClient {
    /// Create a new MQTT client adapter. `client_id` must be unique per client and
    /// is used to route RPC replies (`rpc/<op>/<client_id>/result`).
    pub fn new(
        client: Arc<AsyncClient>,
        client_id: impl Into<String>,
    ) -> Self {
        Self { data: RwLock::new(StructArray2InterfaceData::default()), client, client_id: client_id.into(), next_correlation: std::sync::atomic::AtomicU64::new(1), pending: Arc::new(Mutex::new(HashMap::new())), publisher: StructArray2InterfacePublisher::default() }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
        self.client.subscribe(format!("{}/prop/propBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propInt", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propFloat", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propString", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/prop/propEnum", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigBool", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigInt", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigFloat", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/sig/sigString", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcBool/{}/result", TOPIC_PREFIX, self.client_id), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcInt/{}/result", TOPIC_PREFIX, self.client_id), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcFloat/{}/result", TOPIC_PREFIX, self.client_id), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcString/{}/result", TOPIC_PREFIX, self.client_id), QoS::AtLeastOnce).await?;
        self.client.subscribe(format!("{}/rpc/funcEnum/{}/result", TOPIC_PREFIX, self.client_id), QoS::AtLeastOnce).await?;
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
            "propBool" => {
                if let Ok(v) = serde_json::from_value::<StructBoolWithArray>(value) {
                    let _ = self.publisher.prop_bool_changed.send(v.clone());
                    self.data.write().prop_bool = v;
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<StructIntWithArray>(value) {
                    let _ = self.publisher.prop_int_changed.send(v.clone());
                    self.data.write().prop_int = v;
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<StructFloatWithArray>(value) {
                    let _ = self.publisher.prop_float_changed.send(v.clone());
                    self.data.write().prop_float = v;
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<StructStringWithArray>(value) {
                    let _ = self.publisher.prop_string_changed.send(v.clone());
                    self.data.write().prop_string = v;
                }
            }
            "propEnum" => {
                if let Ok(v) = serde_json::from_value::<StructEnumWithArray>(value) {
                    let _ = self.publisher.prop_enum_changed.send(v.clone());
                    self.data.write().prop_enum = v;
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
            "sigBool" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_bool.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigInt" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_int.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigFloat" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_float.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sigString" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_string.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            _ => {
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
}

impl StructArray2InterfaceTrait for StructArray2InterfaceMqttClient {
    fn func_bool(
        &self,
        param_bool: &StructBoolWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>> {
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

    fn func_int(
        &self,
        param_int: &StructIntWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>> {
        let args = json!([param_int]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/funcInt", TOPIC_PREFIX);
        let id = self.next_correlation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().insert(id, tx);
        let pending = self.pending.clone();
        let response_topic = format!("{}/rpc/funcInt/{}/result", TOPIC_PREFIX, self.client_id);
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

    fn func_float(
        &self,
        param_float: &StructFloatWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>> {
        let args = json!([param_float]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/funcFloat", TOPIC_PREFIX);
        let id = self.next_correlation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().insert(id, tx);
        let pending = self.pending.clone();
        let response_topic = format!("{}/rpc/funcFloat/{}/result", TOPIC_PREFIX, self.client_id);
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

    fn func_string(
        &self,
        param_string: &StructStringWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>> {
        let args = json!([param_string]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/funcString", TOPIC_PREFIX);
        let id = self.next_correlation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().insert(id, tx);
        let pending = self.pending.clone();
        let response_topic = format!("{}/rpc/funcString/{}/result", TOPIC_PREFIX, self.client_id);
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

    fn func_enum(
        &self,
        param_enum: &StructEnumWithArray,
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>> {
        let args = json!([param_enum]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/funcEnum", TOPIC_PREFIX);
        let id = self.next_correlation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().insert(id, tx);
        let pending = self.pending.clone();
        let response_topic = format!("{}/rpc/funcEnum/{}/result", TOPIC_PREFIX, self.client_id);
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

    fn prop_bool(&self) -> StructBoolWithArray {
        self.data.read().prop_bool.clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &StructBoolWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/propBool", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_bool)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn prop_int(&self) -> StructIntWithArray {
        self.data.read().prop_int.clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &StructIntWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/propInt", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_int)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn prop_float(&self) -> StructFloatWithArray {
        self.data.read().prop_float.clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &StructFloatWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/propFloat", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_float)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn prop_string(&self) -> StructStringWithArray {
        self.data.read().prop_string.clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &StructStringWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/propString", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_string)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn prop_enum(&self) -> StructEnumWithArray {
        self.data.read().prop_enum.clone()
    }
    fn set_prop_enum(
        &self,
        prop_enum: &StructEnumWithArray,
    ) {
        let client = self.client.clone();
        let topic = format!("{}/set/propEnum", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!(prop_enum)).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
        });
    }

    fn publisher(&self) -> &StructArray2InterfacePublisher {
        &self.publisher
    }
}
