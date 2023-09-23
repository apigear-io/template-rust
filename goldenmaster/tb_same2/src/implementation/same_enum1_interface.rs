use crate::api::same_enum1_interface::SameEnum1InterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;
use crate::api::same_enum1_interface::SameEnum1InterfaceSignalHandler;
use signals2::*;

#[derive(Default, Clone)]
pub struct SameEnum1Interface {
    prop1: Enum1Enum,
    _signal_handler: SameEnum1InterfaceSignalHandler,
}

#[async_trait]
impl SameEnum1InterfaceTrait for SameEnum1Interface {
    fn func1(
        &mut self,
        _param1: Enum1Enum,
    ) -> Enum1Enum {
        Default::default()
    }
    /// Asynchronous version of [func1](SameEnum1Interface::func1)
    /// returns future of type [`Enum1Enum`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: Enum1Enum,
    ) -> Result<Enum1Enum, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func1(param1))
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
        self._signal_handler.prop1_changed.emit(self.prop1);
    }

    fn _get_signal_handler(&mut self) -> &SameEnum1InterfaceSignalHandler {
        &self._signal_handler
    }
}
