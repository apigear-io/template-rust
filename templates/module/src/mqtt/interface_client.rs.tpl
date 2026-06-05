{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) }}
{{- $hasReturningOps := false }}
{{- range .Interface.Operations }}{{ if not .Return.IsVoid }}{{ $hasReturningOps = true }}{{ end }}{{ end -}}
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
{{- if $hasProps }}
use crate::core_types::{{snake .Interface.Name}}_data::{{Camel .Interface.Name}}Data;
use parking_lot::RwLock;
{{- end }}
{{- if $hasReturningOps }}
use parking_lot::Mutex;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
{{- end }}
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
{{- if $hasReturningOps }}
use std::collections::HashMap;
{{- end }}
use std::sync::Arc;
{{- if $hasReturningOps }}
use std::time::Duration;
use tokio::sync::oneshot;
{{- end }}

const TOPIC_PREFIX: &str = "{{.Module.Name}}/{{.Interface.Name}}";
{{- if $hasReturningOps }}
const RPC_TIMEOUT: Duration = Duration::from_secs(5);
{{- end }}

/// MQTT client adapter for {{Camel .Interface.Name}}.
/// Implements the interface trait using the agreed ApiGear (MQTT 5) wire scheme:
/// operations are published on `rpc/<op>` with an MQTT 5 `ResponseTopic` +
/// `CorrelationData` and the reply is awaited; property writes go to `set/<prop>`;
/// retained `prop/<prop>` notifications and `sig/<sig>` signals update local state.
pub struct {{Camel .Interface.Name}}MqttClient {
{{- if $hasProps }}
    data: RwLock<{{Camel .Interface.Name}}Data>,
{{- end }}
    client: Arc<AsyncClient>,
{{- if $hasReturningOps }}
    client_id: String,
    next_correlation: std::sync::atomic::AtomicU64,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
{{- end }}
{{- if $hasPubSub }}
    publisher: {{Camel .Interface.Name}}Publisher,
{{- end }}
}

impl {{Camel .Interface.Name}}MqttClient {
    /// Create a new MQTT client adapter. `client_id` must be unique per client and
    /// is used to route RPC replies (`rpc/<op>/<client_id>/result`).
    pub fn new(
        client: Arc<AsyncClient>,
        {{ if $hasReturningOps }}client_id{{ else }}_client_id{{ end }}: impl Into<String>,
    ) -> Self {
        Self {{`{`}}
{{- if $hasProps }} data: RwLock::new({{Camel .Interface.Name}}Data::default()),{{ end }} client
{{- if $hasReturningOps }}, client_id: client_id.into(), next_correlation: std::sync::atomic::AtomicU64::new(1), pending: Arc::new(Mutex::new(HashMap::new())){{ end }}
{{- if $hasPubSub }}, publisher: {{Camel .Interface.Name}}Publisher::default(){{ end }} }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
{{- range .Interface.Properties }}
        self.client.subscribe(format!("{}/prop/{{.Name}}", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
{{- end }}
{{- range .Interface.Signals }}
        self.client.subscribe(format!("{}/sig/{{.Name}}", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
{{- end }}
{{- range .Interface.Operations }}
{{- if not .Return.IsVoid }}
        self.client.subscribe(format!("{}/rpc/{{.Name}}/{}/result", TOPIC_PREFIX, self.client_id), QoS::AtLeastOnce).await?;
{{- end }}
{{- end }}
        Ok(())
    }

    /// Handle an incoming MQTT message by dispatching to the appropriate handler.
    /// `correlation_data` (from the MQTT 5 publish properties) routes RPC replies.
    pub fn handle_message(
        &self,
        topic: &str,
        payload: &[u8],
        {{ if $hasReturningOps }}correlation_data{{ else }}_correlation_data{{ end }}: Option<&[u8]>,
    ) {
        let suffix = topic.strip_prefix(&format!("{}/", TOPIC_PREFIX)).unwrap_or("");
{{- if $hasReturningOps }}

        if suffix.starts_with("rpc/") {
            if let Some(id) = correlation_data.and_then(|b| std::str::from_utf8(b).ok()).and_then(|s| s.parse::<u64>().ok()) {
                if let Some(tx) = self.pending.lock().remove(&id) {
                    let value: Value = serde_json::from_slice(payload).unwrap_or_default();
                    let _ = tx.send(value);
                }
            }
            return;
        }
{{- end }}
{{- if $hasPubSub }}
        let value: Value = serde_json::from_slice(payload).unwrap_or_default();

{{- if $hasProps }}
        if let Some(prop_name) = suffix.strip_prefix("prop/") {
            self.handle_property_change(prop_name, value);
            return;
        }
{{- end }}
{{- if $hasSignals }}
        if let Some(sig_name) = suffix.strip_prefix("sig/") {
            self.handle_signal(sig_name, value);
        }
{{- end }}
{{- end }}
    }
{{- if $hasProps }}

    fn handle_property_change(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
{{- range .Interface.Properties }}
{{- $isPropComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}
            "{{.Name}}" => {
                if let Ok(v) = serde_json::from_value::<{{ rsType "" . }}>(value) {
                    let _ = self.publisher.{{ snake .Name }}_changed.send(v{{ if $isPropComplex }}.clone(){{ end }});
                    self.data.write().{{ snake .Name }} = v;
                }
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown property: {}", property_name);
            }
        }
    }
{{- end }}
{{- if $hasSignals }}

    #[allow(clippy::get_first)]
    fn handle_signal(
        &self,
        signal_name: &str,
        args: Value,
    ) {
        match signal_name {
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
                tracing::warn!("Unknown signal: {}", signal_name);
            }
        }
    }
{{- end }}
}

impl {{Camel .Interface.Name}}Trait for {{Camel .Interface.Name}}MqttClient {{ if $isEmpty }}{}{{ else }}{
{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
{{- $hasParams := len $operation.Params }}
{{- if $hasParams }}
    fn {{snake $operation.Name }}(
        &self,
        {{rsParams "" "" ",\n        " $operation.Params}},
    ) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
{{- else }}
    fn {{snake $operation.Name }}(&self) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
{{- end }}
        let args = json!([{{ range $i, $e := $operation.Params }}{{- if $i }}, {{ end }}{{ rsVar "" .}}{{ end }}]);
        let client = self.client.clone();
        let request_topic = format!("{}/rpc/{{$operation.Name}}", TOPIC_PREFIX);
{{- if $operation.Return.IsVoid }}
        // Void operation: fire-and-forget, no reply is requested or awaited.
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(request_topic, QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(())
        })
{{- else }}
        let id = self.next_correlation.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().insert(id, tx);
        let pending = self.pending.clone();
        let response_topic = format!("{}/rpc/{{$operation.Name}}/{}/result", TOPIC_PREFIX, self.client_id);
        Box::pin(async move {
            let props = PublishProperties { response_topic: Some(response_topic), correlation_data: Some(id.to_string().into()), ..Default::default() };
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            if let Err(e) = client.publish_with_properties(request_topic, QoS::AtLeastOnce, false, payload, props).await {
                pending.lock().remove(&id);
                return Err(ApiError::OperationFailed(e.to_string()));
            }
            match tokio::time::timeout(RPC_TIMEOUT, rx).await {
                Ok(Ok(value)) => Ok(serde_json::from_value(value).unwrap_or_default()),
                _ => {
                    pending.lock().remove(&id);
                    Err(ApiError::OperationFailed("rpc reply timed out".to_string()))
                }
            }
        })
{{- end }}
    }
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
        let client = self.client.clone();
        let topic = format!("{}/set/{{$property.Name}}", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!({{ snake $property.Name }})).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, QoS::AtLeastOnce, false, payload).await;
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
