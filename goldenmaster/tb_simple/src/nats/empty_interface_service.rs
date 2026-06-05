#![allow(unused_imports, dead_code, clippy::never_loop)]
use crate::api::empty_interface::EmptyInterfaceTrait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple.EmptyInterface";

/// NATS service adapter for EmptyInterface.
/// Bridges a local implementation to NATS using the agreed ApiGear wire scheme:
/// operation requests on `rpc.<op>` (request/reply), property-change requests on
/// `set.<prop>`, change notifications on `prop.<prop>`, signals on `sig.<sig>`,
/// an availability beacon on `service.available`, and an `init` handshake that
/// replies the current state on `init.resp.<clientId>`.
pub struct EmptyInterfaceNatsService {
    impl_: Arc<dyn EmptyInterfaceTrait>,
    client: async_nats::Client,
}

impl EmptyInterfaceNatsService {
    pub fn new(
        impl_: Arc<dyn EmptyInterfaceTrait>,
        client: async_nats::Client,
    ) -> Self {
        Self { impl_, client }
    }

    /// Start background subscriptions and announce availability.
    /// Returns a `JoinHandle` that runs until the service is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            let mut init_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.init")).await.expect("init subscription failed");
            // Now that we are subscribed, announce availability so clients (re-)sync.
            this.publish_service_available().await;
            loop {
                tokio::select! {
                    Some(msg) = init_sub.next() => {
                        this.handle_init(msg).await;
                    }
                    else => break,
                }
            }
        })
    }

    /// Publish the availability beacon (empty payload) so clients know the service is up.
    pub async fn publish_service_available(&self) {
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.service.available"), Vec::<u8>::new().into()).await;
        let _ = self.client.flush().await;
    }

    /// Answer an `init` handshake: reply the current state on `init.resp.<clientId>`.
    async fn handle_init(
        &self,
        msg: async_nats::Message,
    ) {
        let raw = String::from_utf8_lossy(&msg.payload);
        let trimmed = raw.trim();
        // The client id arrives either as a JSON string ("id") or a bare token.
        let client_id = serde_json::from_str::<String>(trimmed).unwrap_or_else(|_| trimmed.trim_matches('"').to_string());
        let payload = serde_json::to_vec(&self.current_state()).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.init.resp.{client_id}"), payload.into()).await;
        let _ = self.client.flush().await;
    }

    /// Snapshot all property values as a JSON object (the init-handshake payload).
    fn current_state(&self) -> Value {
        json!({})
    }
}
