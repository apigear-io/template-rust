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
use crate::core_types::{{snake .Interface.Name}}_data::{{Camel .Interface.Name}}Data;
use objectlink_core::traits::{IClientNode, ObjectSink};
use objectlink_core::types::Name;
use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink client adapter for {{Camel .Interface.Name}}.
/// Implements the interface trait by forwarding operations over the OLink protocol
/// and caching property values locally.
pub struct {{Camel .Interface.Name}}OlinkClient {
    data: RwLock<{{Camel .Interface.Name}}Data>,
    node: RwLock<Option<Arc<dyn IClientNode>>>,
{{- if $hasPubSub }}
    publisher: {{Camel .Interface.Name}}Publisher,
{{- end }}
}

impl {{Camel .Interface.Name}}OlinkClient {
    /// Set the client node used for remote communication.
    /// Must be called before link_remote() to enable operations.
    pub fn set_node(
        &self,
        node: Arc<dyn IClientNode>,
    ) {
        *self.node.write() = Some(node);
    }
}

impl Default for {{Camel .Interface.Name}}OlinkClient {
    fn default() -> Self {
        Self {{`{`}} data: RwLock::new({{Camel .Interface.Name}}Data::default()), node: RwLock::new(None)
{{- if $hasPubSub }}, publisher: {{Camel .Interface.Name}}Publisher::default(){{ end }} }
    }
}

impl {{Camel .Interface.Name}}Trait for {{Camel .Interface.Name}}OlinkClient {{ if $isEmpty }}{}{{ else }}{
{{- range $i, $e := .Interface.Operations }}
{{- if $i }}{{nl}}{{ end }}
{{- $operation := . }}
{{- if len $operation.Params }}
    fn {{snake $operation.Name }}(
        &self,
        {{rsParams "" "" ",\n        " $operation.Params}},
    ) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        let args = json!([{{ range $i, $e := $operation.Params }}{{- if $i }}, {{ end }}{{ rsVar "" .}}{{ end }}]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("{{$.Module.Name}}.{{$.Interface.Name}}", "{{$operation.Name}}"),
                    args,
                    Some(Box::new(move |value| {
                        let _ = tx.send(value);
                    })),
                );
                match rx.recv() {
                    Ok(value) => Ok(serde_json::from_value(value).unwrap_or_default()),
                    Err(_) => Err(ApiError::OperationFailed("no reply".to_string())),
                }
            } else {
                Err(ApiError::NotConnected)
            }
        })
    }
{{- else }}
    fn {{snake $operation.Name }}(&self) -> ApiFuture<'_, Result<{{ rsReturn "" $operation.Return}}, ApiError>> {
        let args = json!([]);
        let node = self.node.read().clone();
        Box::pin(async move {
            if let Some(node) = node {
                let (tx, rx) = std::sync::mpsc::sync_channel(1);
                node.invoke_remote(
                    &Name::create_member_id("{{$.Module.Name}}.{{$.Interface.Name}}", "{{$operation.Name}}"),
                    args,
                    Some(Box::new(move |value| {
                        let _ = tx.send(value);
                    })),
                );
                match rx.recv() {
                    Ok(value) => Ok(serde_json::from_value(value).unwrap_or_default()),
                    Err(_) => Err(ApiError::OperationFailed("no reply".to_string())),
                }
            } else {
                Err(ApiError::NotConnected)
            }
        })
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
        self.data.read().{{ snake $property.Name }}.clone()
        {{- else }}
        self.data.read().{{ snake $property.Name }}
        {{- end }}
    }
    {{- if not .IsReadOnly }}
    fn set_{{snake $property.Name}}(
        &self,
        {{ rsParam "" "" $property }},
    ) {
        if let Some(node) = self.node.read().as_ref() {
            node.set_remote_property(&Name::create_member_id("{{$.Module.Name}}.{{$.Interface.Name}}", "{{$property.Name}}"), json!({{ snake $property.Name }}));
        }
    }
    {{- end }}
{{- end }}

{{- if $hasPubSub }}

    fn publisher(&self) -> &{{Camel .Interface.Name}}Publisher {
        &self.publisher
    }
{{- end }}
}{{ end }}

impl ObjectSink for {{Camel .Interface.Name}}OlinkClient {
    fn olink_object_name(&self) -> &str {
        "{{.Module.Name}}.{{.Interface.Name}}"
    }

    fn olink_on_init(
        &self,
        _object_id: &str,
        props: Value,
        _node: &dyn IClientNode,
    ) {
        if let Ok(data) = serde_json::from_value::<{{Camel .Interface.Name}}Data>(props) {
            *self.data.write() = data;
        }
        // Store node reference - we need to upcast to Arc, but we only have &dyn IClientNode.
        // The node is set separately via set_node() before link_remote().
    }

    fn olink_on_property_changed(
        &self,
        property_id: &str,
        value: Value,
    ) {
        let member = Name::member_name(property_id);
        match member {
{{- range .Interface.Properties }}
            "{{.Name}}" => {
{{- $isPropComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}
                if let Ok(v) = serde_json::from_value::<{{ rsType "" . }}>(value) {
{{- if $hasPubSub }}
                    let _ = self.publisher.{{ snake .Name }}_changed.send(v{{ if $isPropComplex }}.clone(){{ end }});
{{- end }}
                    self.data.write().{{ snake .Name }} = v;
                }
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown property: {}", property_id);
            }
        }
    }

    #[allow(clippy::get_first)]
    fn olink_on_signal(
        &self,
        signal_id: &str,
        args: Value,
    ) {
        let member = Name::member_name(signal_id);
        match member {
{{- range .Interface.Signals }}
{{- $signal := . }}
{{- $lenParams := len .Params }}
            "{{.Name}}" => {
{{- if gt $lenParams 0 }}
                if let Some(arr) = args.as_array() {
                    let _ = self.publisher.{{ snake .Name }}.send((
{{- range $i, $e := .Params }}
{{-     if $i }}, {{ end -}}
serde_json::from_value(arr.get({{ $i }}).cloned().unwrap_or_default()).unwrap_or_default()
{{- end }}
{{- if eq $lenParams 1 }},{{ end -}}
));
                }
{{- else }}
                let _ = self.publisher.{{ snake .Name }}.send(());
{{- end }}
            }
{{- end }}
            _ => {
                tracing::warn!("Unknown signal: {}", signal_id);
            }
        }
    }

    fn olink_on_release(&self) {
        *self.node.write() = None;
    }
}
