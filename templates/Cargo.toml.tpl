[workspace]
resolver = "2"

members = [
    "apigear",
{{- range .System.Modules }}
{{- $module_id := snake .Name}}
    "{{$module_id}}",
{{- end }}
    "examples",
]

[workspace.dependencies]
apigear = { path = "apigear" }
async-nats = "0.38"
futures = "0.3"
parking_lot = "0.12"
tokio = { version = "1", features = ["sync", "rt", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
objectlink-core = { path = "../deps/objectlink-core-rs" }
rumqttc = "0.24"
