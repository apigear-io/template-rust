---
sidebar_label: "Monitor"
title: "Rust Template Monitoring - ApiGear"
description: "Traffic monitoring in the ApiGear Rust template: trace interface properties, methods, and signals from your Rust SDK in ApiGear Studio."
keywords: [rust, apigear, objectapi, monitoring, tracing, apigear studio]
sidebar_position: 4
---

import CodeBlock from '@theme/CodeBlock';
import helloWorldModuleComponent from '!!raw-loader!./data/helloworld.module.yaml';

# Monitor

Use the monitor feature to observe interface calls and state changes at runtime. It generates a [`tracing`](https://docs.rs/tracing) decorator that wraps any implementation of your interface and emits a tracing event for every operation and property change. Because the decorator implements the same interface trait, you can drop it in wherever your implementation is used.

The monitoring server is embedded into [ApiGear Studio](/docs/studio/intro) and the [CLI](/docs/cli/intro). For more details see the [monitoring documentation](/docs/monitor/intro).

:::note
This feature requires `api` and `core`.
:::

## File overview for module

With our example API definition

<details>
    <summary>Hello World API (click to expand)</summary>
    <CodeBlock language="yaml" showLineNumbers>{helloWorldModuleComponent}</CodeBlock>
</details>

the following files are generated. The purpose and content of each file is explained below.

```bash {7}
📂io_world
 ┣ 📂src
 ┃ ┣ 📂monitor
 ┃ ┃ ┣ 📜mod.rs
 ┃ ┃ ┗ 📜hello_traced.rs   # tracing decorator for Hello
 ┃ ┗ 📜lib.rs
 ...
```

## The tracing decorator

The file `📜hello_traced.rs` contains a `HelloTraced<T>` decorator. It is generic over any `T: HelloTrait`, wraps an inner implementation, and implements `HelloTrait` itself by logging a [`tracing`](https://docs.rs/tracing) event and then forwarding the call to the inner object.

```rust
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use crate::api::hello::HelloPublisher;
use crate::api::hello::HelloTrait;
use tracing;

/// Trace decorator for Hello.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct HelloTraced<T: HelloTrait> {
    inner: T,
}

impl<T: HelloTrait> HelloTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: HelloTrait> HelloTrait for HelloTraced<T> {
    fn say(
        &self,
        msg: &Message,
        when: WhenEnum,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        tracing::info!("Hello::say called");
        self.inner.say(msg, when)
    }

    fn last(&self) -> Message {
        self.inner.last() // getters are forwarded without tracing
    }
    fn set_last(
        &self,
        last: &Message,
    ) {
        tracing::info!("Hello::set_last called");
        self.inner.set_last(last);
    }

    fn publisher(&self) -> &HelloPublisher {
        self.inner.publisher()
    }
}
```

Operations and property setters are logged; getters and the `publisher()` accessor are forwarded unchanged. Since `HelloTraced<T>` is itself a `HelloTrait`, you can use it anywhere the plain implementation fits — including in front of an IPC client adapter.

:::note
The wrapped `inner` value just has to satisfy the `HelloTrait` bound, so it can be the default [stub](stubs.md) implementation, an [OLink client](olink.md), or any other implementation.
:::

## Use the decorator

Wrap your implementation with `HelloTraced::new(...)` and use the result as a `Hello` object. Every traced call produces a `tracing` event that a configured subscriber can forward to the ApiGear monitoring server.

```rust
use io_world::api::hello::HelloTrait;
use io_world::api::hello::HelloTraitAsync;
use io_world::implementation::hello::Hello;
use io_world::monitor::hello_traced::HelloTraced;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // Set up a tracing subscriber (here: log to stdout).
    tracing_subscriber::fmt::init();

    // Wrap the default implementation with the tracing decorator.
    let hello = HelloTraced::new(Hello::default());

    // Use it exactly like a Hello implementation — every call is traced.
    let _ = hello.say_async(&Default::default(), Default::default()).await;
    hello.set_last(&Default::default());
}
```

:::tip
The `tracing` crate decouples *emitting* events from *collecting* them. Install any [`tracing-subscriber`](https://docs.rs/tracing-subscriber) layer to control where the events go — the console for local development, or an exporter that forwards them to the [ApiGear Studio](/docs/studio/intro) or [CLI](/docs/cli/intro) monitor.
:::
