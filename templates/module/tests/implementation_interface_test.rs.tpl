{{- $data_structs := false }}
{{- $ops := false -}}

{{- if or .Module.Structs .Module.Enums -}}
{{- $data_structs = true -}}
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use {{snake .Module.Name}}::api::data_structs::*;{{ nl }}
{{- end -}}
{{- if or (len .Interface.Operations) (len .Interface.Properties) -}}
use {{snake .Module.Name}}::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
{{- end }}
use {{snake .Module.Name}}::implementation::{{snake .Interface.Name}}::{{Camel .Interface.Name}};

/// tests for {{Camel .Interface.Name}}
#[cfg(test)]
mod tests {
    use super::*;{{nl}}

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
    #[test]
    fn test_{{ snake $operation.Name }}() {
        let mut test_object: {{Camel $.Interface.Name}} = Default::default();
        test_object.{{snake $operation.Name }}(
        {{- range $i, $e := $operation.Params }}
        {{- if $i }}, {{ end }}
        {{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
        {{- $param := . }}
        {{- if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end}}Default::default()
        {{- end }}   {{- /* end range operation params */ -}}
        );
    }

    #[test]
    fn test_{{snake $operation.Name }}_async() {
        let mut test_object: {{Camel $.Interface.Name}} = Default::default();
        let _ = test_object.{{snake $operation.Name }}_async(
        {{- range $i, $e := $operation.Params }}
        {{- if $i }}, {{ end }}
        {{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
        {{- $param := . }}
        {{- if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end}}Default::default()
        {{- end }}   {{- /* end range operation params */ -}}
        );
    }
{{- end }}

{{- if len .Interface.Operations }}{{- if len .Interface.Properties }}{{- nl }}{{ end }}{{ end }}

{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- $property := . }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
    #[test]
    fn test_{{snake $property.Name }}() {
        let mut test_object: {{Camel $.Interface.Name}} = Default::default();
        let default_value: {{rsType "" $property}} = Default::default();
        {{- if not .IsReadOnly }}
        test_object.set_{{snake $property.Name }}({{ if $isComplex }}&{{end}}default_value);
        {{- end }}
        assert_eq!(test_object.{{snake $property.Name }}().clone(), default_value);
    }
{{- end }}    {{- /* end range properties */}}
}
