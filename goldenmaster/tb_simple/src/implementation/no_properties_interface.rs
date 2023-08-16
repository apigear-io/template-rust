use crate::api::no_properties_interface::NoPropertiesInterfaceTrait;
use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct NoPropertiesInterface {}

impl NoPropertiesInterfaceTrait for NoPropertiesInterface {
    fn func_void(&mut self) {
        Default::default()
    }
    /// Asynchronous version of `func_void`
    /// returns future of type () which is set once the function has completed
    fn func_void_async(&mut self) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func_bool(
        &mut self,
        _param_bool: bool,
    ) -> bool {
        Default::default()
    }
    /// Asynchronous version of `func_bool`
    /// returns future of type bool which is set once the function has completed
    fn func_bool_async(
        &mut self,
        _param_bool: bool,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }
}
