#![allow(unused_imports, dead_code, clippy::never_loop)]
use crate::api::empty_interface::EmptyInterfaceTrait;
use crate::core_types::empty_interface_data::EmptyInterfaceData;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.tb.simple.EmptyInterface";

/// NATS client adapter for EmptyInterface.
/// Implements the interface trait by forwarding operations via NATS request/reply
/// and caching property values locally.
pub struct EmptyInterfaceNatsClient {
    data: RwLock<EmptyInterfaceData>,
    client: async_nats::Client,
}

impl EmptyInterfaceNatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { data: RwLock::new(EmptyInterfaceData::default()), client }
    }

    /// Start background subscriptions for property changes, signals, and initial state.
    /// Returns a `JoinHandle` that runs until the client is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            let mut state_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.state")).await.expect("state subscription failed");
            loop {
                tokio::select! {
                    Some(msg) = state_sub.next() => {
                        this.handle_state(&msg);
                    }
                    else => break,
                }
            }
        })
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
