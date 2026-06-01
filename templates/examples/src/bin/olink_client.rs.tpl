{{- $module := index .System.Modules 0 }}
{{- $interface := index $module.Interfaces 0 -}}
//! OLink client example: links the {{Camel $interface.Name}} object from the OLink service over a
//! TCP socket carrying newline-delimited JSON ObjectLink messages.
//!
//! Start `olink_server` first, then:
//!     cargo run --bin olink_client
//! Override the address with the OLINK_ADDR environment variable (default 127.0.0.1:8182).
#![allow(unused_imports, unused_variables)]
use objectlink_core::client_node::ClientNode;
use objectlink_core::client_registry::ClientRegistry;
use objectlink_core::traits::ObjectSink;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use {{snake $module.Name}}::api::{{snake $interface.Name}}::{{Camel $interface.Name}}Trait;
use {{snake $module.Name}}::olink::{{snake $interface.Name}}_client::{{Camel $interface.Name}}OlinkClient;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let addr = std::env::var("OLINK_ADDR").unwrap_or_else(|_| "127.0.0.1:8182".to_string());
    let socket = TcpStream::connect(&addr).await.expect("connect");
    let (rd, mut wr) = socket.into_split();

    let registry = Arc::new(ClientRegistry::new());
    let node = Arc::new(ClientNode::new(registry));

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

    let client = Arc::new({{Camel $interface.Name}}OlinkClient::default());
    client.set_node(node.clone());
    let sink: Arc<dyn ObjectSink> = client.clone();
    node.link_remote(&sink);

    let read_node = node.clone();
    tokio::spawn(async move {
        let mut lines = BufReader::new(rd).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if line.trim().is_empty() {
                continue;
            }
            read_node.handle_message(&line);
        }
    });

    // Give the link handshake time to complete and the initial state to arrive.
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("[{{snake $interface.Name}}-olink-client] linked to {addr}");
{{- range $i, $e := $interface.Operations }}
{{- if not $i }}

    // Invoke the first operation over OLink.
    let _ = client.{{snake .Name }}(
{{- range $j, $p := .Params }}
{{-   if $j }}, {{ end -}}
{{-   $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{    if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end -}}
Default::default()
{{- end -}}
).await;
    println!("[{{snake $interface.Name}}-olink-client] called {{snake .Name}}()");
{{- end }}
{{- end }}
{{- if len $interface.Properties }}

    // Read the property values received from the service.
{{- range $interface.Properties }}
    println!("  {{snake .Name}} = {:?}", client.{{snake .Name }}());
{{- end }}
{{- end }}
}
