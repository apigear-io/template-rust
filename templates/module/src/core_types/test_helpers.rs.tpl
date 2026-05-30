{{- if or .Module.Structs .Module.Enums -}}
#[allow(unused_imports)]
use crate::api::data_structs::*;
{{- end }}
{{- range .Module.Structs }}

/// Creates a {{.Name}} populated with test values.
pub fn fill_test_{{snake .Name}}() -> {{.Name}} {
    {{.Name}} {{`{`}}
{{- range $i, $f := .Fields }}
{{-   if $i }},{{ end }} {{snake $f.Name}}:
{{-   if $f.IsArray }} vec![Default::default()]
{{-   else if eq "string" $f.Type }} String::from("test")
{{-   else if eq "bool" $f.Type }} true
{{-   else if or (eq "int" $f.Type) (eq "int32" $f.Type) (eq "int64" $f.Type) }} 1
{{-   else if or (eq "float" $f.Type) (eq "float32" $f.Type) (eq "float64" $f.Type) }} 1.0
{{-   else }} Default::default()
{{-   end }}
{{- end }} }
}
{{- end }}
