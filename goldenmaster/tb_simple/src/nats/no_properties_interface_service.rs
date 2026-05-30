use crate::api::no_properties_interface::NoPropertiesInterfaceTrait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.tb.simple.NoPropertiesInterface";

/// NATS service adapter for NoPropertiesInterface.
/// Bridges a local implementation to NATS by subscribing to operation and
/// set-property subjects, and publishing property changes and signals.
pub struct NoPropertiesInterfaceNatsService {
    impl_: Arc<dyn NoPropertiesInterfaceTrait>,
    client: async_nats::Client,
}

impl NoPropertiesInterfaceNatsService {
    pub fn new(
        impl_: Arc<dyn NoPropertiesInterfaceTrait>,
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
            loop {
                tokio::select! {
                    Some(msg) = op_sub.next() => {
                        this.handle_operation(msg).await;
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
            "funcVoid" => match self.impl_.func_void().await {
                Ok(value) => json!(value),
                _ => json!(null),
            },
            "funcBool" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_bool(param_0).await {
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

    /// Publish the current state of all properties.
    pub async fn publish_state(&self) {
        let state = json!({});
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.state"), payload.into()).await;
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
