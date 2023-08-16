{{- if or .Module.Structs .Module.Enums -}}
//pub mod crate::api::data_structs;{{ nl }}
{{- end }}
{{- range .Module.Interfaces -}}
//pub mod crate::api::{{snake .Name}};
//pub mod crate::impl::{{snake .Name}};{{ nl }}
{{- end}}
pub mod api;
pub mod implementation;

pub fn add(
    left: usize,
    right: usize,
) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
