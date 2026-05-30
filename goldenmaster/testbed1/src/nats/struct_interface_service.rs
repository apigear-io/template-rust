#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::struct_interface::StructInterfaceTrait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.testbed1.StructInterface";

/// NATS service adapter for StructInterface.
/// Bridges a local implementation to NATS by subscribing to operation and
/// set-property subjects, and publishing property changes and signals.
pub struct StructInterfaceNatsService {
    impl_: Arc<dyn StructInterfaceTrait>,
    client: async_nats::Client,
}

impl StructInterfaceNatsService {
    pub fn new(
        impl_: Arc<dyn StructInterfaceTrait>,
        client: async_nats::Client,
    ) -> Self {
        Self { impl_, client }
    }

    /// Start background subscriptions for operations and set-property requests.
    /// Returns a `JoinHandle` that runs until the service is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            let mut op_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.op.*")).await.expect("operation subscription failed");
            let mut prop_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.prop.*")).await.expect("property subscription failed");
            loop {
                tokio::select! {
                    Some(msg) = op_sub.next() => {
                        this.handle_operation(msg).await;
                    }
                    Some(msg) = prop_sub.next() => {
                        this.handle_set_property(&msg);
                    }
                    else => break,
                }
            }
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
            "funcBool" => {
                let param_0: StructBool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_bool(&param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcInt" => {
                let param_0: StructInt = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_int(&param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat" => {
                let param_0: StructFloat = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_float(&param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcString" => {
                let param_0: StructString = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_string(&param_0).await {
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
            "propBool" => {
                if let Ok(v) = serde_json::from_slice::<StructBool>(&msg.payload) {
                    self.impl_.set_prop_bool(&v);
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_slice::<StructInt>(&msg.payload) {
                    self.impl_.set_prop_int(&v);
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_slice::<StructFloat>(&msg.payload) {
                    self.impl_.set_prop_float(&v);
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_slice::<StructString>(&msg.payload) {
                    self.impl_.set_prop_string(&v);
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", subject);
            }
        }
    }

    /// Publish the current state of all properties.
    pub async fn publish_state(&self) {
        let state = json!({
            "propBool": self.impl_.prop_bool(),
            "propInt": self.impl_.prop_int(),
            "propFloat": self.impl_.prop_float(),
            "propString": self.impl_.prop_string()
        });
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.state"), payload.into()).await;
    }

    /// Publish a property change notification.
    pub async fn notify_property_changed(
        &self,
        property: &str,
        value: Value,
    ) {
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.prop.{property}"), payload.into()).await;
    }

    /// Publish a signal.
    pub async fn notify_signal(
        &self,
        signal: &str,
        args: Value,
    ) {
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.sig.{signal}"), payload.into()).await;
    }
}
