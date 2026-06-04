---
sidebar_label: "NATS"
title: "Rust Template NATS Networking - ApiGear"
description: "NATS networking in the ApiGear Rust template: connect Rust interface clients and services over the NATS messaging system."
keywords: [rust, apigear, objectapi, nats, networking, messaging]
sidebar_position: 6
---

import CodeBlock from '@theme/CodeBlock';
import helloWorldModuleComponent from '!!raw-loader!./data/helloworld.module.yaml';

# NATS

This feature provides a _client_ and a _service_ adapter for your interfaces over the [NATS](https://nats.io/) protocol, built on the [`async-nats`](https://docs.rs/async-nats) crate. It lets you connect applications built with the same or different technologies — check all of our [templates](/docs/sdk/intro) and the NATS feature in other templates that support it.

- Use a _NATS client_ in place of your local implementation to receive data from a remote service.
- Use a _NATS service adapter_ to expose your implementation as a remote service.

:::note
This feature requires `api` and `core`.
:::

:::tip
The NATS server is not part of the template. To run a client and a service (both connect as NATS clients) you need a [nats-server](https://nats.io/download/) reachable by both.
:::

## File overview for module

With our example API definition

<details>
  <summary>Hello World API (click to expand)</summary>
  <CodeBlock language="yaml" showLineNumbers>{helloWorldModuleComponent}</CodeBlock>
</details>

the following files are generated. The purpose and content of each file is explained below.

```bash {7,8,13}
📂io_world
 ┣ 📂src
 ┃ ┣ 📂nats
 ┃ ┃ ┣ 📜mod.rs
 ┃ ┃ ┣ 📜hello_client.rs    # NATS client adapter for Hello
 ┃ ┃ ┗ 📜hello_service.rs   # NATS service adapter for Hello
 ┃ ┗ 📜lib.rs
 ┣ 📂tests
 ┃ ┣ 📜nats_common.rs       # server test helper
 ┃ ┗ 📜nats_hello_test.rs   # round-trip tests for Hello
 ...
```

The adapters use NATS request/reply for operations and dedicated subjects for properties, signals and the service's state.

## NATS client adapter

The file `📜hello_client.rs` contains `HelloNatsClient`, the NATS client version of the `Hello` interface. It implements `HelloTrait`, so you use it like a local implementation. It takes a connected [`async-nats`](https://docs.rs/async-nats) client and subscribes to the interface's subjects.

```rust
let nats = async_nats::connect("127.0.0.1:4222").await?;
let client = Arc::new(HelloNatsClient::new(nats));
let _subscription = client.subscribe();
```

#### Properties

A getter (here `last()`) returns the locally cached value last received from the service. A setter (here `set_last()`) sends a change request; the local value updates when the service confirms the change. Subscribe to changes through the [`Publisher`](api.md#publisher) returned by `publisher()`.

#### Operations

Operations use NATS request/reply — the call sends a request and awaits the reply:

```rust
let result = client.say(&message, WhenEnum::Now).await;
```

#### Signals

Do not emit signals from a client. Subscribe to signals through the [`Publisher`](api.md#publisher); incoming signal messages are delivered on the matching `broadcast` channel.

#### Connectivity

The client's `subscribe()` spawns a background task that subscribes to the property (`apigear.io.world.Hello.prop.*`), signal (`apigear.io.world.Hello.sig.*`) and state (`apigear.io.world.Hello.state`) subjects, and keeps the cache in sync as messages arrive. The service publishes its full state on the `.state` subject via `publish_state()`. Because NATS does not retain messages, a client that connects after the service published only sees the state if the service publishes it again — the generated `nats_server` example re-publishes the state periodically so late-joining clients still receive it.

## NATS service adapter

The file `📜hello_service.rs` contains `HelloNatsService`, which wraps a local `Hello` implementation and exposes it over NATS. It applies incoming operation and property-change requests to your local object and publishes property changes and signals back to clients.

- **Properties** — a change on your local object (or a client request) is published to all clients.
- **Operations** — a request is run on your local object; the result is returned only to the requesting client.
- **Signals** — a signal emitted by your local object is forwarded to all clients.

## Use the adapters

The generated `examples` crate ships ready-to-run `nats_server` and `nats_client` binaries. The client connects to the server, hands the connection to the adapter, and subscribes:

```rust
use std::sync::Arc;
use std::time::Duration;

let nats = async_nats::connect("127.0.0.1:4222").await.expect("connect to nats-server");

let client = Arc::new(HelloNatsClient::new(nats));
let _subscription = client.subscribe();

// Give the subscriptions and state exchange a moment.
tokio::time::sleep(Duration::from_millis(500)).await;

// Use the client like a local Hello implementation:
let result = client.say(&Default::default(), WhenEnum::Now).await;
println!("say() -> {result:?}");
```

Start a server, then run the two binaries in separate terminals (override the server URL with the `NATS_URL` environment variable, default `127.0.0.1:4222`):

```bash
nats-server -p 4222 &
cargo run -p rust_hello_world_examples --bin nats_server
cargo run -p rust_hello_world_examples --bin nats_client
```

## Tests

The NATS feature generates round-trip tests in `📜tests/nats_hello_test.rs`, backed by the helper in `📜nats_common.rs`. They exercise a real client ↔ service round-trip over a live server, so they are marked `#[ignore]` and skipped by default. Run them against a server the way CI does:

```bash
nats-server -p 4222 &
cargo test --manifest-path goldenmaster/Cargo.toml -- --ignored
```
