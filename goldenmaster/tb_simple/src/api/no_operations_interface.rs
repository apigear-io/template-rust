pub trait NoOperationsInterface {
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
}
