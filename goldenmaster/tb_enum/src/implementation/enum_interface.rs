use crate::api::enum_interface::EnumInterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct EnumInterface {
    prop0: Enum0Enum,
    prop1: Enum1Enum,
    prop2: Enum2Enum,
    prop3: Enum3Enum,
}

#[async_trait]
impl EnumInterfaceTrait for EnumInterface {
    fn func0(
        &mut self,
        _param0: Enum0Enum,
    ) -> Enum0Enum {
        Default::default()
    }
    /// Asynchronous version of `func0`
    /// returns future of type Enum0Enum which is set once the function has completed
    async fn func0_async(
        &mut self,
        param0: Enum0Enum,
    ) -> Result<Enum0Enum, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func0(param0))
    }

    fn func1(
        &mut self,
        _param1: Enum1Enum,
    ) -> Enum1Enum {
        Default::default()
    }
    /// Asynchronous version of `func1`
    /// returns future of type Enum1Enum which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: Enum1Enum,
    ) -> Result<Enum1Enum, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func1(param1))
    }

    fn func2(
        &mut self,
        _param2: Enum2Enum,
    ) -> Enum2Enum {
        Default::default()
    }
    /// Asynchronous version of `func2`
    /// returns future of type Enum2Enum which is set once the function has completed
    async fn func2_async(
        &mut self,
        param2: Enum2Enum,
    ) -> Result<Enum2Enum, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func2(param2))
    }

    fn func3(
        &mut self,
        _param3: Enum3Enum,
    ) -> Enum3Enum {
        Default::default()
    }
    /// Asynchronous version of `func3`
    /// returns future of type Enum3Enum which is set once the function has completed
    async fn func3_async(
        &mut self,
        param3: Enum3Enum,
    ) -> Result<Enum3Enum, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func3(param3))
    }

    /// Gets the value of the prop0 property.
    fn prop0(&self) -> Enum0Enum {
        self.prop0
    }
    /// Sets the value of the prop0 property.
    fn set_prop0(
        &mut self,
        prop0: Enum0Enum,
    ) {
        if self.prop0 == prop0 {
            return;
        }

        self.prop0 = prop0;
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

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> Enum3Enum {
        self.prop3
    }
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &mut self,
        prop3: Enum3Enum,
    ) {
        if self.prop3 == prop3 {
            return;
        }

        self.prop3 = prop3;
    }
}
