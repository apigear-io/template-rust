use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use crate::api::no_operations_interface::NoOperationsInterfaceSignalHandler;
use signals2::*;

#[derive(Default, Clone)]
pub struct NoOperationsInterface {
    prop_bool: bool,
    prop_int: i32,
    _signal_handler: NoOperationsInterfaceSignalHandler,
}

impl NoOperationsInterfaceTrait for NoOperationsInterface {
    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> bool {
        self.prop_bool
    }
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: bool,
    ) {
        if self.prop_bool == prop_bool {
            return;
        }

        self.prop_bool = prop_bool;
        self._signal_handler.prop_bool_changed.emit(self.prop_bool);
    }

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> i32 {
        self.prop_int
    }
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: i32,
    ) {
        if self.prop_int == prop_int {
            return;
        }

        self.prop_int = prop_int;
        self._signal_handler.prop_int_changed.emit(self.prop_int);
    }

    fn _get_signal_handler(&mut self) -> &NoOperationsInterfaceSignalHandler {
        &self._signal_handler
    }
}
