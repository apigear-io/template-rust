{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) }}
{{- $hasWritableProps := false }}
{{- range .Interface.Properties }}{{ if not .IsReadOnly }}{{ $hasWritableProps = true }}{{ end }}{{ end -}}
{{- if $isEmpty -}}
// {{Camel .Interface.Name}} exposes no operations, properties, or signals to exercise over OLink.
{{- else -}}
mod olink_common;

{{- if or .Module.Structs .Module.Enums }}
#[allow(unused_imports)]
use {{snake .Module.Name}}::api::data_structs::*;
{{- end }}
{{- if or $hasOps $hasPubSub }}
use {{snake .Module.Name}}::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
{{- end }}
use {{snake .Module.Name}}::implementation::{{snake .Interface.Name}}::{{Camel .Interface.Name}};
use {{snake .Module.Name}}::olink::{{snake .Interface.Name}}_client::{{Camel .Interface.Name}}OlinkClient;
use {{snake .Module.Name}}::olink::{{snake .Interface.Name}}_service::{{Camel .Interface.Name}}OlinkService;
use objectlink_core::traits::ObjectSink;
{{- if $hasPubSub }}
use objectlink_core::traits::IRemoteNode;
{{- end }}
use olink_common::setup_olink_loopback;
{{- if or $hasProps $hasSignals }}
use serde_json::json;
{{- end }}
use std::sync::Arc;

/// Helper: create a loopback-wired OLink client backed by a real implementation.
fn setup_{{snake .Interface.Name}}() -> (Arc<{{Camel .Interface.Name}}OlinkClient>, Arc<{{Camel .Interface.Name}}>, olink_common::OlinkLoopback) {
    let impl_ = Arc::new({{Camel .Interface.Name}}::default());
    let service: Arc<dyn objectlink_core::traits::ObjectSource> = Arc::new({{Camel .Interface.Name}}OlinkService::new(impl_.clone() as Arc<dyn {{Camel .Interface.Name}}Trait>));
    let loopback = setup_olink_loopback(service);

    let client = Arc::new({{Camel .Interface.Name}}OlinkClient::default());
    client.set_node(loopback.client_node.clone());
    let sink: Arc<dyn ObjectSink> = client.clone();
    loopback.client_node.link_remote(&sink);

    (client, impl_, loopback)
}

#[cfg(test)]
mod tests {
    use super::*;
{{- if $hasOps }}

    // -- Operations --
{{- range $i, $e := .Interface.Operations }}
{{- $operation := . }}

    #[tokio::test(flavor = "multi_thread")]
    async fn test_olink_{{ snake $operation.Name }}() {
        let (client, _impl, _loopback) = setup_{{snake $.Interface.Name}}();
        let result = client.{{snake $operation.Name }}(
{{- range $i, $e := $operation.Params }}
{{-     if $i }}, {{ end -}}
{{-     $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{      if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end -}}
Default::default()
{{- end -}}
).await;
        assert!(result.is_ok());
    }
{{- end }}
{{- end }}
{{- if $hasWritableProps }}

    // -- Properties (set via client -> verify on impl) --
{{- range $i, $e := .Interface.Properties }}
{{- if not .IsReadOnly }}
{{- $property := . }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}

    #[test]
    fn test_olink_set_{{snake $property.Name }}() {
        let (client, impl_, _loopback) = setup_{{snake $.Interface.Name}}();
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
        assert_eq!(impl_.{{snake $property.Name }}(), test_value);
    }
{{- end }}
{{- end }}

    // -- Properties (notify change from remote -> verify on client) --
{{- range $i, $e := .Interface.Properties }}
{{- $property := . }}

    #[test]
    fn test_olink_{{snake $property.Name }}_notify() {
        let (client, _impl, loopback) = setup_{{snake $.Interface.Name}}();
        {{- if $hasPubSub }}
        let rx = client.publisher().{{snake $property.Name }}_changed.subscribe();
        {{- end }}
        {{- if .IsArray }}
        let test_value = json!([]);
        let expected: {{rsType "" $property}} = vec![];
        {{- else if eq .Type "bool" }}
        let test_value = json!(true);
        let expected: {{rsType "" $property}} = true;
        {{- else if or (eq .Type "int") (eq .Type "int32") }}
        let test_value = json!(1);
        let expected: {{rsType "" $property}} = 1i32;
        {{- else if eq .Type "int64" }}
        let test_value = json!(1);
        let expected: {{rsType "" $property}} = 1i64;
        {{- else if or (eq .Type "float") (eq .Type "float32") }}
        let test_value = json!(1.0);
        let expected: {{rsType "" $property}} = 1.0f32;
        {{- else if eq .Type "float64" }}
        let test_value = json!(1.0);
        let expected: {{rsType "" $property}} = 1.0f64;
        {{- else if eq .Type "string" }}
        let test_value = json!("test");
        let expected: {{rsType "" $property}} = String::from("test");
        {{- else }}
        let expected: {{rsType "" $property}} = Default::default();
        let test_value = serde_json::to_value(&expected).unwrap();
        {{- end }}
        loopback.remote_node.notify_property_change("{{$.Module.Name}}.{{$.Interface.Name}}/{{$property.Name}}", test_value);
        assert_eq!(client.{{snake $property.Name }}(), expected);
        {{- if $hasPubSub }}
        assert!(rx.has_changed().unwrap_or(false));
        {{- end }}
    }
{{- end }}
{{- end }}
{{- if $hasSignals }}

    // -- Signals --
{{- range $i, $e := .Interface.Signals }}
{{- $signal := . }}
{{- $lenParams := len .Params }}

    #[test]
    fn test_olink_{{snake $signal.Name }}() {
        let (client, _impl, loopback) = setup_{{snake $.Interface.Name}}();
        let mut rx = client.publisher().{{snake $signal.Name }}.subscribe();
        loopback.remote_node.notify_signal("{{$.Module.Name}}.{{$.Interface.Name}}/{{$signal.Name}}", json!([
{{- range $i, $e := $signal.Params }}
{{-     if $i }}, {{ end -}}
{{      if .IsArray }}[]
{{-     else if eq .Type "bool" }}true
{{-     else if or (eq .Type "int") (eq .Type "int32") }}1
{{-     else if eq .Type "int64" }}1
{{-     else if or (eq .Type "float") (eq .Type "float32") }}1.0
{{-     else if eq .Type "float64" }}1.0
{{-     else if eq .Type "string" }}"test"
{{-     else }}null
{{-     end -}}
{{- end -}}
]));
        {{- if gt $lenParams 0 }}
        let received = rx.try_recv().unwrap();
        {{- range $i, $e := $signal.Params }}
        {{- if .IsArray }}
        let _signal_param_{{ $i }} = received.{{ $i }}.clone();
        {{- else if eq .Type "bool" }}
        assert_eq!(received.{{ $i }}, true);
        {{- else if or (eq .Type "int") (eq .Type "int32") }}
        assert_eq!(received.{{ $i }}, 1i32);
        {{- else if eq .Type "int64" }}
        assert_eq!(received.{{ $i }}, 1i64);
        {{- else if or (eq .Type "float") (eq .Type "float32") }}
        assert_eq!(received.{{ $i }}, 1.0f32);
        {{- else if eq .Type "float64" }}
        assert_eq!(received.{{ $i }}, 1.0f64);
        {{- else if eq .Type "string" }}
        assert_eq!(received.{{ $i }}, "test");
        {{- else }}
        let _signal_param_{{ $i }} = received.{{ $i }}.clone();
        {{- end }}
        {{- end }}
        {{- else }}
        assert!(rx.try_recv().is_ok());
        {{- end }}
    }
{{- end }}
{{- end }}
}
{{- end }}
