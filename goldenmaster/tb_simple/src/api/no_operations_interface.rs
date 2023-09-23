use signals2::*;

#[derive(Clone, Default)]
pub struct NoOperationsInterfaceSignalHandler {
    pub prop_bool_changed: Signal<(bool,)>,

    pub prop_int_changed: Signal<(i32,)>,

    pub sig_void: Signal<()>,

    pub sig_bool: Signal<(bool,)>,
}
pub trait NoOperationsInterfaceTrait {
    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> bool;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: bool,
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> i32;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: i32,
    );

    fn _get_signal_handler(&mut self) -> &NoOperationsInterfaceSignalHandler;
}
