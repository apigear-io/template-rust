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

const TOPIC_PREFIX: &str = "apigear.{{.Module.Name}}.{{.Interface.Name}}";

/// NATS service adapter for {{Camel .Interface.Name}}.
/// Bridges a local implementation to NATS by subscribing to operation and
/// set-property subjects, and publishing property changes and signals.
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

    /// Start background subscriptions for operations and set-property requests.
    /// Returns a `JoinHandle` that runs until the service is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
{{- if $hasOps }}
            let mut op_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.op.*")).await.expect("operation subscription failed");
{{- end }}
{{- if $hasProps }}
            let mut prop_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.prop.*")).await.expect("property subscription failed");
{{- end }}
            loop {
                tokio::select! {
{{- if $hasOps }}
                    Some(msg) = op_sub.next() => {
                        this.handle_operation(msg).await;
                    }
{{- end }}
{{- if $hasProps }}
                    Some(msg) = prop_sub.next() => {
                        this.handle_set_property(&msg);
                    }
{{- end }}
                    else => break,
                }
            }
        })
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

    /// Publish the current state of all properties.
    pub async fn publish_state(&self) {
{{- if .Interface.Properties }}
        let state = json!({
{{- range $i, $e := .Interface.Properties }}
{{- if $i }},{{ end }}
            "{{.Name}}": self.impl_.{{ snake .Name }}()
{{- end }}
        });
{{- else }}
        let state = json!({});
{{- end }}
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.state"), payload.into()).await;
    }
{{- if $hasProps }}

    /// Publish a property change notification.
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

    /// Publish a signal.
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
