#![allow(unused_imports, unused_variables)]
{{- range .System.Modules }}
{{- $module := . }}
extern crate {{snake $module.Name}};
{{- end }}

{{- range .System.Modules }}
{{- $module := . }}
{{- range $module.Interfaces }}
{{- $interface := . }}
use {{snake $module.Name}}::api::{{ snake $interface.Name }}::{{ Camel $interface.Name }}Trait as {{snake $module.Name}}_{{ Camel $interface.Name }}Trait;
use {{snake $module.Name}}::implementation::{{ snake $interface.Name }}::{{ Camel $interface.Name }} as {{snake $module.Name}}_{{ Camel $interface.Name }};
use {{snake $module.Name}}::core_types::{{ snake $interface.Name }}_shared::new_shared_{{ snake $interface.Name }} as new_shared_{{snake $module.Name}}_{{ snake $interface.Name }};
{{- end }}
{{- end }}

fn main() {
    println!("ApiGear Rust SDK Examples");
    println!("========================");

    // Create default implementations
{{- range .System.Modules }}
{{- $module := . }}
{{- range $module.Interfaces }}
{{- $interface := . }}
    let _{{snake $module.Name}}_{{ snake $interface.Name }} = {{snake $module.Name}}_{{ Camel $interface.Name }}::default();
{{- end }}
{{- end }}

    // Create shared (Arc<dyn Trait>) implementations
{{- range .System.Modules }}
{{- $module := . }}
{{- range $module.Interfaces }}
{{- $interface := . }}
    let _shared_{{snake $module.Name}}_{{ snake $interface.Name }} = new_shared_{{snake $module.Name}}_{{ snake $interface.Name }}();
{{- end }}
{{- end }}

    println!("All interfaces instantiated successfully.");
}
