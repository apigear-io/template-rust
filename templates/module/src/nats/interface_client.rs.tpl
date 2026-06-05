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
{{- if $hasOps -}}
use crate::api::{ApiError, ApiFuture};
{{ end -}}
{{- if $hasPubSub -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Publisher;
{{ end -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
use crate::core_types::{{snake .Interface.Name}}_data::{{Camel .Interface.Name}}Data;
#[allow(unused_imports)]
use futures::StreamExt;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "{{.Module.Name}}.{{.Interface.Name}}";

/// Generate a process-unique **numeric** client id used to route the
/// `init.resp.<clientId>` handshake reply back to this client. It is a number
/// (not a string) and stays within the signed-32-bit range so it matches the
/// init payload the other ApiGear templates (e.g. C++) expect.
fn new_client_id() -> u64 {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let pid = std::process::id() as u64;
    pid.wrapping_mul(100_000).wrapping_add(n) % 2_000_000_000
}

/// NATS client adapter for {{Camel .Interface.Name}}.
/// Implements the interface trait using the agreed ApiGear wire scheme:
/// operations via `rpc.<op>` request/reply, property writes via `set.<prop>`,
/// change notifications on `prop.<prop>`, signals on `sig.<sig>`, and an `init`
/// handshake (replied on `init.resp.<clientId>`) to fetch the current state.
pub struct {{Camel .Interface.Name}}NatsClient {
    data: RwLock<{{Camel .Interface.Name}}Data>,
    client: async_nats::Client,
    client_id: u64,
{{- if $hasPubSub }}
    publisher: {{Camel .Interface.Name}}Publisher,
{{- end }}
}

impl {{Camel .Interface.Name}}NatsClient {
    pub fn new(client: async_nats::Client) -> Self {
        Self {{`{`}} data: RwLock::new({{Camel .Interface.Name}}Data::default()), client, client_id: new_client_id()
{{- if $hasPubSub }}, publisher: {{Camel .Interface.Name}}Publisher::default(){{ end }} }
    }

    /// Start background subscriptions (notifications, signals, availability, init
    /// reply) and request the current state. Returns a `JoinHandle` that runs
    /// until the client is dropped.
    pub fn subscribe(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = Arc::clone(self);
        tokio::spawn(async move {
{{- if $hasProps }}
            let mut prop_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.prop.*")).await.expect("property subscription failed");
{{- end }}
{{- if $hasSignals }}
            let mut sig_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.sig.*")).await.expect("signal subscription failed");
{{- end }}
            let mut avail_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.service.available")).await.expect("availability subscription failed");
            let mut init_resp_sub = this.client.subscribe(format!("{TOPIC_PREFIX}.init.resp.{}", this.client_id)).await.expect("init-reply subscription failed");
            // Ask the service for the current state right away.
            this.request_init().await;
            loop {
                tokio::select! {
{{- if $hasProps }}
                    Some(msg) = prop_sub.next() => {
                        this.handle_property_change(&msg);
                    }
{{- end }}
{{- if $hasSignals }}
                    Some(msg) = sig_sub.next() => {
                        this.handle_signal(&msg);
                    }
{{- end }}
                    Some(_msg) = avail_sub.next() => {
                        // The service (re)appeared: re-sync our state.
                        this.request_init().await;
                    }
                    Some(msg) = init_resp_sub.next() => {
                        this.handle_state(&msg);
                    }
                    else => break,
                }
            }
        })
    }

    /// Send an `init` request carrying our client id; the service replies the
    /// full state on `init.resp.<clientId>`.
    async fn request_init(&self) {
        let payload = serde_json::to_vec(&self.client_id).unwrap_or_default();
        let _ = self.client.publish(format!("{TOPIC_PREFIX}.init"), payload.into()).await;
        let _ = self.client.flush().await;
    }
{{- if $hasProps }}

    fn handle_property_change(
        &self,
        msg: &async_nats::Message,
    ) {
        let subject = msg.subject.as_str();
        let member = subject.rsplit('.').next().unwrap_or("");
        let payload: Value = serde_json::from_slice(&msg.payload).unwrap_or_default();
        match member {
{{- range .Interface.Properties }}
{{- $isPropComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}
            "{{.Name}}" => {
                if let Ok(v) = serde_json::from_value::<{{ rsType "" . }}>(payload) {
{{- if $hasPubSub }}
                    let _ = self.publisher.{{ snake .Name }}_changed.send(v{{ if $isPropComplex }}.clone(){{ end }});
{{- end }}
                    self.data.write().{{ snake .Name }} = v;
                }
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown property: {}", subject);
            }
        }
    }
{{- end }}
{{- if $hasSignals }}

    #[allow(clippy::get_first)]
    fn handle_signal(
        &self,
        msg: &async_nats::Message,
    ) {
        let subject = msg.subject.as_str();
        let member = subject.rsplit('.').next().unwrap_or("");
        let args: Value = serde_json::from_slice(&msg.payload).unwrap_or_default();
        match member {
{{- range .Interface.Signals }}
{{- $signal := . }}
{{- $lenParams := len .Params }}
            "{{.Name}}" => {
{{- if gt $lenParams 0 }}
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.{{ snake .Name }}.send((
{{- range $i, $e := .Params }}
{{-     if $i }}, {{ end -}}
serde_json::from_value(arr.get({{ $i }}).cloned().unwrap_or_default()).unwrap_or_default()
{{- end }}
{{- if eq $lenParams 1 }},{{ end -}}
));
                }
{{- else }}
                let _ = self.publisher.{{ snake .Name }}.send(());
{{- end }}
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown signal: {}", subject);
            }
        }
    }
{{- end }}

    fn handle_state(
        &self,
        msg: &async_nats::Message,
    ) {
        if let Ok(data) = serde_json::from_slice::<{{Camel .Interface.Name}}Data>(&msg.payload) {
            *self.data.write() = data;
        }
    }
}

impl {{Camel .Interface.Name}}Trait for {{Camel .Interface.Name}}NatsClient {{ if $isEmpty }}{}{{ else }}{
{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
{{- if len $operation.Params }}
    fn {{snake $operation.Name }}(
        &self,
        {{rsParams "" "" ",\n        " $operation.Params}},
    ) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        let args = json!([{{ range $i, $e := $operation.Params }}{{- if $i }}, {{ end }}{{ rsVar "" .}}{{ end }}]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.{{$operation.Name}}"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }
{{- else }}
    fn {{snake $operation.Name }}(&self) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        let args = json!([]);
        let client = self.client.clone();
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            match client.request(format!("{TOPIC_PREFIX}.rpc.{{$operation.Name}}"), payload.into()).await {
                Ok(reply) => {
                    let value: Value = serde_json::from_slice(&reply.payload).unwrap_or_default();
                    Ok(serde_json::from_value(value).unwrap_or_default())
                }
                Err(e) => Err(ApiError::OperationFailed(e.to_string())),
            }
        })
    }
{{- end }}
{{- end }}

{{- if $hasOps }}{{- if $hasProps }}{{- nl }}{{ end }}{{ end }}

{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- $property := . }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
    fn {{snake $property.Name }}(&self) -> {{rsType "" $property}} {
        {{- if $isComplex }}
        self.data.read().{{ snake $property.Name }}.clone()
        {{- else }}
        self.data.read().{{ snake $property.Name }}
        {{- end }}
    }
    {{- if not .IsReadOnly }}
    fn set_{{snake $property.Name}}(
        &self,
        {{ rsParam "" "" $property }},
    ) {
        let payload = serde_json::to_vec(&json!({{ snake $property.Name }})).unwrap_or_default();
        let client = self.client.clone();
        tokio::spawn(async move {
            let _ = client.publish(format!("{TOPIC_PREFIX}.set.{{$property.Name}}"), payload.into()).await;
        });
    }
    {{- end }}
{{- end }}

{{- if $hasPubSub }}

    fn publisher(&self) -> &{{Camel .Interface.Name}}Publisher {
        &self.publisher
    }
{{- end }}
}{{ end }}
