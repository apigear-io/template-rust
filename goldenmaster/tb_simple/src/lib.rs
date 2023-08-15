pub use crate::api::void_interface;
pub use crate::api::simple_interface;
pub use crate::api::simple_array_interface;
pub use crate::api::no_properties_interface;
pub use crate::api::no_operations_interface;
pub use crate::api::no_signals_interface;

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
