#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_interface::StructInterfacePublisher;
use crate::api::struct_interface::StructInterfaceTrait;
use tracing;

/// Trace decorator for StructInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct StructInterfaceTraced<T: StructInterfaceTrait> {
    inner: T,
}

impl<T: StructInterfaceTrait> StructInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: StructInterfaceTrait> StructInterfaceTrait for StructInterfaceTraced<T> {
    fn func_bool(
        &self,
        param_bool: &StructBool,
    ) -> ApiFuture<'_, Result<StructBool, ApiError>> {
        tracing::info!("StructInterface::func_bool called");
        self.inner.func_bool(param_bool)
    }

    fn func_int(
        &self,
        param_int: &StructInt,
    ) -> ApiFuture<'_, Result<StructInt, ApiError>> {
        tracing::info!("StructInterface::func_int called");
        self.inner.func_int(param_int)
    }

    fn func_float(
        &self,
        param_float: &StructFloat,
    ) -> ApiFuture<'_, Result<StructFloat, ApiError>> {
        tracing::info!("StructInterface::func_float called");
        self.inner.func_float(param_float)
    }

    fn func_string(
        &self,
        param_string: &StructString,
    ) -> ApiFuture<'_, Result<StructString, ApiError>> {
        tracing::info!("StructInterface::func_string called");
        self.inner.func_string(param_string)
    }

    fn prop_bool(&self) -> StructBool {
        self.inner.prop_bool()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &StructBool,
    ) {
        tracing::info!("StructInterface::set_prop_bool called");
        self.inner.set_prop_bool(prop_bool);
    }

    fn prop_int(&self) -> StructInt {
        self.inner.prop_int()
    }
    fn set_prop_int(
        &self,
        prop_int: &StructInt,
    ) {
        tracing::info!("StructInterface::set_prop_int called");
        self.inner.set_prop_int(prop_int);
    }

    fn prop_float(&self) -> StructFloat {
        self.inner.prop_float()
    }
    fn set_prop_float(
        &self,
        prop_float: &StructFloat,
    ) {
        tracing::info!("StructInterface::set_prop_float called");
        self.inner.set_prop_float(prop_float);
    }

    fn prop_string(&self) -> StructString {
        self.inner.prop_string()
    }
    fn set_prop_string(
        &self,
        prop_string: &StructString,
    ) {
        tracing::info!("StructInterface::set_prop_string called");
        self.inner.set_prop_string(prop_string);
    }

    fn publisher(&self) -> &StructInterfacePublisher {
        self.inner.publisher()
    }
}
