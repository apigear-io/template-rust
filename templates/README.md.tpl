{{- range .System.Modules }}
{{- $ModuleName := snake .Name}}
{{-  range .Interfaces }}
Say hello world to {{ $ModuleName }} {{snake .Name}}
{{- end }}
{{- end -}}
