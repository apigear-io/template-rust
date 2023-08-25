[package]
name = "{{ snake .System.Name }}_examples"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
{{- range .System.Modules }}
{{ snake .Name }} = { path = "../{{ snake .Name }}" }
{{- end }}
