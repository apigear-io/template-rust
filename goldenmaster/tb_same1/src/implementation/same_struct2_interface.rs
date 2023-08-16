use crate::api::same_struct2_interface::SameStruct2InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct SameStruct2Interface {
    prop1: Struct2,
    prop2: Struct2,
}

impl SameStruct2InterfaceTrait for SameStruct2Interface {
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

    fn func2(
        &mut self,
        _param1: &Struct1,
        _param2: &Struct2,
    ) -> Struct1 {
        Default::default()
    }
    /// Asynchronous version of `func2`
    /// returns future of type Struct1 which is set once the function has completed
    fn func2_async(
        &mut self,
        _param1: &Struct1,
        _param2: &Struct2,
    ) -> Pin<Box<dyn Future<Output = Result<Struct1, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> &Struct2 {
        &self.prop1
    }
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: &Struct2,
    ) {
        if self.prop1 == prop1.clone() {
            return;
        }

        self.prop1 = prop1.clone();
    }

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> &Struct2 {
        &self.prop2
    }
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &mut self,
        prop2: &Struct2,
    ) {
        if self.prop2 == prop2.clone() {
            return;
        }

        self.prop2 = prop2.clone();
    }
}
