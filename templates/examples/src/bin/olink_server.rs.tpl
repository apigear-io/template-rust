{{- $module := index .System.Modules 0 }}
{{- $interface := index $module.Interfaces 0 -}}
//! OLink service example: exposes the generated {{Camel $interface.Name}} implementation over a
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
use {{snake $module.Name}}::api::{{snake $interface.Name}}::{{Camel $interface.Name}}Trait;
use {{snake $module.Name}}::implementation::{{snake $interface.Name}}::{{Camel $interface.Name}};
use {{snake $module.Name}}::olink::{{snake $interface.Name}}_service::{{Camel $interface.Name}}OlinkService;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let addr = std::env::var("OLINK_ADDR").unwrap_or_else(|_| "127.0.0.1:8182".to_string());
    let listener = TcpListener::bind(&addr).await.expect("bind");
    println!("[{{snake $interface.Name}}-olink-server] listening on {addr} (Ctrl-C to stop)");

    let object = Arc::new({{Camel $interface.Name}}::default());
    let service: Arc<dyn ObjectSource> = Arc::new({{Camel $interface.Name}}OlinkService::new(object.clone() as Arc<dyn {{Camel $interface.Name}}Trait>));
    let registry = Arc::new(RemoteRegistry::new());
    registry.add_source(Arc::downgrade(&service));

    loop {
        let (socket, peer) = listener.accept().await.expect("accept");
        println!("[{{snake $interface.Name}}-olink-server] client connected: {peer}");
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
        println!("[{{snake $interface.Name}}-olink-server] client disconnected");
    }
}
