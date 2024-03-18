[workspace]

members = [
{{- range .System.Modules }}
{{- $module_id := snake .Name}}
    "{{$module_id}}",
{{- end }}
{{- if .Features.examples }}
    "examples/app",
{{- end }}
]
