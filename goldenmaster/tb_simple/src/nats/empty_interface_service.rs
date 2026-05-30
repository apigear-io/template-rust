#![allow(unused_imports, dead_code, clippy::never_loop)]
use crate::api::empty_interface::EmptyInterfaceTrait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.tb.simple.EmptyInterface";

/// NATS service adapter for EmptyInterface.
/// Bridges a local implementation to NATS by subscribing to operation and
/// set-property subjects, and publishing property changes and signals.
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

    /// Start background subscriptions for operations and set-property requests.
    /// Returns a `JoinHandle` that runs until the service is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    else => break,
                }
            }
        })
    }

    /// Publish the current state of all properties.
    pub async fn publish_state(&self) {
        let state = json!({});
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.state"), payload.into()).await;
    }
}
