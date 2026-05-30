#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::many_param_interface::ManyParamInterfacePublisher;
use crate::api::many_param_interface::ManyParamInterfaceTrait;
use crate::core_types::many_param_interface_data::ManyParamInterfaceData;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear.testbed2.ManyParamInterface";

/// NATS client adapter for ManyParamInterface.
/// Implements the interface trait by forwarding operations via NATS request/reply
/// and caching property values locally.
pub struct ManyParamInterfaceNatsClient {
    data: RwLock<ManyParamInterfaceData>,
    client: async_nats::Client,
    publisher: ManyParamInterfacePublisher,
}

impl ManyParamInterfaceNatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self { data: RwLock::new(ManyParamInterfaceData::default()), client, publisher: ManyParamInterfacePublisher::default() }
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
            "prop1" => {
                if let Ok(v) = serde_json::from_value::<i32>(payload) {
                    let _ = self.publisher.prop1_changed.send(v);
                    self.data.write().prop1 = v;
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_value::<i32>(payload) {
                    let _ = self.publisher.prop2_changed.send(v);
                    self.data.write().prop2 = v;
                }
            }
            "prop3" => {
                if let Ok(v) = serde_json::from_value::<i32>(payload) {
                    let _ = self.publisher.prop3_changed.send(v);
                    self.data.write().prop3 = v;
                }
            }
            "prop4" => {
                if let Ok(v) = serde_json::from_value::<i32>(payload) {
                    let _ = self.publisher.prop4_changed.send(v);
                    self.data.write().prop4 = v;
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
            "sig1" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig1.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(),));
                }
            }
            "sig2" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig2.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(), serde_json::from_value(arr.get(1).cloned().unwrap_or_default()).unwrap_or_default()));
                }
            }
            "sig3" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig3.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(), serde_json::from_value(arr.get(1).cloned().unwrap_or_default()).unwrap_or_default(), serde_json::from_value(arr.get(2).cloned().unwrap_or_default()).unwrap_or_default()));
                }
            }
            "sig4" => {
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.sig4.send((serde_json::from_value(arr.get(0).cloned().unwrap_or_default()).unwrap_or_default(), serde_json::from_value(arr.get(1).cloned().unwrap_or_default()).unwrap_or_default(), serde_json::from_value(arr.get(2).cloned().unwrap_or_default()).unwrap_or_default(), serde_json::from_value(arr.get(3).cloned().unwrap_or_default()).unwrap_or_default()));
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
        if let Ok(data) = serde_json::from_slice::<ManyParamInterfaceData>(&msg.payload) {
            *self.data.write() = data;
        }
    }
}

impl ManyParamInterfaceTrait for ManyParamInterfaceNatsClient {
    fn func1(
        &self,
        param1: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        let args = json!([param1]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.func1"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn func2(
        &self,
        param1: i32,
        param2: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        let args = json!([param1, param2]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.func2"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn func3(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        let args = json!([param1, param2, param3]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.func3"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn func4(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
        param4: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        let args = json!([param1, param2, param3, param4]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.op.func4"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }

    fn prop1(&self) -> i32 {
        self.data.read().prop1
    }
    fn set_prop1(
        &self,
        prop1: i32,
    ) {
        let payload = serde_json::to_vec(&json!(prop1)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.prop1"), payload.into()).await;
        });
    }

    fn prop2(&self) -> i32 {
        self.data.read().prop2
    }
    fn set_prop2(
        &self,
        prop2: i32,
    ) {
        let payload = serde_json::to_vec(&json!(prop2)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.prop2"), payload.into()).await;
        });
    }

    fn prop3(&self) -> i32 {
        self.data.read().prop3
    }
    fn set_prop3(
        &self,
        prop3: i32,
    ) {
        let payload = serde_json::to_vec(&json!(prop3)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.prop3"), payload.into()).await;
        });
    }

    fn prop4(&self) -> i32 {
        self.data.read().prop4
    }
    fn set_prop4(
        &self,
        prop4: i32,
    ) {
        let payload = serde_json::to_vec(&json!(prop4)).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.prop.prop4"), payload.into()).await;
        });
    }

    fn publisher(&self) -> &ManyParamInterfacePublisher {
        &self.publisher
    }
}
