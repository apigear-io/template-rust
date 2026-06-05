{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) }}
{{- if $isEmpty }}#![allow(unused_imports, dead_code, clippy::never_loop)]{{ nl }}{{ end }}
{{- if or .Module.Structs .Module.Enums -}}
#[allow(unused_imports)]
use crate::api::data_structs::*;
{{ end -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
#[allow(unused_imports)]
use futures::StreamExt;
#[allow(unused_imports)]
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "{{.Module.Name}}.{{.Interface.Name}}";

/// NATS service adapter for {{Camel .Interface.Name}}.
/// Bridges a local implementation to NATS using the agreed ApiGear wire scheme:
/// operation requests on `rpc.<op>` (request/reply), property-change requests on
/// `set.<prop>`, change notifications on `prop.<prop>`, signals on `sig.<sig>`,
/// an availability beacon on `service.available`, and an `init` handshake that
/// replies the current state on `init.resp.<clientId>`.
pub struct {{Camel .Interface.Name}}NatsService {
    impl_: Arc<dyn {{Camel .Interface.Name}}Trait>,
    client: async_nats::Client,
}

impl {{Camel .Interface.Name}}NatsService {
    pub fn new(
        impl_: Arc<dyn {{Camel .Interface.Name}}Trait>,
        client: async_nats::Client,
    ) -> Self {
        Self { impl_, client }
    }

    /// Start background subscriptions and announce availability.
    /// Returns a `JoinHandle` that runs until the service is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
{{- if $hasOps }}
            let mut rpc_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.rpc.*")).await.expect("operation subscription failed");
{{- end }}
{{- if $hasProps }}
            let mut set_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.set.*")).await.expect("set-property subscription failed");
{{- end }}
            let mut init_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.init")).await.expect("init subscription failed");
            // Now that we are subscribed, announce availability so clients (re-)sync.
            this.publish_service_available().await;
            loop {
                tokio::select! {
{{- if $hasOps }}
                    Some(msg) = rpc_sub.next() => {
                        this.handle_operation(msg).await;
                    }
{{- end }}
{{- if $hasProps }}
                    Some(msg) = set_sub.next() => {
                        this.handle_set_property(&msg);
                    }
{{- end }}
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
{{- if .Interface.Properties }}
        json!({
{{- range $i, $e := .Interface.Properties }}
{{- if $i }},{{ end }}
            "{{.Name}}": self.impl_.{{ snake .Name }}()
{{- end }}
        })
{{- else }}
        json!({})
{{- end }}
    }
{{- if $hasOps }}

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
{{- range .Interface.Operations }}
{{- $operation := . }}
{{- if $operation.Params }}
            "{{$operation.Name}}" => {
{{- range $i, $e := $operation.Params }}
                let param_{{ $i }}: {{ rsType "" . }} = serde_json::from_value(arr.and_then(|a| a.get({{ $i }}).cloned()).unwrap_or_default()).unwrap_or_default();
{{- end }}
                match self.impl_.{{snake $operation.Name}}(
{{- range $i, $e := $operation.Params }}
{{-     $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{-     if $i }}, {{ end -}}
{{      if and $isComplex (not .IsArray) (ne "string" .Type) }}&{{ end -}}
param_{{ $i }}
{{- if .IsArray }}.as_slice(){{ end -}}
{{- if and (eq "string" .Type) (not .IsArray) }}.as_str(){{ end -}}
{{- end -}}
).await {
                    Ok(value) => json!(value),
                    _ => json!(null),
                }
            }
{{- else }}
            "{{$operation.Name}}" => match self.impl_.{{snake $operation.Name}}().await {
                Ok(value) => json!(value),
                _ => json!(null),
            },
{{- end }}
{{- end }}
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
{{- end }}
{{- if $hasProps }}

    fn handle_set_property(
        &self,
        msg: &async_nats::Message,
    ) {
        let subject = msg.subject.as_str();
        let member = subject.rsplit('.').next().unwrap_or("");
        match member {
{{- range .Interface.Properties }}
{{- if not .IsReadOnly }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}
            "{{.Name}}" => {
                if let Ok(v) = serde_json::from_slice::<{{ rsType "" . }}>(&msg.payload) {
                    self.impl_.set_{{ snake .Name }}({{ if $isComplex }}&{{ end }}v);
                }
            }
{{- end }}
{{- end }}
            _ => {
                tracing::warn!("Unknown property: {}", subject);
            }
        }
    }
{{- end }}
{{- if $hasProps }}

    /// Publish a property change notification on `prop.<property>`.
    pub async fn notify_property_changed(
        &self,
        property: &str,
        value: Value,
    ) {
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.prop.{property}"), payload.into()).await;
    }
{{- end }}
{{- if $hasSignals }}

    /// Publish a signal on `sig.<signal>`.
    pub async fn notify_signal(
        &self,
        signal: &str,
        args: Value,
    ) {
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.sig.{signal}"), payload.into()).await;
    }
{{- end }}
}
