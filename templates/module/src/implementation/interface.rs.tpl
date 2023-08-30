{{- $data_structs := false }}
{{- $ops := false -}}

use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
{{- if or .Module.Structs .Module.Enums }}
{{- $data_structs = true }}
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;{{ nl }}
{{- end }}

{{- if len .Interface.Operations -}}
{{- $ops = true }}
use async_trait::async_trait;
{{- end }}
{{- if or (len .Interface.Properties) $ops }}{{ nl }}{{ end }}
#[derive(Default, Clone)]
{{- if len .Interface.Properties }}
pub struct {{Camel .Interface.Name}} {
{{- range .Interface.Properties }}
{{- $property:= . }}
    {{snake $property.Name}}: {{rsType "" $property}},
{{- end }}
}{{nl}}
{{- else }}
pub struct {{Camel .Interface.Name}} {}{{nl}}
{{- end }}
{{- if len .Interface.Operations }}
#[async_trait]
{{- end }}
impl {{Camel .Interface.Name}}Trait for {{Camel .Interface.Name}} {
{{- $interface := .Interface }}
{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
{{- if $operation.Description }}
    /// {{ $operation.Description }}
{{- range $operation.Params }}
{{- $param := . }}
{{- if $param.Description }}
    /// `{{$param}}` {{$param.Description}}
{{- end }}   {{- /* end if param description */}}
{{- end }}   {{- /* end range operation param*/}}  
{{- end }}   {{- /* end if operations description */}}
{{- if len $operation.Params }}
    fn {{snake $operation.Name }}(
        &mut self,
        {{rsParams "_" "" ",\n        " $operation.Params}},
    ){{- if not .Return.IsVoid }} -> {{ rsReturn "" $operation.Return}}{{- end }} {
        Default::default()
    }
{{- else }}
    fn {{snake $operation.Name }}(&mut self){{- if not .Return.IsVoid }} -> {{ rsReturn "" $operation.Return}}{{- end }} {
        Default::default()
    }
{{- end }}
    /// Asynchronous version of [{{ snake $operation.Name}}]({{Camel $interface.Name}}::{{ snake $operation.Name}})
{{- if $operation.Description }}
    * {{$operation.Description}}
{{- end }}   {{- /* end if description */}}
{{- range $operation.Params }}
{{- $param := . }}
{{- if $param.Description }}
    /// `{{$param}}` {{$param.Description}}
{{- end }}   {{- /* end if description */}}
{{- end }}   {{- /* end range operation params */}}
    /// returns future of type [`{{rsReturn "" $operation.Return}}`] which is set once the function has completed
{{- if len $operation.Params }}
    async fn {{snake $operation.Name }}_async(
        &mut self,
        {{rsParams "" "" ",\n        " $operation.Params}},
    ) -> Result<{{rsReturn "" $operation.Return}}, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.{{snake $operation.Name }}(
        {{- range $i, $e := $operation.Params }}
        {{- $param := . }}
        {{- if $i }}, {{ end }}
        {{- rsVar "" .}}{{ end -}}
        ))
    }
{{- else }}
    async fn {{snake $operation.Name }}_async(&mut self) -> Result<{{rsReturn "" $operation.Return}}, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.{{snake $operation.Name }}())
    }
{{- end }}
{{- end }}   {{- /* end range operations */}}

{{- if len .Interface.Operations }}{{- if len .Interface.Properties }}{{- nl }}{{ end }}{{ end }}

{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- $property := . }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type)}}
    /// Gets the value of the {{$property.Name}} property.
    {{- if $property.Description }}
    /// {{$property.Description}}
    {{- end }}    {{- /* end if property.Description */}}
    fn {{snake $property.Name }}(&self) -> {{rsTypeRef "" $property}} {
        {{ if $isComplex }}&{{end}}self.{{ snake $property.Name }}
    }
    {{- if not .IsReadOnly }}
    /// Sets the value of the {{$property.Name}} property.
    {{- if $property.Description }}
    /// {{$property.Name}} {{$property.Description}}
    {{- end }}    {{- /* end if property.Description */}}
    fn set_{{snake $property.Name}}(
        &mut self,
        {{ rsParam "" "" $property }},
    ) {
        {{- if and ( eq "string" $property.Type ) ( eq false $property.IsArray )}}
        if self.{{ snake $property.Name }} == {{ snake $property.Name }} {
            return;
        }

        self.{{ snake $property.Name }} = {{ snake $property.Name }}.to_string();
        {{- else }}
        {{- if eq false $property.IsArray }}
        if self.{{ snake $property.Name }} == {{ snake $property.Name }}{{ if $isComplex }}.clone(){{ end }} {
            return;
        }

        self.{{ snake $property.Name }} = {{ snake $property.Name }}{{ if $isComplex }}.clone(){{ end }};
        {{- else }}
        if self.{{ snake $property.Name }} == {{ snake $property.Name }}{{ if $isComplex }}.to_vec(){{ end }} {
            return;
        }

        self.{{ snake $property.Name }} = {{ snake $property.Name }}{{ if $isComplex }}.to_vec(){{ end }};
        {{- end }}
        {{- end }}
    }
    {{- end }}
{{- end }}    {{- /* end range properties */}}
}
