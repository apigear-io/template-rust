use crate::api::same_enum2_interface::SameEnum2InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct SameEnum2Interface {
    prop1: Enum1Enum,
    prop2: Enum2Enum,
}

#[async_trait]
impl SameEnum2InterfaceTrait for SameEnum2Interface {
    fn func1(
        &mut self,
        _param1: Enum1Enum,
    ) -> Enum1Enum {
        Default::default()
    }
    /// Asynchronous version of [func1](SameEnum2Interface::func1)
    /// returns future of type [`Enum1Enum`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: Enum1Enum,
    ) -> Result<Enum1Enum, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func1(param1))
    }

    fn func2(
        &mut self,
        _param1: Enum1Enum,
        _param2: Enum2Enum,
    ) -> Enum1Enum {
        Default::default()
    }
    /// Asynchronous version of [func2](SameEnum2Interface::func2)
    /// returns future of type [`Enum1Enum`] which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: Enum1Enum,
        param2: Enum2Enum,
    ) -> Result<Enum1Enum, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func2(param1, param2))
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

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> Enum2Enum {
        self.prop2
    }
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &mut self,
        prop2: Enum2Enum,
    ) {
        if self.prop2 == prop2 {
            return;
        }

        self.prop2 = prop2;
    }
}
