#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use crate::api::struct_array2_interface::StructArray2InterfacePublisher;
use crate::api::struct_array2_interface::StructArray2InterfaceTrait;
use crate::core_types::struct_array2_interface_data::StructArray2InterfaceData;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "testbed1.StructArray2Interface";

/// Generate a process-unique **numeric** client id used to route the
/// `init.resp.<clientId>` handshake reply back to this client. It is a number
/// (not a string) and stays within the signed-32-bit range so it matches the
/// init payload the other ApiGear templates (e.g. C++) expect.
fn new_client_id() -> u64 {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let pid = std::process::id() as u64;
    pid.wrapping_mul(100_000).wrapping_add(n) % 2_000_000_000
}

/// NATS client adapter for StructArray2Interface.
/// Implements the interface trait using the agreed ApiGear wire scheme:
/// operations via `rpc.<op>` request/reply, property writes via `set.<prop>`,
/// change notifications on `prop.<prop>`, signals on `sig.<sig>`, and an `init`
/// handshake (replied on `init.resp.<clientId>`) to fetch the current state.
pub struct StructArray2InterfaceNatsClient {
    data: RwLock<StructArray2InterfaceData>,
    client: async_nats::Client,
    client_id: u64,
    publisher: StructArray2InterfacePublisher,
}

impl StructArray2InterfaceNatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { data: RwLock::new(StructArray2InterfaceData::default()), client, client_id: new_client_id(), publisher: StructArray2InterfacePublisher::default() }
    }

    /// Start background subscriptions (notifications, signals, availability, init
    /// reply) and request the current state. Returns a `JoinHandle` that runs
    /// until the client is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            let mut prop_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.prop.*")).await.expect("property subscription failed");
            let mut sig_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.sig.*")).await.expect("signal subscription failed");
            let mut avail_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.service.available")).await.expect("availability subscription failed");
            let mut init_resp_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.init.resp.{}", this.client_id)).await.expect("init-reply subscription failed");
            // Ask the service for the current state right away.
            this.request_init().await;
            loop {
                tokio::select! {
                    Some(msg) = prop_sub.next() => {
                        this.handle_property_change(&msg);
                    }
                    Some(msg) = sig_sub.next() => {
                        this.handle_signal(&msg);
                    }
                    Some(_msg) = avail_sub.next() => {
                        // The service (re)appeared: re-sync our state.
                        this.request_init().await;
                    }
                    Some(msg) = init_resp_sub.next() => {
                        this.handle_state(&msg);
                    }
                    else => break,
                }
            }
        })
    }

    /// Send an `init` request carrying our client id; the service replies the
    /// full state on `init.resp.<clientId>`.
    async fn request_init(&self) {
        let payload = serde_json::to_vec(&self.client_id).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.init"), payload.into()).await;
        let _ = self.client.flush().await;
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
                if let Ok(v) = serde_json::from_value::<StructBoolWithArray>(payload) {
                    let _ = self.publisher.prop_bool_changed.send(v.clone());
                    self.data.write().prop_bool = v;
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<StructIntWithArray>(payload) {
                    let _ = self.publisher.prop_int_changed.send(v.clone());
                    self.data.write().prop_int = v;
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<StructFloatWithArray>(payload) {
                    let _ = self.publisher.prop_float_changed.send(v.clone());
                    self.data.write().prop_float = v;
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<StructStringWithArray>(payload) {
                    let _ = self.publisher.prop_string_changed.send(v.clone());
                    self.data.write().prop_string = v;
                }
            }
            "propEnum" => {
                if let Ok(v) = serde_json::from_value::<StructEnumWithArray>(payload) {
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
            _ => {
                tracing::warn!("Unknown signal: {}", subject);
            }
        }
    }

    fn handle_state(
        &self,
        msg: &async_nats::Message,
    ) {
        if let Ok(data) = serde_json::from_slice::<StructArray2InterfaceData>(&msg.payload) {
            *self.data.write() = data;
        }
    }
}

impl StructArray2InterfaceTrait for StructArray2InterfaceNatsClient {
    fn func_bool(
        &self,
        param_bool: &StructBoolWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>> {
        let args = json!([param_bool]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.funcBool"), payload.into()).await {
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
        param_int: &StructIntWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>> {
        let args = json!([param_int]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.funcInt"), payload.into()).await {
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
        param_float: &StructFloatWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>> {
        let args = json!([param_float]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.funcFloat"), payload.into()).await {
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
        param_string: &StructStringWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>> {
        let args = json!([param_string]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.funcString"), payload.into()).await {
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
        param_enum: &StructEnumWithArray,
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>> {
        let args = json!([param_enum]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.funcEnum"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
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
        let payload = serde_json::to_vec(&json!(prop_bool)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.propBool"), payload.into()).await;
        });
    }

    fn prop_int(&self) -> StructIntWithArray {
        self.data.read().prop_int.clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &StructIntWithArray,
    ) {
        let payload = serde_json::to_vec(&json!(prop_int)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.propInt"), payload.into()).await;
        });
    }

    fn prop_float(&self) -> StructFloatWithArray {
        self.data.read().prop_float.clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &StructFloatWithArray,
    ) {
        let payload = serde_json::to_vec(&json!(prop_float)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.propFloat"), payload.into()).await;
        });
    }

    fn prop_string(&self) -> StructStringWithArray {
        self.data.read().prop_string.clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &StructStringWithArray,
    ) {
        let payload = serde_json::to_vec(&json!(prop_string)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.propString"), payload.into()).await;
        });
    }

    fn prop_enum(&self) -> StructEnumWithArray {
        self.data.read().prop_enum.clone()
    }
    fn set_prop_enum(
        &self,
        prop_enum: &StructEnumWithArray,
    ) {
        let payload = serde_json::to_vec(&json!(prop_enum)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.propEnum"), payload.into()).await;
        });
    }

    fn publisher(&self) -> &StructArray2InterfacePublisher {
        &self.publisher
    }
}
