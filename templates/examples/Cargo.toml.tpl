[package]
name = "{{ snake .System.Name }}_examples"
version = "0.1.0"
edition = "2021"
rust-version = "1.80"

[dependencies]
tokio = { version = "1", features = ["full"] }
serde_json = { workspace = true }
rumqttc = { workspace = true }
async-nats = { workspace = true }
futures = { workspace = true }
objectlink-core = { workspace = true }
{{- range .System.Modules }}
{{ snake .Name }} = { path = "../{{ snake .Name }}" }
{{- end }}
