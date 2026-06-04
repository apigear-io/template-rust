---
sidebar_label: "Features"
title: "Rust Template Features - ApiGear"
description: "Overview of the Rust template features for the ApiGear code generator: generated interfaces and data types, stub implementations, networking adapters, and traffic monitoring."
keywords: [rust, apigear, objectapi, template features, code generator]
---

import CodeBlock from '@theme/CodeBlock';
import helloWorldModuleComponent from '!!raw-loader!./data/helloworld.module.yaml';

# Features

This guide explains how to use the generated code, what features are available, and their purpose.

:::info
A feature is a part of the template that generates a specific aspect of the code. For example, the `api` feature generates the interface traits and the publisher, while the `stubs` feature generates a default implementation you can build on.
:::

## Get started

This template generates a [Cargo](https://doc.rust-lang.org/cargo/) workspace for pure _Rust_ projects. To successfully compile and use the code you need a working [Rust toolchain](https://www.rust-lang.org/tools/install) (`rustc` and `cargo`, 1.80 or newer). The generated code is asynchronous and built on [tokio](https://tokio.rs/).

:::note
Basic Rust knowledge is necessary.
:::

### Code generation

Follow the documentation for [code generation](/docs/guide/quick-start) in general and the [CLI](/docs/cli/generate) or the [Studio](/docs/studio/intro) tools.
Or try the [quick start guide](../quickstart/index.md) first, which shows how to prepare an API and generate code from it.

:::tip
For questions regarding the template please go to our [discussions page](https://github.com/orgs/apigear-io/discussions). For feature requests or bug reports please use our [issue tracker](https://github.com/apigear-io/template-rust/issues).
:::

### Example API

The following code snippet contains the _API_ definition used throughout this guide to demonstrate the generated code and its usage.

<details>
    <summary>Hello World API (click to expand)</summary>
    <CodeBlock language="yaml" showLineNumbers>{helloWorldModuleComponent}</CodeBlock>
</details>

## Features

### Core Features

The core features generate a working view of your _API_ definition. They are self-contained: the `api` and `core` features do **not** depend on any transport or IPC crate, so you can use the generated module crate in a pure in-process application.

- [api](api.md) - generates the interface traits for your _API_, with awaitable operations and a `Publisher` (built on tokio channels) that exposes property changes and signals. Also generates the `serde`-enabled data structs and enums (`Serialize`/`Deserialize`, `TryFrom<u8>` for enums) defined in your module, and the shared `ApiError` and `ApiFuture` types.
- [core](api.md#data-types-core) - generates per-interface support types under `core_types/`: a property-bundle struct for state synchronization, a shared-reference alias and constructor (`Arc<dyn Trait>`), and a test helper.
- [stubs](stubs.md) - adds a ready-to-use default implementation of each interface trait, the workspace `Cargo.toml`, the `examples` crate, and per-interface unit tests. This is a good starting point for your own implementation.

:::note
The `stubs` feature requires `api` and `core`.
:::

### Extended Features

The extended features build on top of `api` and `core` and add more functionality, like monitoring or sharing your data over the network (see [olink](olink.md), [mqtt](mqtt.md), [nats](nats.md)).

- [monitor](monitor.md) - generates a [`tracing`](https://docs.rs/tracing) decorator that wraps any implementation and logs all operations and state changes to the [CLI](/docs/cli/intro) or the [Studio](/docs/studio/intro).
- [olink](olink.md) - provides a client and a service adapter for each interface that can be connected to any of the other technology templates with support for [ObjectLink](/docs/protocols/objectlink/intro). Use this feature to connect with the ApiGear simulation tools. Includes in-process loopback round-trip tests.
- [mqtt](mqtt.md) - provides client and service adapters for each interface over the [MQTT](https://mqtt.org/) protocol (via the [`rumqttc`](https://docs.rs/rumqttc) crate). Check also MQTT in other technology templates that support it. Includes broker-backed integration tests.
- [nats](nats.md) - provides client and service adapters for each interface over the [NATS](https://nats.io/) protocol (via the [`async-nats`](https://docs.rs/async-nats) crate). Check also NATS in other technology templates that support it. Includes server-backed integration tests.
- examples - a shared `examples` crate with runnable programs: a local example exercising every interface in-process, plus `*_server` and `*_client` binaries for each IPC transport (OLink over TCP, MQTT and NATS over a broker/server). Run them with `cargo run`.

Each feature can be selected using the solution file or via the command line tool.

:::note
_Features are case sensitive, make sure to always **use lower-case.**_
:::

:::tip
The _meta_ feature `all` enables all specified features of the template. If you want to see the full extent of the generated code, `all` is the easiest solution.
Please note, `all` is part of the code generator and not explicitly defined within templates.
:::

## Folder structure

This graph shows the folder structure generated with `all` features enabled. Each module becomes its own workspace member crate (here `io_world`), and the shared `examples` crate sits next to it at the workspace root (here `rust_hello_world`). For more details visit the documentation for each feature.

```bash
📂hello-world
 ┣ 📂apigear
 ┃ ┣ 📜helloworld.solution.yaml
 ┃ ┗ 📜helloworld.module.yaml
 ┣ 📂rust_hello_world
 ┃ ┣ 📜Cargo.toml          # workspace manifest
 ┃ ┣ 📜rustfmt.toml
 ┃ ┣ 📂examples            # runnable example programs (local + IPC client/server)
 ┃ ┗ 📂io_world            # one crate per module
 ┃ ┃ ┣ 📜Cargo.toml
 ┃ ┃ ┣ 📂src
 ┃ ┃ ┃ ┣ 📜lib.rs
 ┃ ┃ ┃ ┣ 📂api             # api feature: traits + Publisher + data structs/enums + ApiError/ApiFuture
 ┃ ┃ ┃ ┣ 📂core_types      # core feature: property bundle + shared reference + test helpers
 ┃ ┃ ┃ ┣ 📂implementation  # stubs feature: default implementation
 ┃ ┃ ┃ ┣ 📂monitor         # monitor feature: tracing decorator
 ┃ ┃ ┃ ┣ 📂olink           # olink feature: client + service adapters
 ┃ ┃ ┃ ┣ 📂mqtt            # mqtt feature: client + service adapters
 ┃ ┃ ┃ ┗ 📂nats            # nats feature: client + service adapters
 ┃ ┃ ┗ 📂tests             # unit tests + IPC round-trip tests
```
