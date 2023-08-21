use crate::api::struct_interface::StructInterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct StructInterface {
    prop_bool: StructBool,
    prop_int: StructInt,
    prop_float: StructFloat,
    prop_string: StructString,
}

#[async_trait]
impl StructInterfaceTrait for StructInterface {
    fn func_bool(
        &mut self,
        _param_bool: &StructBool,
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of `func_bool`
    /// returns future of type StructBool which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: &StructBool,
    ) -> Result<StructBool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_bool(param_bool))
    }

    fn func_int(
        &mut self,
        _param_int: &StructInt,
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of `func_int`
    /// returns future of type StructBool which is set once the function has completed
    async fn func_int_async(
        &mut self,
        param_int: &StructInt,
    ) -> Result<StructBool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_int(param_int))
    }

    fn func_float(
        &mut self,
        _param_float: &StructFloat,
    ) -> StructFloat {
        Default::default()
    }
    /// Asynchronous version of `func_float`
    /// returns future of type StructFloat which is set once the function has completed
    async fn func_float_async(
        &mut self,
        param_float: &StructFloat,
    ) -> Result<StructFloat, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_float(param_float))
    }

    fn func_string(
        &mut self,
        _param_string: &StructString,
    ) -> StructString {
        Default::default()
    }
    /// Asynchronous version of `func_string`
    /// returns future of type StructString which is set once the function has completed
    async fn func_string_async(
        &mut self,
        param_string: &StructString,
    ) -> Result<StructString, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_string(param_string))
    }

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> &StructBool {
        &self.prop_bool
    }
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: &StructBool,
    ) {
        if self.prop_bool == prop_bool.clone() {
            return;
        }

        self.prop_bool = prop_bool.clone();
    }

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> &StructInt {
        &self.prop_int
    }
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: &StructInt,
    ) {
        if self.prop_int == prop_int.clone() {
            return;
        }

        self.prop_int = prop_int.clone();
    }

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> &StructFloat {
        &self.prop_float
    }
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &mut self,
        prop_float: &StructFloat,
    ) {
        if self.prop_float == prop_float.clone() {
            return;
        }

        self.prop_float = prop_float.clone();
    }

    /// Gets the value of the propString property.
    fn prop_string(&self) -> &StructString {
        &self.prop_string
    }
    /// Sets the value of the propString property.
    fn set_prop_string(
        &mut self,
        prop_string: &StructString,
    ) {
        if self.prop_string == prop_string.clone() {
            return;
        }

        self.prop_string = prop_string.clone();
    }
}
