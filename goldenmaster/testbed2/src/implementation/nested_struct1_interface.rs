use crate::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct NestedStruct1Interface {
    prop1: NestedStruct1,
}

#[async_trait]
impl NestedStruct1InterfaceTrait for NestedStruct1Interface {
    fn func1(
        &mut self,
        _param1: &NestedStruct1,
    ) -> NestedStruct1 {
        Default::default()
    }
    /// Asynchronous version of [func1](NestedStruct1Interface::func1)
    /// returns future of type [`NestedStruct1`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: &NestedStruct1,
    ) -> Result<NestedStruct1, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func1(param1))
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
}
