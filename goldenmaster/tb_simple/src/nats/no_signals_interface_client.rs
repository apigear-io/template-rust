use crate::api::{ApiError, ApiFuture};
use crate::api::no_signals_interface::NoSignalsInterfacePublisher;
use crate::api::no_signals_interface::NoSignalsInterfaceTrait;
use crate::core_types::no_signals_interface_data::NoSignalsInterfaceData;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple.NoSignalsInterface";

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

/// NATS client adapter for NoSignalsInterface.
/// Implements the interface trait using the agreed ApiGear wire scheme:
/// operations via `rpc.<op>` request/reply, property writes via `set.<prop>`,
/// change notifications on `prop.<prop>`, signals on `sig.<sig>`, and an `init`
/// handshake (replied on `init.resp.<clientId>`) to fetch the current state.
pub struct NoSignalsInterfaceNatsClient {
    data: RwLock<NoSignalsInterfaceData>,
    client: async_nats::Client,
    client_id: u64,
    publisher: NoSignalsInterfacePublisher,
}

impl NoSignalsInterfaceNatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { data: RwLock::new(NoSignalsInterfaceData::default()), client, client_id: new_client_id(), publisher: NoSignalsInterfacePublisher::default() }
    }

    /// Start background subscriptions (notifications, signals, availability, init
    /// reply) and request the current state. Returns a `JoinHandle` that runs
    /// until the client is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            let mut prop_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.prop.*")).await.expect("property subscription failed");
            let mut avail_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.service.available")).await.expect("availability subscription failed");
            let mut init_resp_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.init.resp.{}", this.client_id)).await.expect("init-reply subscription failed");
            // Ask the service for the current state right away.
            this.request_init().await;
            loop {
                tokio::select! {
                    Some(msg) = prop_sub.next() => {
                        this.handle_property_change(&msg);
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
                if let Ok(v) = serde_json::from_value::<bool>(payload) {
                    let _ = self.publisher.prop_bool_changed.send(v);
                    self.data.write().prop_bool = v;
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<i32>(payload) {
                    let _ = self.publisher.prop_int_changed.send(v);
                    self.data.write().prop_int = v;
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", subject);
            }
        }
    }

    fn handle_state(
        &self,
        msg: &async_nats::Message,
    ) {
        if let Ok(data) = serde_json::from_slice::<NoSignalsInterfaceData>(&msg.payload) {
            *self.data.write() = data;
        }
    }
}

impl NoSignalsInterfaceTrait for NoSignalsInterfaceNatsClient {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.funcVoid"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
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

    fn prop_bool(&self) -> bool {
        self.data.read().prop_bool
    }
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    ) {
        let payload = serde_json::to_vec(&json!(prop_bool)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.propBool"), payload.into()).await;
        });
    }

    fn prop_int(&self) -> i32 {
        self.data.read().prop_int
    }
    fn set_prop_int(
        &self,
        prop_int: i32,
    ) {
        let payload = serde_json::to_vec(&json!(prop_int)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.propInt"), payload.into()).await;
        });
    }

    fn publisher(&self) -> &NoSignalsInterfacePublisher {
        &self.publisher
    }
}
