use crate::api::no_operations_interface::NoOperationsInterfacePublisher;
use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use crate::core_types::no_operations_interface_data::NoOperationsInterfaceData;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.tb.simple.NoOperationsInterface";

/// NATS client adapter for NoOperationsInterface.
/// Implements the interface trait by forwarding operations via NATS request/reply
/// and caching property values locally.
pub struct NoOperationsInterfaceNatsClient {
    data: RwLock<NoOperationsInterfaceData>,
    client: async_nats::Client,
    publisher: NoOperationsInterfacePublisher,
}

impl NoOperationsInterfaceNatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { data: RwLock::new(NoOperationsInterfaceData::default()), client, publisher: NoOperationsInterfacePublisher::default() }
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

    #[allow(clippy::get_first)]
    fn handle_signal(
        &self,
        msg: &async_nats::Message,
    ) {
        let subject = msg.subject.as_str();
        let member = subject.rsplit('.').next().unwrap_or("");
        let args: Value = serde_json::from_slice(&msg.payload).unwrap_or_default();
        match member {
            "sigVoid" => {
                let _ = self.publisher.sig_void.send(());
            }
            "sigBool" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig_bool.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
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
        if let Ok(data) = serde_json::from_slice::<NoOperationsInterfaceData>(&msg.payload) {
            *self.data.write() = data;
        }
    }
}

impl NoOperationsInterfaceTrait for NoOperationsInterfaceNatsClient {
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
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.propBool"), payload.into()).await;
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
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.propInt"), payload.into()).await;
        });
    }

    fn publisher(&self) -> &NoOperationsInterfacePublisher {
        &self.publisher
    }
}
