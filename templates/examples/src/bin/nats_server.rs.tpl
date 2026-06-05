{{- $module := index .System.Modules 0 }}
{{- $interface := index $module.Interfaces 0 -}}
//! NATS service example: exposes the generated {{Camel $interface.Name}} implementation over NATS.
//!
//! Start a server (e.g. `nats-server`), then run this server and, in another terminal,
//! the matching client:
//!     cargo run --bin nats_server
//!     cargo run --bin nats_client
//! Override the server URL with the NATS_URL environment variable (default 127.0.0.1:4222).
use std::sync::Arc;
use std::time::Duration;
use {{snake $module.Name}}::api::{{snake $interface.Name}}::{{Camel $interface.Name}}Trait;
use {{snake $module.Name}}::implementation::{{snake $interface.Name}}::{{Camel $interface.Name}};
use {{snake $module.Name}}::nats::{{snake $interface.Name}}_service::{{Camel $interface.Name}}NatsService;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let url = std::env::var("NATS_URL").unwrap_or_else(|_| "127.0.0.1:4222".to_string());
    let nats = async_nats::connect(&url).await.expect("connect to nats-server");

    let object = Arc::new({{Camel $interface.Name}}::default());
    let service = Arc::new({{Camel $interface.Name}}NatsService::new(object.clone() as Arc<dyn {{Camel $interface.Name}}Trait>, nats));
    let _subscription = service.subscribe();
    println!("[{{snake $interface.Name}}-nats-service] serving on {url} (Ctrl-C to stop)");

    // subscribe() announced `service.available` and answers the `init` handshake,
    // so late-joining clients fetch the current state themselves. Stay alive.
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
