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
use apigear::{ApiError, ApiFuture};
{{ end -}}
{{- if $hasPubSub -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Publisher;
{{ end -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
use crate::core_types::{{snake .Interface.Name}}_data::{{Camel .Interface.Name}}Data;
use parking_lot::RwLock;
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/{{.Module.Name}}/{{.Interface.Name}}";

/// MQTT client adapter for {{Camel .Interface.Name}}.
/// Implements the interface trait by forwarding operations over MQTT
/// and caching property values locally.
pub struct {{Camel .Interface.Name}}MqttClient {
    data: RwLock<{{Camel .Interface.Name}}Data>,
    client: Arc<AsyncClient>,
{{- if $hasPubSub }}
    publisher: {{Camel .Interface.Name}}Publisher,
{{- end }}
}

impl {{Camel .Interface.Name}}MqttClient {
    /// Create a new MQTT client adapter with the given MQTT async client.
    pub fn new(client: Arc<AsyncClient>) -> Self {
        Self {{`{`}} data: RwLock::new({{Camel .Interface.Name}}Data::default()), client
{{- if $hasPubSub }}, publisher: {{Camel .Interface.Name}}Publisher::default(){{ end }} }
    }

    /// Subscribe to all relevant MQTT topics for this interface.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
{{- range .Interface.Properties }}
        self.client.subscribe(format!("{}/prop/{{.Name}}", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
{{- end }}
{{- range .Interface.Signals }}
        self.client.subscribe(format!("{}/sig/{{.Name}}", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
{{- end }}
{{- range .Interface.Operations }}
        self.client.subscribe(format!("{}/op/{{.Name}}/resp", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
{{- end }}
        self.client.subscribe(format!("{}/state", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
        Ok(())
    }

    /// Handle an incoming MQTT message by dispatching to the appropriate handler.
    pub fn handle_message(
        &self,
        topic: &str,
        payload: &[u8],
    ) {
        let suffix = topic.strip_prefix(&format!("{}/", TOPIC_PREFIX)).unwrap_or("");
        let value: Value = serde_json::from_slice(payload).unwrap_or_default();

        if suffix == "state" {
            self.handle_state(value);
            return;
        }

        if let Some(prop_name) = suffix.strip_prefix("prop/") {
            self.handle_property_change(prop_name, value);
            return;
        }

        if let Some(sig_name) = suffix.strip_prefix("sig/") {
            self.handle_signal(sig_name, value);
        }
    }

    fn handle_state(
        &self,
        value: Value,
    ) {
        if let Ok(data) = serde_json::from_value::<{{Camel .Interface.Name}}Data>(value) {
            *self.data.write() = data;
        }
    }

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
{{- if $hasPubSub }}
                    let _ = self.publisher.{{ snake .Name }}_changed.send(v{{ if $isPropComplex }}.clone(){{ end }});
{{- end }}
                    self.data.write().{{ snake .Name }} = v;
                }
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown property: {}", property_name);
            }
        }
    }

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
}

impl {{Camel .Interface.Name}}Trait for {{Camel .Interface.Name}}MqttClient {{ if $isEmpty }}{}{{ else }}{
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
        let topic = format!("{}/op/{{$operation.Name}}/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
        })
    }
{{- else }}
    fn {{snake $operation.Name }}(&self) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        let args = json!([]);
        let client = self.client.clone();
        let topic = format!("{}/op/{{$operation.Name}}/req", TOPIC_PREFIX);
        Box::pin(async move {
            let payload = serde_json::to_vec(&args).unwrap_or_default();
            client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await.map_err(|e| ApiError::OperationFailed(e.to_string()))?;
            Ok(Default::default())
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
        let client = self.client.clone();
        let topic = format!("{}/prop/{{$property.Name}}", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&json!({{ snake $property.Name }})).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await;
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
