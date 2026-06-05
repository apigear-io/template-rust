{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) }}
{{- $hasWritableProps := false }}
{{- range .Interface.Properties }}{{ if not .IsReadOnly }}{{ $hasWritableProps = true }}{{ end }}{{ end -}}
{{- if $isEmpty }}#![allow(unused_imports, dead_code, clippy::never_loop)]{{ nl }}{{ end }}
{{- if or .Module.Structs .Module.Enums -}}
#[allow(unused_imports)]
use crate::api::data_structs::*;
{{ end -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
{{- if $hasOps }}
use rumqttc::v5::mqttbytes::v5::PublishProperties;
{{- end }}
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use serde_json::{json, Value};
use std::sync::Arc;

const TOPIC_PREFIX: &str = "{{.Module.Name}}/{{.Interface.Name}}";

/// MQTT service adapter for {{Camel .Interface.Name}}.
/// Bridges a local implementation to MQTT using the agreed ApiGear (MQTT 5) wire
/// scheme: operation requests on `rpc/<op>` (answered on the request's
/// `ResponseTopic` with `CorrelationData` echoed), property-change requests on
/// `set/<prop>`, retained change notifications on `prop/<prop>`, signals on
/// `sig/<sig>`.
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
    pub async fn subscribe_topics(&self) -> Result<(), rumqttc::v5::ClientError> {
{{- range .Interface.Operations }}
        self.client.subscribe(format!("{}/rpc/{{.Name}}", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
{{- end }}
{{- range .Interface.Properties }}
{{- if not .IsReadOnly }}
        self.client.subscribe(format!("{}/set/{{.Name}}", TOPIC_PREFIX), QoS::AtLeastOnce).await?;
{{- end }}
{{- end }}
        Ok(())
    }

    /// Handle an incoming MQTT message by dispatching to the appropriate handler.
    /// `response_topic` and `correlation_data` come from the MQTT 5 publish
    /// properties and route RPC replies back to the caller.
    pub fn handle_message(
        &self,
        topic: &str,
        payload: &[u8],
        {{ if $hasOps }}response_topic{{ else }}_response_topic{{ end }}: Option<&str>,
        {{ if $hasOps }}correlation_data{{ else }}_correlation_data{{ end }}: Option<&[u8]>,
    ) {
        let suffix = topic.strip_prefix(&format!("{}/", TOPIC_PREFIX)).unwrap_or("");
{{- if $hasOps }}

        if let Some(op_name) = suffix.strip_prefix("rpc/") {
            let value: Value = serde_json::from_slice(payload).unwrap_or_default();
            self.handle_invoke(op_name, value, response_topic, correlation_data);
            return;
        }
{{- end }}
{{- if $hasWritableProps }}

        if let Some(prop_name) = suffix.strip_prefix("set/") {
            let value: Value = serde_json::from_slice(payload).unwrap_or_default();
            self.handle_set_property(prop_name, value);
        }
{{- end }}
    }
{{- if $hasOps }}

    #[allow(clippy::get_first)]
    fn handle_invoke(
        &self,
        method_name: &str,
        args: Value,
        response_topic: Option<&str>,
        correlation_data: Option<&[u8]>,
    ) {
        #[allow(unused_variables)]
        let arr = args.as_array();
        let client = self.client.clone();
        let response_topic = response_topic.map(|s| s.to_string());
        let correlation_data = correlation_data.map(|b| b.to_vec());
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
                self.send_reply(client, response_topic, correlation_data, result);
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown method: {}", method_name);
            }
        }
    }

    /// Publish an RPC result back to the caller's `ResponseTopic`, echoing its
    /// `CorrelationData`. No reply is sent when the caller did not request one
    /// (e.g. void operations).
    fn send_reply(
        &self,
        client: Arc<AsyncClient>,
        response_topic: Option<String>,
        correlation_data: Option<Vec<u8>>,
        result: Value,
    ) {
        let Some(response_topic) = response_topic else {
            return;
        };
        let props = PublishProperties { correlation_data: correlation_data.map(Into::into), ..Default::default() };
        let payload = serde_json::to_vec(&result).unwrap_or_default();
        tokio::spawn(async move {
            let _ = client.publish_with_properties(response_topic, QoS::AtLeastOnce, false, payload, props).await;
        });
    }
{{- end }}
{{- if $hasWritableProps }}

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
{{- end }}

{{- range .Interface.Properties }}
    /// Publish {{.Name}} property change over MQTT (retained).
    pub async fn publish_{{snake .Name}}_changed(&self) -> Result<(), rumqttc::v5::ClientError> {
        let value = json!(self.impl_.{{ snake .Name }}());
        let topic = format!("{}/prop/{{.Name}}", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&value).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, true, payload).await
    }
{{- end }}

{{- range .Interface.Signals }}
{{- $signal := . }}
{{- $lenParams := len .Params }}
{{- if gt $lenParams 0 }}
    pub async fn publish_{{snake .Name}}(
        &self,
        {{rsParams "" "" ",\n        " $signal.Params}},
    ) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([{{ range $i, $e := $signal.Params }}{{- if $i }}, {{ end }}{{ rsVar "" .}}{{ end }}]);
        let topic = format!("{}/sig/{{.Name}}", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
{{- else }}
    pub async fn publish_{{snake .Name}}(&self) -> Result<(), rumqttc::v5::ClientError> {
        let args = json!([]);
        let topic = format!("{}/sig/{{.Name}}", TOPIC_PREFIX);
        let payload = serde_json::to_vec(&args).unwrap_or_default();
        self.client.publish(&topic, QoS::AtLeastOnce, false, payload).await
    }
{{- end }}
{{- end }}

    /// Re-publish the current value of every property (retained) so newly
    /// connected clients receive the latest state. The MQTT scheme has no
    /// dedicated state topic; retained `prop/<name>` messages carry the state.
    pub async fn publish_current_state(&self) -> Result<(), rumqttc::v5::ClientError> {
{{- range .Interface.Properties }}
        self.publish_{{snake .Name}}_changed().await?;
{{- end }}
        Ok(())
    }
}
