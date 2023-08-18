{{- /* ***************************************************************** */ -}}
{{- /* *** ENUMERATIONS                                              *** */ -}}
{{- /* ***************************************************************** */ -}}
// Enumerations
{{- range $i, $e := .Module.Enums }}
{{- $enum := . }}
{{- if $i }}{{nl}}{{ end }}
{{- $class := .Name }}
/// Enumeration {{$class}}
 {{- if .Description }}
/// {{.Description}}
{{- end }}
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum {{$class}}Enum {
{{- range $idx, $elem := .Members }}
    {{- if eq .Name $enum.Default.Name }}
    #[default]
    {{- end }}
    {{ upper1 .Name }} = {{ .Value }}, 
    {{- if .Description -}}
    /// .Description
    {{- end }}
{{- end }}
}
// fn to{{ upper1 $class }}Enum(v: u8, ok: *mut bool) -> {{$class}}Enum;
// fn from{{ upper1 $class }}Enum(v: {{$class}}Enum, ok: *mut bool) -> u8;
{{- end }}

{{- if len .Module.Structs}}
{{- nl }}
// Structs
{{- end }}
{{- /* ***************************************************************** */ -}}
{{- /* *** STRUCTS                                                   *** */ -}}
{{- /* ***************************************************************** */ -}}

{{- range $i, $e := .Module.Structs }}
{{- if $i }}{{nl}}{{ end }}
{{- $class := .Name}}
/// Struct {{$class}}
{{- if .Description }}
/// {{.Description}}
{{- end }}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct {{$class}} {
{{- /* members */}}
{{- range  .Fields }}
{{- if .Description }}
     /// {{.Description}}
{{- end }}
    pub {{snake .Name}}: {{ rustType "" . }},
{{- end }}
}
{{- end }}
