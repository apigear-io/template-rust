pub use crate::api::data_structs;
pub use crate::api::struct_interface;
pub use crate::api::struct_array_interface;

pub mod api;

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
