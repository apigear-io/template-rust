#![allow(unused_variables, clippy::get_first, clippy::let_unit_value, clippy::match_single_binding, clippy::unit_arg)]
{{ range .Module.Interfaces -}}
pub mod {{snake .Name}}_client;
pub mod {{snake .Name}}_service;
{{ end -}}
