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
use rumqttc::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "apigear/{{.Module.Name}}/{{.Interface.Name}}";

/// MQTT service adapter for {{Camel .Interface.Name}}.
/// Bridges a local implementation to MQTT by subscribing to operation requests
/// and publishing property changes and signals.
pub struct {{Camel .Interface.Name}}MqttService {
    impl_: Arc<dyn {{Camel .Interface.Name}}Trait>,
    client: Arc<AsyncClient>,
}

impl {{Camel .Interface.Name}}MqttService {
    pub fn new(
        impl_: Arc<dyn {{Camel .Interface.Name}}Trait>,
        client: Arc<AsyncClient>,
    ) -> Self {
        Self { impl_, client }
    }

    /// Subscribe to all relevant MQTT topics for this service.
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::ClientError> {
{{- range .Interface.Operations }}
        self.client.subscribe(format!("{}/op/{{.Name}}/req", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
{{- end }}
{{- range .Interface.Properties }}
{{- if not .IsReadOnly }}
        self.client.subscribe(format!("{}/prop/{{.Name}}", TOPIC_PREFIX), rumqttc::QoS::AtLeastOnce).await?;
{{- end }}
{{- end }}
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

        if let Some(rest) = suffix.strip_prefix("op/") {
            if let Some(op_name) = rest.strip_suffix("/req") {
                self.handle_invoke(op_name, value);
            }
            return;
        }

        if let Some(prop_name) = suffix.strip_prefix("prop/") {
            self.handle_set_property(prop_name, value);
        }
    }

    #[allow(clippy::get_first)]
    fn handle_invoke(
        &self,
        method_name: &str,
        args: Value,
    ) {
        #[allow(unused_variables)]
        let arr = args.as_array();
        let client = self.client.clone();
        match method_name {
{{- range .Interface.Operations }}
{{- $operation := . }}
            "{{$operation.Name}}" => {
{{- range $i, $e := $operation.Params }}
                let param_{{ $i }}: {{ rsType "" . }} = serde_json::from_value(arr.and_then(|a| a.get({{ $i }}).cloned()).unwrap_or_default()).unwrap_or_default();
{{- end }}
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.{{snake $operation.Name}}(
{{- range $i, $e := $operation.Params }}
{{-     $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{-     if $i }}, {{ end -}}
{{      if and $isComplex (not .IsArray) (ne "string" .Type) }}&{{ end -}}
param_{{ $i }}
{{- if .IsArray }}.as_slice(){{ end -}}
{{- if and (eq "string" .Type) (not .IsArray) }}.as_str(){{ end -}}
{{- end -}}
))));
                let result = match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                };
                let topic = format!("{}/op/{{$operation.Name}}/resp", TOPIC_PREFIX);
                let payload = serde_json::to_vec(&result).unwrap_or_default();
                tokio::spawn(async move {
                    let _ = client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await;
                });
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown method: {}", method_name);
            }
        }
    }

    fn handle_set_property(
        &self,
        property_name: &str,
        value: Value,
    ) {
        match property_name {
{{- range .Interface.Properties }}
{{- if not .IsReadOnly }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}
            "{{.Name}}" => {
                if let Ok(v) = serde_json::from_value::<{{ rsType "" . }}>(value) {
                    self.impl_.set_{{ snake .Name }}({{ if $isComplex }}&{{ end }}v);
                }
            }
{{- end }}
{{- end }}
            _ => {
                tracing::warn!("Unknown property: {}", property_name);
            }
        }
    }

{{- range .Interface.Properties }}
    /// Publish {{.Name}} property change over MQTT (retained).
    pub async fn publish_{{snake .Name}}_changed(&self) -> Result<(), rumqttc::ClientError> {
        let value = json!(self.impl_.{{ snake .Name }}());
        let topic = format!("{}/prop/{{.Name}}", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
{{- end }}

{{- range .Interface.Signals }}
{{- $signal := . }}
{{- $lenParams := len .Params }}
{{- if gt $lenParams 0 }}
    pub async fn publish_{{snake .Name}}(
        &self,
        {{rsParams "" "" ",\n        " $signal.Params}},
    ) -> Result<(), rumqttc::ClientError> {
        let args = json!([{{ range $i, $e := $signal.Params }}{{- if $i }}, {{ end }}{{ rsVar "" .}}{{ end }}]);
        let topic = format!("{}/sig/{{.Name}}", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
{{- else }}
    pub async fn publish_{{snake .Name}}(&self) -> Result<(), rumqttc::ClientError> {
        let args = json!([]);
        let topic = format!("{}/sig/{{.Name}}", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, false, payload).await
    }
{{- end }}
{{- end }}

    /// Publish the full initial state (retained).
    pub async fn publish_state(&self) -> Result<(), rumqttc::ClientError> {
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
        let topic = format!("{}/state", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&state).unwrap_or_default();
        self.client.publish(&topic, rumqttc::QoS::AtLeastOnce, true, payload).await
    }
}
