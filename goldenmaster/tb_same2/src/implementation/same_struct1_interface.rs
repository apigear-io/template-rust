use crate::api::same_struct1_interface::SameStruct1InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct SameStruct1Interface {
    prop1: Struct1,
}

impl SameStruct1InterfaceTrait for SameStruct1Interface {
    fn func1(
        &mut self,
        _param1: &Struct1,
    ) -> Struct1 {
        Default::default()
    }
    /// Asynchronous version of `func1`
    /// returns future of type Struct1 which is set once the function has completed
    fn func1_async(
        &mut self,
        _param1: &Struct1,
    ) -> Pin<Box<dyn Future<Output = Result<Struct1, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> &Struct1 {
        &self.prop1
    }
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: &Struct1,
    ) {
        if self.prop1 == prop1.clone() {
            return;
        }

        self.prop1 = prop1.clone();
    }
}
