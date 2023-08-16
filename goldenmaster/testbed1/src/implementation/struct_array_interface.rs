use crate::api::struct_array_interface::StructArrayInterfaceTrait;
// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;

use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct StructArrayInterface {
    prop_bool: Vec<StructBool>,
    prop_int: Vec<StructInt>,
    prop_float: Vec<StructFloat>,
    prop_string: Vec<StructString>,
}

impl StructArrayInterfaceTrait for StructArrayInterface {
    fn func_bool(
        &mut self,
        _param_bool: &[StructBool],
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of `func_bool`
    /// returns future of type StructBool which is set once the function has completed
    fn func_bool_async(
        &mut self,
        _param_bool: &[StructBool],
    ) -> Pin<Box<dyn Future<Output = Result<StructBool, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func_int(
        &mut self,
        _param_int: &[StructInt],
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of `func_int`
    /// returns future of type StructBool which is set once the function has completed
    fn func_int_async(
        &mut self,
        _param_int: &[StructInt],
    ) -> Pin<Box<dyn Future<Output = Result<StructBool, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func_float(
        &mut self,
        _param_float: &[StructFloat],
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of `func_float`
    /// returns future of type StructBool which is set once the function has completed
    fn func_float_async(
        &mut self,
        _param_float: &[StructFloat],
    ) -> Pin<Box<dyn Future<Output = Result<StructBool, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }

    fn func_string(
        &mut self,
        _param_string: &[StructString],
    ) -> StructBool {
        Default::default()
    }
    /// Asynchronous version of `func_string`
    /// returns future of type StructBool which is set once the function has completed
    fn func_string_async(
        &mut self,
        _param_string: &[StructString],
    ) -> Pin<Box<dyn Future<Output = Result<StructBool, ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
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
