use crate::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct NestedStruct3Interface {
    prop1: NestedStruct1,
    prop2: NestedStruct2,
    prop3: NestedStruct3,
}

impl NestedStruct3InterfaceTrait for NestedStruct3Interface {
    fn func1(
        &mut self,
        _param1: &NestedStruct1,
    ) -> NestedStruct1 {
        Default::default()
    }
    /// Asynchronous version of `func1`
    /// returns future of type NestedStruct1 which is set once the function has completed
    fn func1_async(
        &mut self,
        _param1: &NestedStruct1,
    ) -> Pin<Box<dyn Future<Output = Result<NestedStruct1, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func2(
        &mut self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
    ) -> NestedStruct1 {
        Default::default()
    }
    /// Asynchronous version of `func2`
    /// returns future of type NestedStruct1 which is set once the function has completed
    fn func2_async(
        &mut self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
    ) -> Pin<Box<dyn Future<Output = Result<NestedStruct1, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func3(
        &mut self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
        _param3: &NestedStruct3,
    ) -> NestedStruct1 {
        Default::default()
    }
    /// Asynchronous version of `func3`
    /// returns future of type NestedStruct1 which is set once the function has completed
    fn func3_async(
        &mut self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
        _param3: &NestedStruct3,
    ) -> Pin<Box<dyn Future<Output = Result<NestedStruct1, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> &NestedStruct1 {
        &self.prop1
    }
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: &NestedStruct1,
    ) {
        if self.prop1 == prop1.clone() {
            return;
        }

        self.prop1 = prop1.clone();
    }

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> &NestedStruct2 {
        &self.prop2
    }
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &mut self,
        prop2: &NestedStruct2,
    ) {
        if self.prop2 == prop2.clone() {
            return;
        }

        self.prop2 = prop2.clone();
    }

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> &NestedStruct3 {
        &self.prop3
    }
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &mut self,
        prop3: &NestedStruct3,
    ) {
        if self.prop3 == prop3.clone() {
            return;
        }

        self.prop3 = prop3.clone();
    }
}
