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
use std::pin::Pin;
use std::future::Future;{{ nl }}
{{- end }}

{{- if or $data_structs $ops }}{{ nl }}{{ end -}}

pub trait {{Camel .Interface.Name}}Trait {
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
        {{rustParams "" "" ",\n        " $operation.Params}},
    ){{- if not .Return.IsVoid }} -> {{ rustReturn "" $operation.Return}}{{- end }};
{{- else }}
    fn {{snake $operation.Name }}(&mut self){{- if not .Return.IsVoid }} -> {{ rustReturn "" $operation.Return}}{{- end }};
{{- end }}
    /// Asynchronous version of `{{ snake $operation.Name}}`
{{- if $operation.Description }}
    * {{$operation.Description}}
{{- end }}   {{- /* end if description */}}
{{- range $operation.Params }}
{{- $param := . }}
{{- if $param.Description }}
    /// `{{$param}}` {{$param.Description}}
{{- end }}   {{- /* end if description */}}
{{- end }}   {{- /* end range operation params */}}
    /// returns future of type {{rustReturn "" $operation.Return}} which is set once the function has completed
{{- if len $operation.Params }}
    fn {{snake $operation.Name }}_async(
        &mut self,
        {{rustParams "" "" ",\n        " $operation.Params}},
    ) -> Pin<Box<dyn Future<Output = Result<{{rustReturn "" $operation.Return}}, ()>> + Unpin>>;
{{- else }}
    fn {{snake $operation.Name }}_async(&mut self) -> Pin<Box<dyn Future<Output = Result<{{rustReturn "" $operation.Return}}, ()>> + Unpin>>;
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
    fn {{snake $property.Name }}(&self) -> {{rustTypeRef "" $property}};
    {{- if not .IsReadOnly }}
    /// Sets the value of the {{$property.Name}} property.
    {{- if $property.Description }}
    /// {{$property.Name}} {{$property.Description}}
    {{- end }}    {{- /* end if property.Description */}}
    fn set_{{snake $property.Name}}(
        &mut self,
        {{ rustParam "" "" $property }},
    );
    {{- end }}
{{- end }}    {{- /* end range properties */}}
}
