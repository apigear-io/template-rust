{{- $data_structs := false }}
{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) }}

{{- if or .Module.Structs .Module.Enums -}}
{{- $data_structs = true -}}
#[allow(unused_imports)]
use crate::api::data_structs::*;{{ nl }}
{{- end }}

{{- if $hasOps -}}
use apigear::{ApiError, ApiFuture};{{ nl }}
{{- end }}

{{- if $hasPubSub -}}
use tokio::sync::{
{{- if $hasProps }}watch{{ end }}
{{- if and $hasProps $hasSignals }}, {{ end }}
{{- if $hasSignals }}broadcast{{ end -}}
};

pub struct {{Camel .Interface.Name}}Publisher {
{{- range $i, $e := .Interface.Properties }}
{{- if .Description }}
    /// {{ .Description }}
{{- end }}
    pub {{snake .Name}}_changed: watch::Sender<{{ rsType "" .}}>,
{{- end }}
{{- range $i, $e := .Interface.Signals }}
{{- if .Description }}
    /// {{ .Description }}
{{- end }}
{{- $lenParams := len .Params }}
    pub {{snake .Name}}: broadcast::Sender<(
        {{- range $i, $e := .Params }}
        {{- if $i }}, {{ end }}{{ rsType "" .}}{{ end }}{{- if eq 1 $lenParams }},{{ end -}}
        )>,
{{- end }}
}

impl Default for {{Camel .Interface.Name}}Publisher {
    fn default() -> Self {
        Self {{`{`}}
{{- range $i, $e := .Interface.Properties }}
{{-   if $i }},{{ end }} {{snake .Name}}_changed: watch::channel(Default::default()).0
{{- end }}
{{- range $i, $e := .Interface.Signals }}
{{-   if or $i $hasProps }},{{ end }} {{snake .Name}}: broadcast::Sender::new(16)
{{- end }} }
    }
}

{{ end -}}
pub trait {{Camel .Interface.Name}}Trait: Send + Sync {{ if $isEmpty }}{}{{ else }}{
{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
{{- if $operation.Description }}
    /// {{ $operation.Description }}
{{- range $operation.Params }}
{{- $param := . }}
{{- if $param.Description }}
    /// `{{snake $param.Name}}` {{$param.Description}}
{{- end }}
{{- end }}
{{- end }}
{{- if len $operation.Params }}
    fn {{snake $operation.Name }}(
        &self,
        {{rsParams "" "" ",\n        " $operation.Params}},
    ) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>>;
{{- else }}
    fn {{snake $operation.Name }}(&self) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>>;
{{- end }}
{{- end }}

{{- if $hasOps }}{{- if $hasProps }}{{- nl }}{{ end }}{{ end }}

{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- $property := . }}
    /// Gets the value of the {{$property.Name}} property.
    {{- if $property.Description }}
    /// {{$property.Description}}
    {{- end }}
    fn {{snake $property.Name }}(&self) -> {{rsType "" $property}};
    {{- if not .IsReadOnly }}
    /// Sets the value of the {{$property.Name}} property.
    {{- if $property.Description }}
    /// {{$property.Name}} {{$property.Description}}
    {{- end }}
    fn set_{{snake $property.Name}}(
        &self,
        {{ rsParam "" "" $property }},
    );
    {{- end }}
{{- end }}

{{- if $hasPubSub }}

    fn publisher(&self) -> &{{Camel .Interface.Name}}Publisher;
{{- end }}
}{{ end }}
