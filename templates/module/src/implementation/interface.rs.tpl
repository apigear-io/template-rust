{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
{{- if or .Module.Structs .Module.Enums }}
#[allow(unused_imports)]
use crate::api::data_structs::*;
{{- end }}
{{- if $hasOps }}
use apigear::{ApiError, ApiFuture};
{{- end }}
{{- if $hasPubSub }}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Publisher;
{{- end }}
{{- if $hasProps }}
use parking_lot::RwLock;
{{- end }}

{{ if not $hasProps -}}
#[derive(Default)]
{{ end -}}
pub struct {{Camel .Interface.Name}} {{ if not $hasPubSub }}{}{{ else }}{
{{- range .Interface.Properties }}
    {{snake .Name}}: RwLock<{{rsType "" .}}>,
{{- end }}
{{- if $hasPubSub }}
    publisher: {{Camel .Interface.Name}}Publisher,
{{- end }}
}{{ end }}
{{- if $hasProps }}

impl Default for {{Camel .Interface.Name}} {
    fn default() -> Self {
        Self {{`{`}}
{{- range $i, $e := .Interface.Properties }}
{{-   if $i }},{{ end }} {{snake .Name}}: RwLock::new(Default::default())
{{- end }}
{{- if $hasPubSub }}, publisher: Default::default(){{ end }} }
    }
}
{{- end }}

impl {{Camel .Interface.Name}}Trait for {{Camel .Interface.Name}} {{ if $isEmpty }}{}{{ else }}{
{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
{{- if len $operation.Params }}
    fn {{snake $operation.Name }}(
        &self,
        {{rsParams "_" "" ",\n        " $operation.Params}},
    ) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        Box::pin(async move { Ok({{ if .Return.IsVoid }}(){{ else }}Default::default(){{ end }}) })
    }
{{- else }}
    fn {{snake $operation.Name }}(&self) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        Box::pin(async move { Ok({{ if .Return.IsVoid }}(){{ else }}Default::default(){{ end }}) })
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
        self.{{ snake $property.Name }}.read().clone()
        {{- else }}
        *self.{{ snake $property.Name }}.read()
        {{- end }}
    }
    {{- if not .IsReadOnly }}
    fn set_{{snake $property.Name}}(
        &self,
        {{ rsParam "" "" $property }},
    ) {
        {{- if and ( eq "string" $property.Type ) ( eq false $property.IsArray )}}
        let new_val = {{ snake $property.Name }}.to_string();
        let mut value = self.{{ snake $property.Name }}.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.{{ snake $property.Name }}_changed.send(new_val);
        {{- else if $property.IsArray }}
        let new_val = {{ snake $property.Name }}.to_vec();
        let mut value = self.{{ snake $property.Name }}.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.{{ snake $property.Name }}_changed.send(new_val);
        {{- else if $isComplex }}
        let new_val = {{ snake $property.Name }}.clone();
        let mut value = self.{{ snake $property.Name }}.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.{{ snake $property.Name }}_changed.send(new_val);
        {{- else }}
        let mut value = self.{{ snake $property.Name }}.write();
        if *value == {{ snake $property.Name }} {
            return;
        }
        *value = {{ snake $property.Name }};
        let _ = self.publisher.{{ snake $property.Name }}_changed.send({{ snake $property.Name }});
        {{- end }}
    }
    {{- end }}
{{- end }}

{{- if $hasPubSub }}

    fn publisher(&self) -> &{{Camel .Interface.Name}}Publisher {
        &self.publisher
    }
{{- end }}
}{{ end }}
