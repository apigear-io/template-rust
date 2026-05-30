{{- if or .Module.Structs .Module.Enums -}}
#[allow(unused_imports)]
use crate::api::data_structs::*;
{{ end -}}
use serde::{Deserialize, Serialize};

/// Bundles all properties of {{Camel .Interface.Name}} for state synchronization.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
{{- if .Interface.Properties }}
pub struct {{Camel .Interface.Name}}Data {
{{- range .Interface.Properties }}
    pub {{snake .Name}}: {{rsType "" .}},
{{- end }}
}
{{- else }}
pub struct {{Camel .Interface.Name}}Data {}
{{- end }}
