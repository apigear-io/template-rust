//! NATS service example: exposes the generated ManyParamInterface implementation over NATS.
//!
//! Start a server (e.g. `nats-server`), then run this server and, in another terminal,
//! the matching client:
//!     cargo run --bin nats_server
//!     cargo run --bin nats_client
//! Override the server URL with the NATS_URL environment variable (default 127.0.0.1:4222).
use std::sync::Arc;
use std::time::Duration;
use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
use testbed2::implementation::many_param_interface::ManyParamInterface;
use testbed2::nats::many_param_interface_service::ManyParamInterfaceNatsService;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let url = std::env::var("NATS_URL").unwrap_or_else(|_| "127.0.0.1:4222".to_string());
    let nats = async_nats::connect(&url).await.expect("connect to nats-server");

    let object = Arc::new(ManyParamInterface::default());
    let service = Arc::new(ManyParamInterfaceNatsService::new(object.clone() as Arc<dyn ManyParamInterfaceTrait>, nats));
    let _subscription = service.subscribe();
    println!("[many_param_interface-nats-service] serving on {url} (Ctrl-C to stop)");

    // subscribe() announced `service.available` and answers the `init` handshake,
    // so late-joining clients fetch the current state themselves. Stay alive.
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
