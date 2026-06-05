use crate::api::simple_interface::SimpleInterfaceTrait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "tb.simple.SimpleInterface";

/// NATS service adapter for SimpleInterface.
/// Bridges a local implementation to NATS using the agreed ApiGear wire scheme:
/// operation requests on `rpc.<op>` (request/reply), property-change requests on
/// `set.<prop>`, change notifications on `prop.<prop>`, signals on `sig.<sig>`,
/// an availability beacon on `service.available`, and an `init` handshake that
/// replies the current state on `init.resp.<clientId>`.
pub struct SimpleInterfaceNatsService {
    impl_: Arc<dyn SimpleInterfaceTrait>,
    client: async_nats::Client,
}

impl SimpleInterfaceNatsService {
    pub fn new(
        impl_: Arc<dyn SimpleInterfaceTrait>,
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
            "propBool": self.impl_.prop_bool(),
            "propInt": self.impl_.prop_int(),
            "propInt32": self.impl_.prop_int32(),
            "propInt64": self.impl_.prop_int64(),
            "propFloat": self.impl_.prop_float(),
            "propFloat32": self.impl_.prop_float32(),
            "propFloat64": self.impl_.prop_float64(),
            "propString": self.impl_.prop_string()
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
            "funcNoReturnValue" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_no_return_value(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcNoParams" => match self.impl_.func_no_params().await {
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
            "funcInt" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_int(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcInt32" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_int32(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcInt64" => {
                let param_0: i64 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_int64(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat" => {
                let param_0: f32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_float(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat32" => {
                let param_0: f32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_float32(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat64" => {
                let param_0: f64 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_float64(param_0).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
            "funcString" => {
                let param_0: String = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                match self.impl_.func_string(param_0.as_str()).await {
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
                if let Ok(v) = serde_json::from_slice::<bool>(&msg.payload) {
                    self.impl_.set_prop_bool(v);
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_slice::<i32>(&msg.payload) {
                    self.impl_.set_prop_int(v);
                }
            }
            "propInt32" => {
                if let Ok(v) = serde_json::from_slice::<i32>(&msg.payload) {
                    self.impl_.set_prop_int32(v);
                }
            }
            "propInt64" => {
                if let Ok(v) = serde_json::from_slice::<i64>(&msg.payload) {
                    self.impl_.set_prop_int64(v);
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_slice::<f32>(&msg.payload) {
                    self.impl_.set_prop_float(v);
                }
            }
            "propFloat32" => {
                if let Ok(v) = serde_json::from_slice::<f32>(&msg.payload) {
                    self.impl_.set_prop_float32(v);
                }
            }
            "propFloat64" => {
                if let Ok(v) = serde_json::from_slice::<f64>(&msg.payload) {
                    self.impl_.set_prop_float64(v);
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_slice::<String>(&msg.payload) {
                    self.impl_.set_prop_string(&v);
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
