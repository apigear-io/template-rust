{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $noTests := and (not (len .Module.Enums)) (not $hasOps) (not $hasProps) (not $hasSignals) }}

{{- if or .Module.Structs .Module.Enums -}}
#[allow(unused_imports)]
use {{snake .Module.Name}}::api::data_structs::*;{{ nl }}
{{- end -}}
{{- if or $hasOps $hasPubSub -}}
use {{snake .Module.Name}}::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
{{ end -}}
use {{snake .Module.Name}}::implementation::{{snake .Interface.Name}}::{{Camel .Interface.Name}};

/// tests for {{Camel .Interface.Name}}
#[cfg(test)]
mod tests {
    use super::*;{{ if not $noTests }}{{nl}}{{ end }}

{{- range $i, $e := .Module.Enums }}
{{- $enum := . }}
{{- if $i }}{{nl}}{{ end }}
    #[test]
    fn test_to_{{snake .Name}}_enum() {
        {{- $errValUsed := false }}
        {{- range $idx, $elem := .Members }}
        {{- if eq 254 .Value }}{{ $errValUsed = true }}{{ end }}
        assert_eq!({{$enum}}Enum::try_from({{ .Value }}), Ok({{$enum}}Enum::{{ upper1 .Name }}));
        {{- if eq .Name $enum.Default.Name }}
        assert_eq!({{$enum}}Enum::try_from({{ .Value }}), Ok({{$enum}}Enum::default()));
        {{- end }}
        {{- end }}
        {{- if not $errValUsed }}
        // test error case assuming 254 is not defined in IDL
        assert_eq!({{$enum}}Enum::try_from(254), Err(()));
        {{- end }}
    }

    #[test]
    fn test_from_{{snake .Name}}_enum() {
        {{- $errValUsed := false }}
        {{- range $idx, $elem := .Members }}
        {{- if eq 254 .Value }}{{ $errValUsed = true }}{{ end }}
        let result: Result<{{$enum}}Enum, ()> = {{ .Value }}u8.try_into();
        assert_eq!(result, Ok({{$enum}}Enum::{{ upper1 .Name }}));
        {{- end }}
        {{- if not $errValUsed }}
        // test error case assuming 254 is not defined in IDL
        let result: Result<{{$enum}}Enum, ()> = 254u8.try_into();
        assert_eq!(result, Err(()));
        {{- end }}
    }
{{- end }}
{{- if len .Module.Enums }}{{ nl }}{{ end }}

{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
    #[tokio::test]
    async fn test_{{ snake $operation.Name }}() {
        let test_object = {{Camel $.Interface.Name}}::default();
        let result = test_object.{{snake $operation.Name }}(
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

{{- if $hasOps }}{{- if $hasProps }}{{- nl }}{{ end }}{{ end }}

{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- $property := . }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
    #[test]
    fn test_{{snake $property.Name }}() {
        let test_object = {{Camel $.Interface.Name}}::default();
        let default_value: {{rsType "" $property}} = Default::default();
        {{- if not .IsReadOnly }}
        test_object.set_{{snake $property.Name }}({{ if and $isComplex (not .IsArray) (ne "string" .Type) }}&{{end}}default_value{{ if and $isComplex (not .IsArray) (ne "string" .Type) }}.clone(){{ end }}{{ if .IsArray }}.as_slice(){{ end }}{{ if and (eq "string" .Type) (not .IsArray) }}.as_str(){{ end }});
        {{- end }}
        assert_eq!(test_object.{{snake $property.Name }}(), default_value);
    }
{{- end }}

{{- if $hasSignals }}{{- nl }}{{ end }}

{{- range $i, $e := .Interface.Signals }}
{{- if $i }}{{nl}}{{ end }}
{{- $signal := . }}
{{- $lenParams := len .Params }}
    #[test]
    fn test_{{snake $signal.Name }}() {
        let test_object = {{Camel $.Interface.Name}}::default();
        let mut rx = test_object.publisher().{{snake $signal.Name }}.subscribe();
        {{- range $signal.Params}}
        let default_value_{{ rsVar "" .}}: {{rsType "" .}} = Default::default();
        {{- end }}
        let _ = test_object.publisher().{{snake $signal.Name }}.send((
{{- range $i, $e := $signal.Params}}
{{-     if $i }}, {{ end -}}
default_value_{{ rsVar "" .}}.clone()
{{- end }}
{{- if eq 1 $lenParams }},{{ end -}}
));
        {{- if gt $lenParams 0 }}
        let received = rx.try_recv().unwrap();
        {{- range $i, $e := $signal.Params}}
        assert_eq!(received.{{ $i }}, default_value_{{ rsVar "" .}});
        {{- end }}
        {{- else }}
        assert!(rx.try_recv().is_ok());
        {{- end }}
    }
{{- end }}
}
