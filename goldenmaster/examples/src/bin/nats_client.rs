//! NATS client example: talks to the ManyParamInterface NATS service.
//!
//! Start a server and `nats_server` first, then:
//!     cargo run --bin nats_client
//! Override the server URL with the NATS_URL environment variable (default 127.0.0.1:4222).
#![allow(unused_imports, unused_variables)]
use std::sync::Arc;
use std::time::Duration;
use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
use testbed2::nats::many_param_interface_client::ManyParamInterfaceNatsClient;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let url = std::env::var("NATS_URL").unwrap_or_else(|_| "127.0.0.1:4222".to_string());
    let nats = async_nats::connect(&url).await.expect("connect to nats-server");

    let client = Arc::new(ManyParamInterfaceNatsClient::new(nats));
    let _subscription = client.subscribe();

    // Give the subscriptions time to register and the state to arrive.
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("[many_param_interface-nats-client] connected to {url}");

    // Invoke the first operation via NATS request/reply.
    let result = client.func1(Default::default()).await;
    println!("[many_param_interface-nats-client] func1() -> {:?}", result);

    // Read the property values received from the service.
    println!("  prop1 = {:?}", client.prop1());
    println!("  prop2 = {:?}", client.prop2());
    println!("  prop3 = {:?}", client.prop3());
    println!("  prop4 = {:?}", client.prop4());
}
