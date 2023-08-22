{{- if len .Module.Enums -}}
use std::convert::TryFrom;{{nl}}
{{- end }}
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
#[repr(u8)]
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

impl TryFrom<u8> for {{$class}}Enum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
{{- range $idx, $elem := .Members }}
            {{ .Value }} => Ok({{$class}}Enum::{{ upper1 .Name }}), 
{{- end }}
            _ => Err(()),
        }
    }
}
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
