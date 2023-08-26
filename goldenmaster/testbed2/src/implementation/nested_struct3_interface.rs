use crate::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct NestedStruct3Interface {
    prop1: NestedStruct1,
    prop2: NestedStruct2,
    prop3: NestedStruct3,
}

#[async_trait]
impl NestedStruct3InterfaceTrait for NestedStruct3Interface {
    fn func1(
        &mut self,
        _param1: &NestedStruct1,
    ) -> NestedStruct1 {
        Default::default()
    }
    /// Asynchronous version of [func1](NestedStruct3Interface::func1)
    /// returns future of type [`NestedStruct1`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: &NestedStruct1,
    ) -> Result<NestedStruct1, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func1(param1))
    }

    fn func2(
        &mut self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
    ) -> NestedStruct1 {
        Default::default()
    }
    /// Asynchronous version of [func2](NestedStruct3Interface::func2)
    /// returns future of type [`NestedStruct1`] which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
    ) -> Result<NestedStruct1, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func2(param1, param2))
    }

    fn func3(
        &mut self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
        _param3: &NestedStruct3,
    ) -> NestedStruct1 {
        Default::default()
    }
    /// Asynchronous version of [func3](NestedStruct3Interface::func3)
    /// returns future of type [`NestedStruct1`] which is set once the function has completed
    async fn func3_async(
        &mut self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
        param3: &NestedStruct3,
    ) -> Result<NestedStruct1, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func3(param1, param2, param3))
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
