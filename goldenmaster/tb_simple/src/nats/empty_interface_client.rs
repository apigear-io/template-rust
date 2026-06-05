#![allow(unused_imports, dead_code, clippy::never_loop)]
use crate::api::empty_interface::EmptyInterfaceTrait;
use crate::core_types::empty_interface_data::EmptyInterfaceData;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple.EmptyInterface";

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

/// NATS client adapter for EmptyInterface.
/// Implements the interface trait using the agreed ApiGear wire scheme:
/// operations via `rpc.<op>` request/reply, property writes via `set.<prop>`,
/// change notifications on `prop.<prop>`, signals on `sig.<sig>`, and an `init`
/// handshake (replied on `init.resp.<clientId>`) to fetch the current state.
pub struct EmptyInterfaceNatsClient {
    data: RwLock<EmptyInterfaceData>,
    client: async_nats::Client,
    client_id: u64,
}

impl EmptyInterfaceNatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { data: RwLock::new(EmptyInterfaceData::default()), client, client_id: new_client_id() }
    }

    /// Start background subscriptions (notifications, signals, availability, init
    /// reply) and request the current state. Returns a `JoinHandle` that runs
    /// until the client is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            let mut avail_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.service.available")).await.expect("availability subscription failed");
            let mut init_resp_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.init.resp.{}", this.client_id)).await.expect("init-reply subscription failed");
            // Ask the service for the current state right away.
            this.request_init().await;
            loop {
                tokio::select! {
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

    fn handle_state(
        &self,
        msg: &async_nats::Message,
    ) {
        if let Ok(data) = serde_json::from_slice::<EmptyInterfaceData>(&msg.payload) {
            *self.data.write() = data;
        }
    }
}

impl EmptyInterfaceTrait for EmptyInterfaceNatsClient {}
