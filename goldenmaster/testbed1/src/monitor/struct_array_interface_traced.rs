#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_array_interface::StructArrayInterfacePublisher;
use crate::api::struct_array_interface::StructArrayInterfaceTrait;
use tracing;

/// Trace decorator for StructArrayInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct StructArrayInterfaceTraced<T: StructArrayInterfaceTrait> {
    inner: T,
}

impl<T: StructArrayInterfaceTrait> StructArrayInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: StructArrayInterfaceTrait> StructArrayInterfaceTrait for StructArrayInterfaceTraced<T> {
    fn func_bool(
        &self,
        param_bool: &[StructBool],
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>> {
        tracing::info!("StructArrayInterface::func_bool called");
        self.inner.func_bool(param_bool)
    }

    fn func_int(
        &self,
        param_int: &[StructInt],
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>> {
        tracing::info!("StructArrayInterface::func_int called");
        self.inner.func_int(param_int)
    }

    fn func_float(
        &self,
        param_float: &[StructFloat],
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>> {
        tracing::info!("StructArrayInterface::func_float called");
        self.inner.func_float(param_float)
    }

    fn func_string(
        &self,
        param_string: &[StructString],
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>> {
        tracing::info!("StructArrayInterface::func_string called");
        self.inner.func_string(param_string)
    }

    fn func_enum(
        &self,
        param_enum: &[Enum0Enum],
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>> {
        tracing::info!("StructArrayInterface::func_enum called");
        self.inner.func_enum(param_enum)
    }

    fn prop_bool(&self) -> Vec<StructBool> {
        self.inner.prop_bool()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &[StructBool],
    ) {
        tracing::info!("StructArrayInterface::set_prop_bool called");
        self.inner.set_prop_bool(prop_bool);
    }

    fn prop_int(&self) -> Vec<StructInt> {
        self.inner.prop_int()
    }
    fn set_prop_int(
        &self,
        prop_int: &[StructInt],
    ) {
        tracing::info!("StructArrayInterface::set_prop_int called");
        self.inner.set_prop_int(prop_int);
    }

    fn prop_float(&self) -> Vec<StructFloat> {
        self.inner.prop_float()
    }
    fn set_prop_float(
        &self,
        prop_float: &[StructFloat],
    ) {
        tracing::info!("StructArrayInterface::set_prop_float called");
        self.inner.set_prop_float(prop_float);
    }

    fn prop_string(&self) -> Vec<StructString> {
        self.inner.prop_string()
    }
    fn set_prop_string(
        &self,
        prop_string: &[StructString],
    ) {
        tracing::info!("StructArrayInterface::set_prop_string called");
        self.inner.set_prop_string(prop_string);
    }

    fn prop_enum(&self) -> Vec<Enum0Enum> {
        self.inner.prop_enum()
    }
    fn set_prop_enum(
        &self,
        prop_enum: &[Enum0Enum],
    ) {
        tracing::info!("StructArrayInterface::set_prop_enum called");
        self.inner.set_prop_enum(prop_enum);
    }

    fn publisher(&self) -> &StructArrayInterfacePublisher {
        self.inner.publisher()
    }
}
