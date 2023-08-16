//pub mod crate::api::void_interface;
//pub mod crate::impl::void_interface;
//pub mod crate::api::simple_interface;
//pub mod crate::impl::simple_interface;
//pub mod crate::api::simple_array_interface;
//pub mod crate::impl::simple_array_interface;
//pub mod crate::api::no_properties_interface;
//pub mod crate::impl::no_properties_interface;
//pub mod crate::api::no_operations_interface;
//pub mod crate::impl::no_operations_interface;
//pub mod crate::api::no_signals_interface;
//pub mod crate::impl::no_signals_interface;

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
