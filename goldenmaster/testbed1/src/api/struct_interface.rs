// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::data_structs::*;
use std::future::Future;

pub trait StructInterface {
    fn func_bool(
        &mut self,
        param_bool: &StructBool,
    ) -> StructBool;
    /// Asynchronous version of `func_bool`
    /// returns future of type StructBool which is set once the function has completed
    fn func_bool_async(
        &mut self,
        param_bool: &StructBool,
    ) -> dyn Future<Output = StructBool>;

    fn func_int(
        &mut self,
        param_int: &StructInt,
    ) -> StructBool;
    /// Asynchronous version of `func_int`
    /// returns future of type StructBool which is set once the function has completed
    fn func_int_async(
        &mut self,
        param_int: &StructInt,
    ) -> dyn Future<Output = StructBool>;

    fn func_float(
        &mut self,
        param_float: &StructFloat,
    ) -> StructFloat;
    /// Asynchronous version of `func_float`
    /// returns future of type StructFloat which is set once the function has completed
    fn func_float_async(
        &mut self,
        param_float: &StructFloat,
    ) -> dyn Future<Output = StructFloat>;

    fn func_string(
        &mut self,
        param_string: &StructString,
    ) -> StructString;
    /// Asynchronous version of `func_string`
    /// returns future of type StructString which is set once the function has completed
    fn func_string_async(
        &mut self,
        param_string: &StructString,
    ) -> dyn Future<Output = StructString>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> &StructBool;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: &StructBool,
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> &StructInt;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: &StructInt,
    );

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> &StructFloat;
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &mut self,
        prop_float: &StructFloat,
    );

    /// Gets the value of the propString property.
    fn prop_string(&self) -> &StructString;
    /// Sets the value of the propString property.
    fn set_prop_string(
        &mut self,
        prop_string: &StructString,
    );
}
