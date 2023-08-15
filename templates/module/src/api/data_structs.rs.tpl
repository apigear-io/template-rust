{{- /* ***************************************************************** */ -}}
{{- /* *** ENUMERATIONS                                              *** */ -}}
{{- /* ***************************************************************** */ -}}
// Enumerations
{{- range $i, $e := .Module.Enums }}
{{- if $i }}{{nl}}{{ end }}
{{- $class := .Name }}
/// Enumeration {{$class}}
 {{- if .Description }}
/// {{.Description}}
{{- end }}
#[derive(Copy, Clone)]
pub enum {{$class}}Enum {
{{- range $idx, $elem := .Members }}
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
#[derive(Default)]
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
