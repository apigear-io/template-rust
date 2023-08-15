{{- if or .Module.Structs .Module.Enums -}}
pub mod data_structs;{{ nl }}
{{- end }}
{{- range .Module.Interfaces -}}
pub mod {{snake .Name}};{{ nl }}
{{- end -}}
