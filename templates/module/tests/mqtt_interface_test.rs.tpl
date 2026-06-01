{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) }}
{{- $hasWritableProps := false }}
{{- range .Interface.Properties }}{{ if not .IsReadOnly }}{{ $hasWritableProps = true }}{{ end }}{{ end -}}
{{- if $isEmpty -}}
// {{Camel .Interface.Name}} exposes no operations, properties, or signals to exercise over MQTT.
{{- else -}}
mod mqtt_common;

{{- if or .Module.Structs .Module.Enums }}
#[allow(unused_imports)]
use {{snake .Module.Name}}::api::data_structs::*;
{{- end }}
use {{snake .Module.Name}}::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
use {{snake .Module.Name}}::implementation::{{snake .Interface.Name}}::{{Camel .Interface.Name}};
use {{snake .Module.Name}}::mqtt::{{snake .Interface.Name}}_client::{{Camel .Interface.Name}}MqttClient;
use {{snake .Module.Name}}::mqtt::{{snake .Interface.Name}}_service::{{Camel .Interface.Name}}MqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_{{snake .Interface.Name}}_roundtrip() {
    let impl_ = Arc::new({{Camel .Interface.Name}}::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-{{snake .Module.Name}}-{{snake .Interface.Name}}");
    let service = Arc::new({{Camel .Interface.Name}}MqttService::new(impl_.clone() as Arc<dyn {{Camel .Interface.Name}}Trait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload| service_drive.handle_message(topic, payload));

    let (client_client, client_loop) = mqtt_common::connect("cli-{{snake .Module.Name}}-{{snake .Interface.Name}}");
    let client = Arc::new({{Camel .Interface.Name}}MqttClient::new(client_client));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload| client_drive.handle_message(topic, payload));

    mqtt_common::settle().await;
    let _ = service.publish_state().await;
{{- if $hasOps }}

    // Operations: published as MQTT requests and delivered to the broker.
{{- range $i, $e := .Interface.Operations }}
{{- $operation := . }}
    assert!(client.{{snake $operation.Name }}(
{{- range $j, $p := $operation.Params }}
{{-   if $j }}, {{ end -}}
{{-   $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{    if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end -}}
Default::default()
{{- end -}}
).await.is_ok());
{{- end }}
{{- end }}
{{- if $hasWritableProps }}

    // Writable properties: a client set propagates over MQTT to the service implementation.
{{- range $i, $e := .Interface.Properties }}
{{- if not .IsReadOnly }}
{{- $property := . }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}
    {
        {{- if .IsArray }}
        let test_value: {{rsType "" $property}} = vec![Default::default()];
        {{- else if eq .Type "bool" }}
        let test_value: {{rsType "" $property}} = true;
        {{- else if or (eq .Type "int") (eq .Type "int32") }}
        let test_value: {{rsType "" $property}} = 1i32;
        {{- else if eq .Type "int64" }}
        let test_value: {{rsType "" $property}} = 1i64;
        {{- else if or (eq .Type "float") (eq .Type "float32") }}
        let test_value: {{rsType "" $property}} = 1.0f32;
        {{- else if eq .Type "float64" }}
        let test_value: {{rsType "" $property}} = 1.0f64;
        {{- else if eq .Type "string" }}
        let test_value: {{rsType "" $property}} = String::from("test");
        {{- else }}
        let test_value: {{rsType "" $property}} = Default::default();
        {{- end }}
        client.set_{{snake $property.Name }}({{ if and $isComplex (not .IsArray) (ne "string" .Type) }}&{{ end }}test_value{{ if and $isComplex (not .IsArray) (ne "string" .Type) }}.clone(){{ end }}{{ if .IsArray }}.as_slice(){{ end }}{{ if and (eq "string" .Type) (not .IsArray) }}.as_str(){{ end }});
        assert!(mqtt_common::wait_until(|| impl_.{{snake $property.Name }}() == test_value).await);
    }
{{- end }}
{{- end }}
{{- end }}
}
{{- end }}
