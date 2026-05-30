#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_array2_interface::StructArray2InterfacePublisher;
use crate::api::struct_array2_interface::StructArray2InterfaceTrait;
use tracing;

/// Trace decorator for StructArray2Interface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct StructArray2InterfaceTraced<T: StructArray2InterfaceTrait> {
    inner: T,
}

impl<T: StructArray2InterfaceTrait> StructArray2InterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: StructArray2InterfaceTrait> StructArray2InterfaceTrait for StructArray2InterfaceTraced<T> {
    fn func_bool(
        &self,
        param_bool: &StructBoolWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>> {
        tracing::info!("StructArray2Interface::func_bool called");
        self.inner.func_bool(param_bool)
    }

    fn func_int(
        &self,
        param_int: &StructIntWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>> {
        tracing::info!("StructArray2Interface::func_int called");
        self.inner.func_int(param_int)
    }

    fn func_float(
        &self,
        param_float: &StructFloatWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>> {
        tracing::info!("StructArray2Interface::func_float called");
        self.inner.func_float(param_float)
    }

    fn func_string(
        &self,
        param_string: &StructStringWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>> {
        tracing::info!("StructArray2Interface::func_string called");
        self.inner.func_string(param_string)
    }

    fn func_enum(
        &self,
        param_enum: &StructEnumWithArray,
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>> {
        tracing::info!("StructArray2Interface::func_enum called");
        self.inner.func_enum(param_enum)
    }

    fn prop_bool(&self) -> StructBoolWithArray {
        self.inner.prop_bool()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &StructBoolWithArray,
    ) {
        tracing::info!("StructArray2Interface::set_prop_bool called");
        self.inner.set_prop_bool(prop_bool);
    }

    fn prop_int(&self) -> StructIntWithArray {
        self.inner.prop_int()
    }
    fn set_prop_int(
        &self,
        prop_int: &StructIntWithArray,
    ) {
        tracing::info!("StructArray2Interface::set_prop_int called");
        self.inner.set_prop_int(prop_int);
    }

    fn prop_float(&self) -> StructFloatWithArray {
        self.inner.prop_float()
    }
    fn set_prop_float(
        &self,
        prop_float: &StructFloatWithArray,
    ) {
        tracing::info!("StructArray2Interface::set_prop_float called");
        self.inner.set_prop_float(prop_float);
    }

    fn prop_string(&self) -> StructStringWithArray {
        self.inner.prop_string()
    }
    fn set_prop_string(
        &self,
        prop_string: &StructStringWithArray,
    ) {
        tracing::info!("StructArray2Interface::set_prop_string called");
        self.inner.set_prop_string(prop_string);
    }

    fn prop_enum(&self) -> StructEnumWithArray {
        self.inner.prop_enum()
    }
    fn set_prop_enum(
        &self,
        prop_enum: &StructEnumWithArray,
    ) {
        tracing::info!("StructArray2Interface::set_prop_enum called");
        self.inner.set_prop_enum(prop_enum);
    }

    fn publisher(&self) -> &StructArray2InterfacePublisher {
        self.inner.publisher()
    }
}
