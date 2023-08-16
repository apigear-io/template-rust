//pub mod crate::api::data_structs;
//pub mod crate::api::struct_interface;
//pub mod crate::impl::struct_interface;
//pub mod crate::api::struct_array_interface;
//pub mod crate::impl::struct_array_interface;

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
