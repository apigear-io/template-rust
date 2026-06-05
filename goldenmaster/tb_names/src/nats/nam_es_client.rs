#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use crate::api::nam_es::NamEsPublisher;
use crate::api::nam_es::NamEsTrait;
use crate::core_types::nam_es_data::NamEsData;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.names.Nam_Es";

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

/// NATS client adapter for NamEs.
/// Implements the interface trait using the agreed ApiGear wire scheme:
/// operations via `rpc.<op>` request/reply, property writes via `set.<prop>`,
/// change notifications on `prop.<prop>`, signals on `sig.<sig>`, and an `init`
/// handshake (replied on `init.resp.<clientId>`) to fetch the current state.
pub struct NamEsNatsClient {
    data: RwLock<NamEsData>,
    client: async_nats::Client,
    client_id: u64,
    publisher: NamEsPublisher,
}

impl NamEsNatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { data: RwLock::new(NamEsData::default()), client, client_id: new_client_id(), publisher: NamEsPublisher::default() }
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
            "Switch" => {
                if let Ok(v) = serde_json::from_value::<bool>(payload) {
                    let _ = self.publisher.switch_changed.send(v);
                    self.data.write().switch = v;
                }
            }
            "SOME_PROPERTY" => {
                if let Ok(v) = serde_json::from_value::<i32>(payload) {
                    let _ = self.publisher.some_property_changed.send(v);
                    self.data.write().some_property = v;
                }
            }
            "Some_Poperty2" => {
                if let Ok(v) = serde_json::from_value::<i32>(payload) {
                    let _ = self.publisher.some_poperty2_changed.send(v);
                    self.data.write().some_poperty2 = v;
                }
            }
            "enum_property" => {
                if let Ok(v) = serde_json::from_value::<Enum_With_Under_scoresEnum>(payload) {
                    let _ = self.publisher.enum_property_changed.send(v);
                    self.data.write().enum_property = v;
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
            "SOME_SIGNAL" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.some_signal.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "Some_Signal2" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.some_signal2.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
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
        if let Ok(data) = serde_json::from_slice::<NamEsData>(&msg.payload) {
            *self.data.write() = data;
        }
    }
}

impl NamEsTrait for NamEsNatsClient {
    fn some_function(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([some_param]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.SOME_FUNCTION"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn some_function2(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        let args = json!([some_param]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.Some_Function2"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn switch(&self) -> bool {
        self.data.read().switch
    }
    fn set_switch(
        &self,
        switch: bool,
    ) {
        let payload = serde_json::to_vec(&json!(switch)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.Switch"), payload.into()).await;
        });
    }

    fn some_property(&self) -> i32 {
        self.data.read().some_property
    }
    fn set_some_property(
        &self,
        some_property: i32,
    ) {
        let payload = serde_json::to_vec(&json!(some_property)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.SOME_PROPERTY"), payload.into()).await;
        });
    }

    fn some_poperty2(&self) -> i32 {
        self.data.read().some_poperty2
    }
    fn set_some_poperty2(
        &self,
        some_poperty2: i32,
    ) {
        let payload = serde_json::to_vec(&json!(some_poperty2)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.Some_Poperty2"), payload.into()).await;
        });
    }

    fn enum_property(&self) -> Enum_With_Under_scoresEnum {
        self.data.read().enum_property
    }
    fn set_enum_property(
        &self,
        enum_property: Enum_With_Under_scoresEnum,
    ) {
        let payload = serde_json::to_vec(&json!(enum_property)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.enum_property"), payload.into()).await;
        });
    }

    fn publisher(&self) -> &NamEsPublisher {
        &self.publisher
    }
}
