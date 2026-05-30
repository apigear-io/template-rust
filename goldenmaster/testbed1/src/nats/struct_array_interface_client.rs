#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_array_interface::StructArrayInterfacePublisher;
use crate::api::struct_array_interface::StructArrayInterfaceTrait;
use crate::core_types::struct_array_interface_data::StructArrayInterfaceData;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.testbed1.StructArrayInterface";

/// NATS client adapter for StructArrayInterface.
/// Implements the interface trait by forwarding operations via NATS request/reply
/// and caching property values locally.
pub struct StructArrayInterfaceNatsClient {
    data: RwLock<StructArrayInterfaceData>,
    client: async_nats::Client,
    publisher: StructArrayInterfacePublisher,
}

impl StructArrayInterfaceNatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { data: RwLock::new(StructArrayInterfaceData::default()), client, publisher: StructArrayInterfacePublisher::default() }
    }

    /// Start background subscriptions for property changes, signals, and initial state.
    /// Returns a `JoinHandle` that runs until the client is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            let mut prop_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.prop.*")).await.expect("property subscription failed");
            let mut sig_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.sig.*")).await.expect("signal subscription failed");
            let mut state_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.state")).await.expect("state subscription failed");
            loop {
                tokio::select! {
                    Some(msg) = prop_sub.next() => {
                        this.handle_property_change(&msg);
                    }
                    Some(msg) = sig_sub.next() => {
                        this.handle_signal(&msg);
                    }
                    Some(msg) = state_sub.next() => {
                        this.handle_state(&msg);
                    }
                    else => break,
                }
            }
        })
    }

    fn handle_property_change(
        &self,
        msg: &async_nats::Message,
    ) {
        let subject = msg.subject.as_str();
        let member = subject.rsplit('.').next().unwrap_or("");
        let payload: Value = serde_json::from_slice(&msg.payload).unwrap_or_default();
        match member {
            "propBool" => {
                if let Ok(v) = serde_json::from_value::<Vec<StructBool>>(payload) {
                    let _ = self.publisher.prop_bool_changed.send(v.clone());
                    self.data.write().prop_bool = v;
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<Vec<StructInt>>(payload) {
                    let _ = self.publisher.prop_int_changed.send(v.clone());
                    self.data.write().prop_int = v;
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<Vec<StructFloat>>(payload) {
                    let _ = self.publisher.prop_float_changed.send(v.clone());
                    self.data.write().prop_float = v;
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<Vec<StructString>>(payload) {
                    let _ = self.publisher.prop_string_changed.send(v.clone());
                    self.data.write().prop_string = v;
                }
            }
            "propEnum" => {
                if let Ok(v) = serde_json::from_value::<Vec<Enum0Enum>>(payload) {
                    let _ = self.publisher.prop_enum_changed.send(v.clone());
                    self.data.write().prop_enum = v;
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", subject);
            }
        }
    }

    #[allow(clippy::get_first)]
    fn handle_signal(
        &self,
        msg: &async_nats::Message,
    ) {
        let subject = msg.subject.as_str();
        let member = subject.rsplit('.').next().unwrap_or("");
        let args: Value = serde_json::from_slice(&msg.payload).unwrap_or_default();
        match member {
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
            "sigEnum" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_enum.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            _ => {
                tracing::warn!("Unknown signal: {}", subject);
            }
        }
    }

    fn handle_state(
        &self,
        msg: &async_nats::Message,
    ) {
        if let Ok(data) = serde_json::from_slice::<StructArrayInterfaceData>(&msg.payload) {
            *self.data.write() = data;
        }
    }
}

impl StructArrayInterfaceTrait for StructArrayInterfaceNatsClient {
    fn func_bool(
        &self,
        param_bool: &[StructBool],
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>> {
        let args = json!([param_bool]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.funcBool"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn func_int(
        &self,
        param_int: &[StructInt],
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>> {
        let args = json!([param_int]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.funcInt"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn func_float(
        &self,
        param_float: &[StructFloat],
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>> {
        let args = json!([param_float]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.funcFloat"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn func_string(
        &self,
        param_string: &[StructString],
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>> {
        let args = json!([param_string]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.funcString"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn func_enum(
        &self,
        param_enum: &[Enum0Enum],
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>> {
        let args = json!([param_enum]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.funcEnum"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn prop_bool(&self) -> Vec<StructBool> {
        self.data.read().prop_bool.clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &[StructBool],
    ) {
        let payload = serde_json::to_vec(&json!(prop_bool)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.propBool"), payload.into()).await;
        });
    }

    fn prop_int(&self) -> Vec<StructInt> {
        self.data.read().prop_int.clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &[StructInt],
    ) {
        let payload = serde_json::to_vec(&json!(prop_int)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.propInt"), payload.into()).await;
        });
    }

    fn prop_float(&self) -> Vec<StructFloat> {
        self.data.read().prop_float.clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &[StructFloat],
    ) {
        let payload = serde_json::to_vec(&json!(prop_float)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.propFloat"), payload.into()).await;
        });
    }

    fn prop_string(&self) -> Vec<StructString> {
        self.data.read().prop_string.clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &[StructString],
    ) {
        let payload = serde_json::to_vec(&json!(prop_string)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.propString"), payload.into()).await;
        });
    }

    fn prop_enum(&self) -> Vec<Enum0Enum> {
        self.data.read().prop_enum.clone()
    }
    fn set_prop_enum(
        &self,
        prop_enum: &[Enum0Enum],
    ) {
        let payload = serde_json::to_vec(&json!(prop_enum)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.propEnum"), payload.into()).await;
        });
    }

    fn publisher(&self) -> &StructArrayInterfacePublisher {
        &self.publisher
    }
}
