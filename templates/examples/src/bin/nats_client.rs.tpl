{{- $module := index .System.Modules 0 }}
{{- $interface := index $module.Interfaces 0 -}}
//! NATS client example: talks to the {{Camel $interface.Name}} NATS service.
//!
//! Start a server and `nats_server` first, then:
//!     cargo run --bin nats_client
//! Override the server URL with the NATS_URL environment variable (default 127.0.0.1:4222).
#![allow(unused_imports, unused_variables)]
use std::sync::Arc;
use std::time::Duration;
use {{snake $module.Name}}::api::{{snake $interface.Name}}::{{Camel $interface.Name}}Trait;
use {{snake $module.Name}}::nats::{{snake $interface.Name}}_client::{{Camel $interface.Name}}NatsClient;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let url = std::env::var("NATS_URL").unwrap_or_else(|_| "127.0.0.1:4222".to_string());
    let nats = async_nats::connect(&url).await.expect("connect to nats-server");

    let client = Arc::new({{Camel $interface.Name}}NatsClient::new(nats));
    let _subscription = client.subscribe();

    // Give the subscriptions time to register and the state to arrive.
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("[{{snake $interface.Name}}-nats-client] connected to {url}");
{{- range $i, $e := $interface.Operations }}
{{- if not $i }}

    // Invoke the first operation via NATS request/reply.
    let result = client.{{snake .Name }}(
{{- range $j, $p := .Params }}
{{-   if $j }}, {{ end -}}
{{-   $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{    if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end -}}
Default::default()
{{- end -}}
).await;
    println!("[{{snake $interface.Name}}-nats-client] {{snake .Name}}() -> {:?}", result);
{{- end }}
{{- end }}
{{- if len $interface.Properties }}

    // Read the property values received from the service.
{{- range $interface.Properties }}
    println!("  {{snake .Name}} = {:?}", client.{{snake .Name }}());
{{- end }}
{{- end }}
}
