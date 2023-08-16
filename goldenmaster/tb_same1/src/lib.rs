//pub mod crate::api::data_structs;
//pub mod crate::api::same_struct1_interface;
//pub mod crate::impl::same_struct1_interface;
//pub mod crate::api::same_struct2_interface;
//pub mod crate::impl::same_struct2_interface;
//pub mod crate::api::same_enum1_interface;
//pub mod crate::impl::same_enum1_interface;
//pub mod crate::api::same_enum2_interface;
//pub mod crate::impl::same_enum2_interface;

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
