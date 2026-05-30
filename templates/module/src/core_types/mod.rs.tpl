{{- range .Module.Interfaces -}}
pub mod {{snake .Name}}_data;
pub mod {{snake .Name}}_shared;
{{ end -}}
pub mod test_helpers;
