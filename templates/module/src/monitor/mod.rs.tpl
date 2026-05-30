{{- range .Module.Interfaces -}}
pub mod {{snake .Name}}_traced;
{{ end -}}
