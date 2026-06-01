//! OLink service example: exposes the generated ManyParamInterface implementation over a
//! TCP socket carrying newline-delimited JSON ObjectLink messages.
//!
//!     cargo run --bin olink_server
//!     cargo run --bin olink_client
//! Override the address with the OLINK_ADDR environment variable (default 127.0.0.1:8182).
use objectlink_core::remote_node::RemoteNode;
use objectlink_core::remote_registry::RemoteRegistry;
use objectlink_core::traits::ObjectSource;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
use testbed2::implementation::many_param_interface::ManyParamInterface;
use testbed2::olink::many_param_interface_service::ManyParamInterfaceOlinkService;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let addr = std::env::var("OLINK_ADDR").unwrap_or_else(|_| "127.0.0.1:8182".to_string());
    let listener = TcpListener::bind(&addr).await.expect("bind");
    println!("[many_param_interface-olink-server] listening on {addr} (Ctrl-C to stop)");

    let object = Arc::new(ManyParamInterface::default());
    let service: Arc<dyn ObjectSource> = Arc::new(ManyParamInterfaceOlinkService::new(object.clone() as Arc<dyn ManyParamInterfaceTrait>));
    let registry = Arc::new(RemoteRegistry::new());
    registry.add_source(Arc::downgrade(&service));

    loop {
        let (socket, peer) = listener.accept().await.expect("accept");
        println!("[many_param_interface-olink-server] client connected: {peer}");
        let (rd, mut wr) = socket.into_split();
        let node = Arc::new(RemoteNode::new(registry.clone()));

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        node.on_write(Box::new(move |msg: &str| {
            let _ = tx.send(msg.to_string());
        }));
        tokio::spawn(async move {
            while let Some(m) = rx.recv().await {
                if wr.write_all(m.as_bytes()).await.is_err() {
                    break;
                }
                let _ = wr.write_all(b"\n").await;
                let _ = wr.flush().await;
            }
        });

        let mut lines = BufReader::new(rd).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if line.trim().is_empty() {
                continue;
            }
            node.handle_message(&line);
        }
        println!("[many_param_interface-olink-server] client disconnected");
    }
}
