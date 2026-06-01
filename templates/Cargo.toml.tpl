[workspace]
resolver = "2"

members = [
{{- range .System.Modules }}
{{- $module_id := snake .Name}}
    "{{$module_id}}",
{{- end }}
    "examples",
]

[workspace.dependencies]
async-nats = "0.38"
futures = "0.3"
parking_lot = "0.12"
tokio = { version = "1", features = ["sync", "rt", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tracing = "0.1"
objectlink-core = { path = "../deps/objectlink-core-rs" }
rumqttc = "0.24"
