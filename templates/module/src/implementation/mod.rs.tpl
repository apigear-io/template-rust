{{- range .Module.Interfaces -}}
pub mod {{snake .Name}};{{ nl }}
{{- end -}}
