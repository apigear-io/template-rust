use crate::api::no_signals_interface::NoSignalsInterfaceTrait;
use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct NoSignalsInterface {
    prop_bool: bool,
    prop_int: i32,
}

impl NoSignalsInterfaceTrait for NoSignalsInterface {
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

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> bool {
        self.prop_bool
    }
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: bool,
    ) {
        if self.prop_bool == prop_bool {
            return;
        }

        self.prop_bool = prop_bool;
    }

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> i32 {
        self.prop_int
    }
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: i32,
    ) {
        if self.prop_int == prop_int {
            return;
        }

        self.prop_int = prop_int;
    }
}
