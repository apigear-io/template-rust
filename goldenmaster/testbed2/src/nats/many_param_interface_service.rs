#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::many_param_interface::ManyParamInterfaceTrait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.testbed2.ManyParamInterface";

/// NATS service adapter for ManyParamInterface.
/// Bridges a local implementation to NATS by subscribing to operation and
/// set-property subjects, and publishing property changes and signals.
pub struct ManyParamInterfaceNatsService {
    impl_: Arc<dyn ManyParamInterfaceTrait>,
    client: async_nats::Client,
}

impl ManyParamInterfaceNatsService {
    pub fn new(
        impl_: Arc<dyn ManyParamInterfaceTrait>,
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
            "func1" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func1(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "func2" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func2(param_0, param_1).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "func3" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_2: i32 = serde_json::from_value(arr.and_then(|a| a.get(2).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func3(param_0, param_1, param_2).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "func4" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_2: i32 = serde_json::from_value(arr.and_then(|a| a.get(2).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_3: i32 = serde_json::from_value(arr.and_then(|a| a.get(3).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func4(param_0, param_1, param_2, param_3).await {
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
                if let Ok(v) = serde_json::from_slice::<i32>(&msg.payload) {
                    self.impl_.set_prop1(v);
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_slice::<i32>(&msg.payload) {
                    self.impl_.set_prop2(v);
                }
            }
            "prop3" => {
                if let Ok(v) = serde_json::from_slice::<i32>(&msg.payload) {
                    self.impl_.set_prop3(v);
                }
            }
            "prop4" => {
                if let Ok(v) = serde_json::from_slice::<i32>(&msg.payload) {
                    self.impl_.set_prop4(v);
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
            "prop1": self.impl_.prop1(),
            "prop2": self.impl_.prop2(),
            "prop3": self.impl_.prop3(),
            "prop4": self.impl_.prop4()
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
