{{- $hasOps := len .Interface.Operations }}
{{- $hasProps := len .Interface.Properties }}
{{- $hasSignals := len .Interface.Signals }}
{{- $hasPubSub := or $hasSignals $hasProps }}
{{- $isEmpty := and (not $hasOps) (not $hasPubSub) }}
{{- if $isEmpty }}#![allow(unused_imports, dead_code, clippy::never_loop)]{{ nl }}{{ end }}
{{- if or .Module.Structs .Module.Enums -}}
#[allow(unused_imports)]
use crate::api::data_structs::*;
{{ end -}}
{{- if $hasOps -}}
use apigear::{ApiError, ApiFuture};
{{ end -}}
{{- if $hasPubSub -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Publisher;
{{ end -}}
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
use tracing;

/// Trace decorator for {{Camel .Interface.Name}}.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct {{Camel .Interface.Name}}Traced<T: {{Camel .Interface.Name}}Trait> {
    inner: T,
}

impl<T: {{Camel .Interface.Name}}Trait> {{Camel .Interface.Name}}Traced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: {{Camel .Interface.Name}}Trait> {{Camel .Interface.Name}}Trait for {{Camel .Interface.Name}}Traced<T> {{ if $isEmpty }}{}{{ else }}{
{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
{{- if len $operation.Params }}
    fn {{snake $operation.Name }}(
        &self,
        {{rsParams "" "" ",\n        " $operation.Params}},
    ) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        tracing::info!("{{Camel $.Interface.Name}}::{{snake $operation.Name}} called");
        self.inner.{{snake $operation.Name}}({{ range $i, $e := $operation.Params }}{{- if $i }}, {{ end }}{{ rsVar "" .}}{{ end }})
    }
{{- else }}
    fn {{snake $operation.Name }}(&self) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        tracing::info!("{{Camel $.Interface.Name}}::{{snake $operation.Name}} called");
        self.inner.{{snake $operation.Name}}()
    }
{{- end }}
{{- end }}

{{- if $hasOps }}{{- if $hasProps }}{{- nl }}{{ end }}{{ end }}

{{- range $i, $e := .Interface.Properties }}
{{- if $i }}{{nl}}{{ end }}
{{- $property := . }}
    fn {{snake $property.Name }}(&self) -> {{rsType "" $property}} {
        self.inner.{{ snake $property.Name }}()
    }
    {{- if not .IsReadOnly }}
    fn set_{{snake $property.Name}}(
        &self,
        {{ rsParam "" "" $property }},
    ) {
        tracing::info!("{{Camel $.Interface.Name}}::set_{{snake $property.Name}} called");
        self.inner.set_{{ snake $property.Name }}({{ snake $property.Name }});
    }
    {{- end }}
{{- end }}

{{- if $hasPubSub }}

    fn publisher(&self) -> &{{Camel .Interface.Name}}Publisher {
        self.inner.publisher()
    }
{{- end }}
}{{ end }}
