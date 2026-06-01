---
sidebar_position: 2
---
import QuickStartCommon from "@site/docs/_quickstart_common.md"

# Quick-Start

The Quick-Start guide explains how in a few steps you get from an API definition to a functional _Rust_ example.

Steps one and two are universal for other technologies. In step two you will choose a concrete _rust_ template.
For more general information about first steps with ApiGear see [First Steps](/docs/guide/quick-start).

The quick start enables only basic features: the [api](../features/api.md) generation and a ready-to-use [stub](../features/stubs.md) implementation.
For all available features check the [overview](../features/features.md).

<QuickStartCommon />

## 5. Use the generated Cargo workspace

The _rust_ template generates a [Cargo](https://doc.rust-lang.org/cargo/) workspace. Each API module becomes its own member crate, and a shared `examples` crate ties them together with runnable programs. To build and run the code you need a working [Rust toolchain](https://www.rust-lang.org/tools/install) (`rustc` and `cargo`, 1.80 or newer).

:::note
Basic Rust knowledge is necessary.
:::

### Project folder structure

With the output directory set as in the example, both _ApiGear_ files reside in an `apigear` subfolder next to the generated _Rust_ workspace.
In this case the folder structure should look similar to this:

```bash
📂hello-world
 ┣ 📂apigear
 ┃ ┣ 📜helloworld.solution.yaml
 ┃ ┗ 📜helloworld.module.yaml
 ┣ 📂rust_hello_world
 ┃ ┣ 📜Cargo.toml          # workspace manifest (lists every module crate)
 ┃ ┣ 📜rustfmt.toml
 # highlight-next-line
 ┃ ┣ 📂io_world            # one crate per module
 ┃ ┃ ┣ 📜Cargo.toml
 ┃ ┃ ┣ 📂src
 ┃ ┃ ┃ ┣ 📜lib.rs
 ┃ ┃ ┃ ┣ 📂api             # interface traits + Publisher + data structs/enums
 ┃ ┃ ┃ ┣ 📂core_types      # property bundle + shared reference
 ┃ ┃ ┃ ┗ 📂implementation  # ready-to-use default impl
 ┃ ┃ ┗ 📂tests             # per-interface unit tests
 ┃ ┗ 📂examples            # runnable example programs
```

Using the solution file from the previous paragraph the code is generated in the `rust_hello_world` folder, with a subfolder (member crate) for each module — here `io_world`, the name of the module defined in line 2 of `helloworld.module.yaml`. It contains both generated features: the basic [api](../features/api.md) and a [stub](../features/stubs.md) implementation.

The `io_world/src/api/` folder contains the trait definitions for your interfaces along with the `serde`-enabled enums and structs for your module, while `io_world/src/core_types/` holds the per-interface support types (a property bundle for state sync and a shared-reference helper). The `io_world/src/implementation/` folder holds a default implementation you can use as-is or build on.

:::tip
Check our `examples` crate with all features enabled to get more working examples — including IPC clients and servers for [OLink](../features/olink.md), [MQTT](../features/mqtt.md) and [NATS](../features/nats.md).
:::

:::note
For the simulation, check [the olink feature](../features/olink.md) which provides a middle layer on your code side, and the [simulation](/docs/scripting/backends/intro) explained.
:::

### Build the workspace

Open a terminal, navigate to the generated `rust_hello_world` folder and build everything with Cargo:

```bash
cargo build
```

`cargo` resolves the workspace from the top-level `Cargo.toml`, downloads the dependencies, and compiles every module crate together with the `examples` crate.

### Run an example

The generated `examples` crate ships a runnable program that exercises every interface in-process — calling operations, setting properties and listing available signals. Run it with:

```bash
cargo run -p rust_hello_world_examples
```

:::note
The example crate's package name is derived from your solution target, with an `_examples` suffix. If your target is named `rust_hello_world`, the package is `rust_hello_world_examples`. Check the `[package] name` in `examples/Cargo.toml` and run `cargo run -p <name>` accordingly.
:::

### Run the tests

Each interface comes with a generated unit test that instantiates the default implementation and exercises its operations and properties. Run them with:

```bash
cargo test
```

From now on you can simply add your module crate to your own `Cargo.toml` as a dependency and use the generated traits, data types and default implementation.
For more details on the generated features please check [api](../features/api.md) and [stubs](../features/stubs.md).
