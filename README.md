# Rust Blueprint

[![Build & Test](https://github.com/apigear-io/template-rust/actions/workflows/ci_build_test.yml/badge.svg)](https://github.com/apigear-io/template-rust/actions/workflows/ci_build_test.yml)
[![IPC Integration](https://github.com/apigear-io/template-rust/actions/workflows/ci_ipc_integration.yml/badge.svg)](https://github.com/apigear-io/template-rust/actions/workflows/ci_ipc_integration.yml)
[![Goldenmaster](https://github.com/apigear-io/template-rust/actions/workflows/ci_generate.yml/badge.svg)](https://github.com/apigear-io/template-rust/actions/workflows/ci_generate.yml)

A Rust blueprint template for [ApiGear](https://apigear.io/) that generates a complete
[Cargo](https://doc.rust-lang.org/cargo/) workspace from API definitions (YAML). It produces
trait-based interfaces with async operations, a [tokio](https://tokio.rs/)-channel publisher
for properties and signals, [serde](https://serde.rs/) data types, default implementations, a
tracing decorator, and IPC client/service adapters for ObjectLink, MQTT, and NATS.

## Features

Features are individually enabled in the solution file for code generation. Each input module
becomes a separate workspace member crate. The `api`/`core` crates are self-contained — only
the IPC and monitor features pull in transport dependencies.

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| **api** | Interface traits — awaitable `fn op() -> ApiFuture<…>` plus an ergonomic `async fn op_async()` companion; a `Publisher` (tokio `watch`/`broadcast`) for properties and signals; `ApiError`/`ApiFuture` | - |
| **core** | Per-interface data structs and shared serde types | api |
| **stubs** | Workspace, default trait implementations, examples, per-interface unit tests | api, core |
| **monitor** | [`tracing`](https://docs.rs/tracing) decorator wrapping any implementation | api, core |
| **olink** | [ObjectLink](https://objectlinkprotocol.net/) IPC adapters + in-process loopback tests | api, core |
| **mqtt** | [MQTT](https://mqtt.org/) IPC adapters via [`rumqttc`](https://docs.rs/rumqttc) + broker integration tests | api, core |
| **nats** | [NATS](https://nats.io/) IPC adapters via [`async-nats`](https://docs.rs/async-nats) + server integration tests | api, core |

## Building

```bash
cargo build   --manifest-path goldenmaster/Cargo.toml
cargo test    --manifest-path goldenmaster/Cargo.toml
cargo clippy  --manifest-path goldenmaster/Cargo.toml --all-targets
cargo fmt     --manifest-path goldenmaster/Cargo.toml --all -- --check
cargo doc     --manifest-path goldenmaster/Cargo.toml --no-deps
```

## Testing

| Tests | How they run |
|-------|--------------|
| Implementation unit tests | part of `cargo test` (in-process) |
| OLink round-trip tests | part of `cargo test` (in-process loopback, no broker) |
| MQTT / NATS round-trip tests | marked `#[ignore]`; require a running broker / server |

The MQTT and NATS tests exercise a real client↔service round-trip over a live broker, so they
are `#[ignore]`d by default. Run them against a broker/server the way CI does:

```bash
# MQTT — defaults to 127.0.0.1:1883 (override with MQTT_PORT)
mosquitto -p 1883 &
# NATS — defaults to 127.0.0.1:4222 (override with NATS_URL)
nats-server -p 4222 &

cargo test --manifest-path goldenmaster/Cargo.toml -- --ignored
```

## CI

| Workflow | What it checks | Platform |
|----------|---------------|----------|
| **Build & Test** | clippy (correctness+perf), rustfmt, `cargo test`, `cargo doc` on the goldenmaster | Ubuntu |
| **IPC Integration** | MQTT + NATS round-trip tests against a real Mosquitto broker and nats-server | Ubuntu |
| **Goldenmaster** | generated code matches the templates (`go run main.go diff`) | Ubuntu |

All workflows check out submodules recursively ([test-apis](https://github.com/apigear-io/test-apis)
for input modules, [objectlink-core-rs](https://github.com/apigear-io/objectlink-core-rs) for OLink).

## Template Development

This repository is a template project: `templates/` contains Go template files (`.tpl` and
copied-verbatim sources), and `goldenmaster/` is the reference output that must always match
what the generator produces.

```bash
go run main.go install   # Download the apigear CLI to bin/
go run main.go gentest   # Generate code into test/
go run main.go diff      # Generate + diff test/ vs goldenmaster/
go run main.go master    # Regenerate the goldenmaster from the solution
```

Workflow for template changes:

1. Edit templates in `templates/`
2. Run `go run main.go master` to regenerate `goldenmaster/`
3. Verify the generated code builds and passes tests
4. CI validates the goldenmaster is up-to-date on every PR

## License

Licensed under the [MIT License](./LICENSE). See [LICENSE](./LICENSE) for details.
