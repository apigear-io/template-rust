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

{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
    #[test]
    #[rustfmt::skip]
    fn test_{{ snake $operation.Name }}() {
        let mut test_object: {{Camel $.Interface.Name}} = Default::default();
        test_object.{{snake $operation.Name }}(
        {{- range $operation.Params }}
        {{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
        {{- $param := . }}
        {{ if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end}}Default::default(),
        {{- end }}   {{- /* end range operation params */}}
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_{{snake $operation.Name }}_async() {
        let mut test_object: {{Camel $.Interface.Name}} = Default::default();
        let _ = test_object.{{snake $operation.Name }}_async(
        {{- range $operation.Params }}
        {{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
        {{- $param := . }}
        {{ if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end}}Default::default(),
        {{- end }}   {{- /* end range operation params */}}
        );
    }
{{- end }}

{{- if len .Interface.Operations }}{{- if len .Interface.Properties }}{{- nl }}{{ end }}{{ end }}

{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- $property := . }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
    #[test]
    #[rustfmt::skip]
    fn test_{{snake $property.Name }}() {
        let mut test_object: {{Camel $.Interface.Name}} = Default::default();
        let default_value: {{rustType "" $property}} = Default::default();
        {{- if not .IsReadOnly }}
        test_object.set_{{snake $property.Name }}({{ if $isComplex }}&{{end}}default_value);
        {{- end }}
        assert_eq!(test_object.{{snake $property.Name }}().clone(), default_value);
    }
{{- end }}    {{- /* end range properties */}}
}
