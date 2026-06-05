#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::same_struct2_interface::SameStruct2InterfaceTrait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.same1.SameStruct2Interface";

/// NATS service adapter for SameStruct2Interface.
/// Bridges a local implementation to NATS using the agreed ApiGear wire scheme:
/// operation requests on `rpc.<op>` (request/reply), property-change requests on
/// `set.<prop>`, change notifications on `prop.<prop>`, signals on `sig.<sig>`,
/// an availability beacon on `service.available`, and an `init` handshake that
/// replies the current state on `init.resp.<clientId>`.
pub struct SameStruct2InterfaceNatsService {
    impl_: Arc<dyn SameStruct2InterfaceTrait>,
    client: async_nats::Client,
}

impl SameStruct2InterfaceNatsService {
    pub fn new(
        impl_: Arc<dyn SameStruct2InterfaceTrait>,
        client: async_nats::Client,
    ) -> Self {
        Self { impl_, client }
    }

    /// Start background subscriptions and announce availability.
    /// Returns a `JoinHandle` that runs until the service is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            let mut rpc_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.rpc.*")).await.expect("operation subscription failed");
            let mut set_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.set.*")).await.expect("set-property subscription failed");
            let mut init_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.init")).await.expect("init subscription failed");
            // Now that we are subscribed, announce availability so clients (re-)sync.
            this.publish_service_available().await;
            loop {
                tokio::select! {
                    Some(msg) = rpc_sub.next() => {
                        this.handle_operation(msg).await;
                    }
                    Some(msg) = set_sub.next() => {
                        this.handle_set_property(&msg);
                    }
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
        json!({
            "prop1": self.impl_.prop1(),
            "prop2": self.impl_.prop2()
        })
    }

    #[allow(clippy::get_first)]
    async fn handle_operation(
        &self,
        msg: async_nats::Message,
    ) {
        let subject = msg.subject.as_str();
        let member = subject.rsplit('.').next().unwrap_or("");
        #[allow(unused_variables)]
        let args: Value = serde_json::from_slice(&msg.payload).unwrap_or_default();
        #[allow(unused_variables)]
        let arr = args.as_array();
        let result = match member {
            "func1" => {
                let param_0: Struct1 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func1(&param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "func2" => {
                let param_0: Struct1 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: Struct2 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func2(&param_0, &param_1).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            _ => {
                tracing::warn!("Unknown operation: {}", subject);
                json!(null)
            }
        };
        if let Some(reply) = msg.reply {
            let payload = serde_json::to_vec(&result).unwrap_or_default();
            let _ = self.client.publish(reply, payload.into()).await;
        }
    }

    fn handle_set_property(
        &self,
        msg: &async_nats::Message,
    ) {
        let subject = msg.subject.as_str();
        let member = subject.rsplit('.').next().unwrap_or("");
        match member {
            "prop1" => {
                if let Ok(v) = serde_json::from_slice::<Struct2>(&msg.payload) {
                    self.impl_.set_prop1(&v);
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_slice::<Struct2>(&msg.payload) {
                    self.impl_.set_prop2(&v);
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", subject);
            }
        }
    }

    /// Publish a property change notification on `prop.<property>`.
    pub async fn notify_property_changed(
        &self,
        property: &str,
        value: Value,
    ) {
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.prop.{property}"), payload.into()).await;
    }

    /// Publish a signal on `sig.<signal>`.
    pub async fn notify_signal(
        &self,
        signal: &str,
        args: Value,
    ) {
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.sig.{signal}"), payload.into()).await;
    }
}
