#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::nam_es::NamEsTrait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.tb.names.Nam_Es";

/// NATS service adapter for NamEs.
/// Bridges a local implementation to NATS by subscribing to operation and
/// set-property subjects, and publishing property changes and signals.
pub struct NamEsNatsService {
    impl_: Arc<dyn NamEsTrait>,
    client: async_nats::Client,
}

impl NamEsNatsService {
    pub fn new(
        impl_: Arc<dyn NamEsTrait>,
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
            "SOME_FUNCTION" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.some_function(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "Some_Function2" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.some_function2(param_0).await {
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
            "Switch" => {
                if let Ok(v) = serde_json::from_slice::<bool>(&msg.payload) {
                    self.impl_.set_switch(v);
                }
            }
            "SOME_PROPERTY" => {
                if let Ok(v) = serde_json::from_slice::<i32>(&msg.payload) {
                    self.impl_.set_some_property(v);
                }
            }
            "Some_Poperty2" => {
                if let Ok(v) = serde_json::from_slice::<i32>(&msg.payload) {
                    self.impl_.set_some_poperty2(v);
                }
            }
            "enum_property" => {
                if let Ok(v) = serde_json::from_slice::<Enum_With_Under_scoresEnum>(&msg.payload) {
                    self.impl_.set_enum_property(v);
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
            "Switch": self.impl_.switch(),
            "SOME_PROPERTY": self.impl_.some_property(),
            "Some_Poperty2": self.impl_.some_poperty2(),
            "enum_property": self.impl_.enum_property()
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
