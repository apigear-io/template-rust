use crate::api::same_enum1_interface::SameEnum1InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct SameEnum1Interface {
    prop1: Enum1Enum,
}

impl SameEnum1InterfaceTrait for SameEnum1Interface {
    fn func1(
        &mut self,
        _param1: Enum1Enum,
    ) -> Enum1Enum {
        Default::default()
    }
    /// Asynchronous version of `func1`
    /// returns future of type Enum1Enum which is set once the function has completed
    fn func1_async(
        &mut self,
        _param1: Enum1Enum,
    ) -> Pin<Box<dyn Future<Output = Result<Enum1Enum, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Enum1Enum {
        self.prop1
    }
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: Enum1Enum,
    ) {
        if self.prop1 == prop1 {
            return;
        }

        self.prop1 = prop1;
    }
}
