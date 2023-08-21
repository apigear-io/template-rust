use crate::api::nested_struct2_interface::NestedStruct2InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct NestedStruct2Interface {
    prop1: NestedStruct1,
    prop2: NestedStruct2,
}

#[async_trait]
impl NestedStruct2InterfaceTrait for NestedStruct2Interface {
    fn func1(
        &mut self,
        _param1: &NestedStruct1,
    ) -> NestedStruct1 {
        Default::default()
    }
    /// Asynchronous version of `func1`
    /// returns future of type NestedStruct1 which is set once the function has completed
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
    /// Asynchronous version of `func2`
    /// returns future of type NestedStruct1 which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
    ) -> Result<NestedStruct1, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func2(param1, param2))
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
}
