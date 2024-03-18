#![allow(unused_imports)]{{nl}}
{{- range .System.Modules -}}
{{- $module := . -}}
extern crate {{snake $module.Name}};{{nl}}
{{- end }}
// importing traits and allowing unused_imports for easy use below
{{- range .System.Modules }}
{{- $module := . }}
{{- range $module.Interfaces }}
{{- $interface := . }}
use {{snake $module.Name}}::api::{{ snake $interface.Name }}::{{ Camel $interface.Name }}Trait as {{snake $module.Name}}{{ Camel $interface.Name }}Trait;
use {{snake $module.Name}}::implementation::{{ snake $interface.Name }}::{{ Camel $interface.Name }} as {{snake $module.Name}}{{ Camel $interface.Name }};
{{- end }}
{{- end }}
use std::io;

fn main() {
{{- range .System.Modules }}
{{- $module := . }}
{{- range $module.Interfaces }}
{{- $interface := . }}
    let mut _test_{{snake $module.Name}}_{{ snake $interface.Name }} = {{snake $module.Name}}{{ Camel $interface.Name }}::default();
{{- end }}
{{- end }}

    let mut cmd = String::new();
    println!("Enter command, or q to exit:");
    while cmd.trim() != "q" {
        cmd.clear();
        io::stdin().read_line(&mut cmd).unwrap();
    }
}
