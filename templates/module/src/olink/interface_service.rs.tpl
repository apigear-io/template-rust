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
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for {{Camel .Interface.Name}}.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct {{Camel .Interface.Name}}OlinkService {
    impl_: Arc<dyn {{Camel .Interface.Name}}Trait>,
}

impl {{Camel .Interface.Name}}OlinkService {
    pub fn new(impl_: Arc<dyn {{Camel .Interface.Name}}Trait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for {{Camel .Interface.Name}}OlinkService {
    fn olink_object_name(&self) -> &str {
        "{{.Module.Name}}.{{.Interface.Name}}"
    }

    #[allow(clippy::get_first)]
    fn olink_invoke(
        &self,
        method_id: &str,
        args: Value,
    ) -> Value {
        let member = Name::member_name(method_id);
        #[allow(unused_variables)]
        let arr = args.as_array();
        match member {
{{- range .Interface.Operations }}
{{- $operation := . }}
            "{{$operation.Name}}" => {
{{- range $i, $e := $operation.Params }}
                let param_{{ $i }}: {{ rsType "" . }} = serde_json::from_value(arr.and_then(|a| a.get({{ $i }}).cloned()).unwrap_or_default()).unwrap_or_default();
{{- end }}
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.{{snake $operation.Name}}(
{{- range $i, $e := $operation.Params }}
{{-     $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{-     if $i }}, {{ end -}}
{{      if and $isComplex (not .IsArray) (ne "string" .Type) }}&{{ end -}}
param_{{ $i }}
{{- if .IsArray }}.as_slice(){{ end -}}
{{- if and (eq "string" .Type) (not .IsArray) }}.as_str(){{ end -}}
{{- end -}}
))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown method: {}", method_id);
                json!(null)
            }
        }
    }

    fn olink_set_property(
        &self,
        property_id: &str,
        value: Value,
    ) {
        let member = Name::member_name(property_id);
        match member {
{{- range .Interface.Properties }}
{{- if not .IsReadOnly }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}
            "{{.Name}}" => {
                if let Ok(v) = serde_json::from_value::<{{ rsType "" . }}>(value) {
                    self.impl_.set_{{ snake .Name }}({{ if $isComplex }}&{{ end }}v);
                }
            }
{{- end }}
{{- end }}
            _ => {
                tracing::warn!("Unknown property: {}", property_id);
            }
        }
    }

    fn olink_linked(
        &self,
        _object_id: &str,
        _node: &dyn IRemoteNode,
    ) {
        tracing::info!("{{Camel .Interface.Name}} linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("{{Camel .Interface.Name}} unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
{{- if .Interface.Properties }}
        json!({
{{- range $i, $e := .Interface.Properties }}
{{- if $i }},{{ end }}
            "{{.Name}}": self.impl_.{{ snake .Name }}()
{{- end }}
        })
{{- else }}
        json!({})
{{- end }}
    }
}
