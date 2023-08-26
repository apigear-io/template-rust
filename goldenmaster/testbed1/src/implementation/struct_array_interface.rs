use crate::api::struct_array_interface::StructArrayInterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct StructArrayInterface {
    prop_bool: Vec<StructBool>,
    prop_int: Vec<StructInt>,
    prop_float: Vec<StructFloat>,
    prop_string: Vec<StructString>,
}

#[async_trait]
impl StructArrayInterfaceTrait for StructArrayInterface {
    fn func_bool(
        &mut self,
        _param_bool: &[StructBool],
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of [func_bool](StructArrayInterface::func_bool)
    /// returns future of type [`StructBool`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: &[StructBool],
    ) -> Result<StructBool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_bool(param_bool))
    }

    fn func_int(
        &mut self,
        _param_int: &[StructInt],
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of [func_int](StructArrayInterface::func_int)
    /// returns future of type [`StructBool`] which is set once the function has completed
    async fn func_int_async(
        &mut self,
        param_int: &[StructInt],
    ) -> Result<StructBool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_int(param_int))
    }

    fn func_float(
        &mut self,
        _param_float: &[StructFloat],
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of [func_float](StructArrayInterface::func_float)
    /// returns future of type [`StructBool`] which is set once the function has completed
    async fn func_float_async(
        &mut self,
        param_float: &[StructFloat],
    ) -> Result<StructBool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_float(param_float))
    }

    fn func_string(
        &mut self,
        _param_string: &[StructString],
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of [func_string](StructArrayInterface::func_string)
    /// returns future of type [`StructBool`] which is set once the function has completed
    async fn func_string_async(
        &mut self,
        param_string: &[StructString],
    ) -> Result<StructBool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_string(param_string))
    }

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> &Vec<StructBool> {
        &self.prop_bool
    }
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: &[StructBool],
    ) {
        if self.prop_bool == prop_bool.to_vec() {
            return;
        }

        self.prop_bool = prop_bool.to_vec();
    }

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> &Vec<StructInt> {
        &self.prop_int
    }
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: &[StructInt],
    ) {
        if self.prop_int == prop_int.to_vec() {
            return;
        }

        self.prop_int = prop_int.to_vec();
    }

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> &Vec<StructFloat> {
        &self.prop_float
    }
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &mut self,
        prop_float: &[StructFloat],
    ) {
        if self.prop_float == prop_float.to_vec() {
            return;
        }

        self.prop_float = prop_float.to_vec();
    }

    /// Gets the value of the propString property.
    fn prop_string(&self) -> &Vec<StructString> {
        &self.prop_string
    }
    /// Sets the value of the propString property.
    fn set_prop_string(
        &mut self,
        prop_string: &[StructString],
    ) {
        if self.prop_string == prop_string.to_vec() {
            return;
        }

        self.prop_string = prop_string.to_vec();
    }
}
