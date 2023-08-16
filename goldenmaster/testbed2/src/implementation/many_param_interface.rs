use crate::api::many_param_interface::ManyParamInterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct ManyParamInterface {
    prop1: i32,
    prop2: i32,
    prop3: i32,
    prop4: i32,
}

impl ManyParamInterfaceTrait for ManyParamInterface {
    fn func1(
        &mut self,
        _param1: i32,
    ) -> i32 {
        Default::default()
    }
    /// Asynchronous version of `func1`
    /// returns future of type i32 which is set once the function has completed
    fn func1_async(
        &mut self,
        _param1: i32,
    ) -> Pin<Box<dyn Future<Output = Result<i32, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func2(
        &mut self,
        _param1: i32,
        _param2: i32,
    ) -> i32 {
        Default::default()
    }
    /// Asynchronous version of `func2`
    /// returns future of type i32 which is set once the function has completed
    fn func2_async(
        &mut self,
        _param1: i32,
        _param2: i32,
    ) -> Pin<Box<dyn Future<Output = Result<i32, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func3(
        &mut self,
        _param1: i32,
        _param2: i32,
        _param3: i32,
    ) -> i32 {
        Default::default()
    }
    /// Asynchronous version of `func3`
    /// returns future of type i32 which is set once the function has completed
    fn func3_async(
        &mut self,
        _param1: i32,
        _param2: i32,
        _param3: i32,
    ) -> Pin<Box<dyn Future<Output = Result<i32, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func4(
        &mut self,
        _param1: i32,
        _param2: i32,
        _param3: i32,
        _param4: i32,
    ) -> i32 {
        Default::default()
    }
    /// Asynchronous version of `func4`
    /// returns future of type i32 which is set once the function has completed
    fn func4_async(
        &mut self,
        _param1: i32,
        _param2: i32,
        _param3: i32,
        _param4: i32,
    ) -> Pin<Box<dyn Future<Output = Result<i32, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> i32 {
        self.prop1
    }
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: i32,
    ) {
        if self.prop1 == prop1 {
            return;
        }

        self.prop1 = prop1;
    }

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> i32 {
        self.prop2
    }
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &mut self,
        prop2: i32,
    ) {
        if self.prop2 == prop2 {
            return;
        }

        self.prop2 = prop2;
    }

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> i32 {
        self.prop3
    }
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &mut self,
        prop3: i32,
    ) {
        if self.prop3 == prop3 {
            return;
        }

        self.prop3 = prop3;
    }

    /// Gets the value of the prop4 property.
    fn prop4(&self) -> i32 {
        self.prop4
    }
    /// Sets the value of the prop4 property.
    fn set_prop4(
        &mut self,
        prop4: i32,
    ) {
        if self.prop4 == prop4 {
            return;
        }

        self.prop4 = prop4;
    }
}
