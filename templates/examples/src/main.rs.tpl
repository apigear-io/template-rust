//! Local example: use the generated default implementations directly, in-process.
//! Each interface is instantiated and exercised (operations, properties, signals)
//! without any IPC. See `src/bin/{olink,mqtt,nats}_{server,client}.rs` for the IPC
//! examples.
#![allow(unused_imports, unused_variables)]

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("ApiGear Rust SDK — local implementation example");
    println!("===============================================");
{{- range .System.Modules }}
{{- $module := . }}
{{- range $module.Interfaces }}
{{- $interface := . }}
{{- $hasOps := len $interface.Operations }}
{{- $hasSignals := len $interface.Signals }}
    {
        use {{snake $module.Name}}::api::{{snake $interface.Name}}::{{Camel $interface.Name}}Trait;
{{- if $hasOps }}
        use {{snake $module.Name}}::api::{{snake $interface.Name}}::{{Camel $interface.Name}}TraitAsync;
{{- end }}
        use {{snake $module.Name}}::implementation::{{snake $interface.Name}}::{{Camel $interface.Name}};
{{- if or $module.Structs $module.Enums }}
        #[allow(unused_imports)]
        use {{snake $module.Name}}::api::data_structs::*;
{{- end }}
        println!("\n== {{$module.Name}}.{{Camel $interface.Name}} ==");
        let object = {{Camel $interface.Name}}::default();
{{- range $i, $e := $interface.Operations }}
{{- if not $i }}
        // call the first operation through the async wrapper
        let _ = object.{{snake .Name }}_async(
{{- range $j, $p := .Params }}
{{-   if $j }}, {{ end -}}
{{-   $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{    if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end -}}
Default::default()
{{- end -}}
).await;
        println!("  called {{snake .Name}}()");
{{- end }}
{{- end }}
{{- range $interface.Properties }}
{{- if not .IsReadOnly }}
{{- $property := . }}
{{- $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) }}
{{- if .IsArray }}
        let value: {{rsType "" $property}} = vec![Default::default()];
{{- else if eq .Type "bool" }}
        let value: {{rsType "" $property}} = true;
{{- else if or (eq .Type "int") (eq .Type "int32") }}
        let value: {{rsType "" $property}} = 42i32;
{{- else if eq .Type "int64" }}
        let value: {{rsType "" $property}} = 42i64;
{{- else if or (eq .Type "float") (eq .Type "float32") }}
        let value: {{rsType "" $property}} = 4.2f32;
{{- else if eq .Type "float64" }}
        let value: {{rsType "" $property}} = 4.2f64;
{{- else if eq .Type "string" }}
        let value: {{rsType "" $property}} = String::from("hello");
{{- else }}
        let value: {{rsType "" $property}} = Default::default();
{{- end }}
        object.set_{{snake $property.Name }}({{ if and $isComplex (not .IsArray) (ne "string" .Type) }}&{{ end }}value{{ if and $isComplex (not .IsArray) (ne "string" .Type) }}.clone(){{ end }}{{ if .IsArray }}.as_slice(){{ end }}{{ if and (eq "string" .Type) (not .IsArray) }}.as_str(){{ end }});
        println!("  {{snake $property.Name}} = {:?}", object.{{snake $property.Name }}());
{{- end }}
{{- end }}
{{- if $hasSignals }}
        let _publisher = object.publisher();
        println!("  {{$hasSignals}} signal(s) available via publisher()");
{{- end }}
    }
{{- end }}
{{- end }}

    println!("\nAll interfaces exercised locally.");
}
