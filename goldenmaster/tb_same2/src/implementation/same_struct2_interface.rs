use crate::api::same_struct2_interface::SameStruct2InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;
use crate::api::same_struct2_interface::SameStruct2InterfaceSignalHandler;
use signals2::*;

#[derive(Default, Clone)]
pub struct SameStruct2Interface {
    prop1: Struct2,
    prop2: Struct2,
    _signal_handler: SameStruct2InterfaceSignalHandler,
}

#[async_trait]
impl SameStruct2InterfaceTrait for SameStruct2Interface {
    fn func1(
        &mut self,
        _param1: &Struct1,
    ) -> Struct1 {
        Default::default()
    }
    /// Asynchronous version of [func1](SameStruct2Interface::func1)
    /// returns future of type [`Struct1`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: &Struct1,
    ) -> Result<Struct1, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func1(param1))
    }

    fn func2(
        &mut self,
        _param1: &Struct1,
        _param2: &Struct2,
    ) -> Struct1 {
        Default::default()
    }
    /// Asynchronous version of [func2](SameStruct2Interface::func2)
    /// returns future of type [`Struct1`] which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> Result<Struct1, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func2(param1, param2))
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
        self._signal_handler.prop1_changed.emit(self.prop1.clone());
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
        self._signal_handler.prop2_changed.emit(self.prop2.clone());
    }

    fn _get_signal_handler(&mut self) -> &SameStruct2InterfaceSignalHandler {
        &self._signal_handler
    }
}
