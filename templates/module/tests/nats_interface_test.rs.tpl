{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) }}
{{- $hasWritableProps := false }}
{{- range .Interface.Properties }}{{ if not .IsReadOnly }}{{ $hasWritableProps = true }}{{ end }}{{ end -}}
{{- if $isEmpty -}}
// {{Camel .Interface.Name}} exposes no operations, properties, or signals to exercise over NATS.
{{- else -}}
mod nats_common;

{{- if or .Module.Structs .Module.Enums }}
#[allow(unused_imports)]
use {{snake .Module.Name}}::api::data_structs::*;
{{- end }}
use {{snake .Module.Name}}::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
use {{snake .Module.Name}}::implementation::{{snake .Interface.Name}}::{{Camel .Interface.Name}};
use {{snake .Module.Name}}::nats::{{snake .Interface.Name}}_client::{{Camel .Interface.Name}}NatsClient;
use {{snake .Module.Name}}::nats::{{snake .Interface.Name}}_service::{{Camel .Interface.Name}}NatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_{{snake .Interface.Name}}_roundtrip() {
    let impl_ = Arc::new({{Camel .Interface.Name}}::default());
    let service = Arc::new({{Camel .Interface.Name}}NatsService::new(impl_.clone() as Arc<dyn {{Camel .Interface.Name}}Trait>, nats_common::connect().await));
    let _service_sub = service.subscribe();
    let _ = service.publish_state().await;

    let client = Arc::new({{Camel .Interface.Name}}NatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;
{{- if $hasOps }}

    // Operations: NATS request/reply round-trip (the service answers each request).
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

    // Writable properties: a client set propagates over NATS to the service implementation.
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
        assert!(nats_common::wait_until(|| impl_.{{snake $property.Name }}() == test_value).await);
    }
{{- end }}
{{- end }}
{{- end }}
}
{{- end }}
