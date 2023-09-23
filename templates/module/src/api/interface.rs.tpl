{{- $data_structs := false }}
{{- $ops := false }}

{{- if or .Module.Structs .Module.Enums -}}
{{- $data_structs = true -}}
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;{{ nl }}
{{- end }}

{{- if len .Interface.Operations -}}
{{- $ops = true -}}
use async_trait::async_trait;{{ nl }}
{{- end }}

{{- if or (len .Interface.Signals) (len .Interface.Properties) -}}
use signals2::*;

#[derive(Clone, Default)]
pub struct {{Camel .Interface.Name}}SignalHandler {
{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- if .Description }}
    /// {{ .Description }}
{{- end }}
    pub {{snake .Name}}_changed: Signal<({{ rsType "" .}},)>,
{{- end }}
{{- if and (len .Interface.Signals) (len .Interface.Properties) }}{{nl}}{{end}}
{{- range $i, $e := .Interface.Signals }}
{{- if $i }}{{nl}}{{ end }}
{{- if .Description }}
    /// {{ .Description }}
{{- end }}
{{- $lenParams := len .Params }}
    pub {{snake .Name}}: Signal<(
        {{- range $i, $e := .Params }}
        {{- if $i }}, {{ end }}{{ rsType "" .}}{{ end }}{{- if eq 1 $lenParams }},{{ end -}}
        )>,
{{- end }}
}{{ nl }}
{{- end }}

{{- if len .Interface.Operations }}
#[async_trait]{{ nl }}
{{- end -}}
pub trait {{Camel .Interface.Name}}Trait {
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
        {{rsParams "" "" ",\n        " $operation.Params}},
    ){{- if not .Return.IsVoid }} -> {{ rsReturn "" $operation.Return}}{{- end }};
{{- else }}
    fn {{snake $operation.Name }}(&mut self){{- if not .Return.IsVoid }} -> {{ rsReturn "" $operation.Return}}{{- end }};
{{- end }}
    /// Asynchronous version of [{{ snake $operation.Name}}]({{Camel $interface.Name}}Trait::{{ snake $operation.Name}})
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
    ) -> Result<{{rsReturn "" $operation.Return}}, ()>;
{{- else }}
    async fn {{snake $operation.Name }}_async(&mut self) -> Result<{{rsReturn "" $operation.Return}}, ()>;
{{- end }}
{{- end }}   {{- /* end range operations */}}

{{- if len .Interface.Operations }}{{- if len .Interface.Properties }}{{- nl }}{{ end }}{{ end }}

{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- $property := . }}
    /// Gets the value of the {{$property.Name}} property.
    {{- if $property.Description }}
    /// {{$property.Description}}
    {{- end }}    {{- /* end if property.Description */}}
    fn {{snake $property.Name }}(&self) -> {{rsTypeRef "" $property}};
    {{- if not .IsReadOnly }}
    /// Sets the value of the {{$property.Name}} property.
    {{- if $property.Description }}
    /// {{$property.Name}} {{$property.Description}}
    {{- end }}    {{- /* end if property.Description */}}
    fn set_{{snake $property.Name}}(
        &mut self,
        {{ rsParam "" "" $property }},
    );
    {{- end }}
{{- end }}    {{- /* end range properties */}}

{{- if or (len .Interface.Signals) (len .Interface.Properties) }}

    fn _get_signal_handler(&mut self) -> &{{Camel .Interface.Name}}SignalHandler;
{{- end }}
}
