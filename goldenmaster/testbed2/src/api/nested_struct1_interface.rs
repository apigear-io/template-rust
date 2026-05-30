#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct NestedStruct1InterfacePublisher {
    pub prop1_changed: watch::Sender<NestedStruct1>,
    pub sig1: broadcast::Sender<(NestedStruct1,)>,
}

impl Default for NestedStruct1InterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16) }
    }
}

pub trait NestedStruct1InterfaceTrait: Send + Sync {
    fn func_no_return_value(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<(), ApiError>>;

    fn func_no_params(&self) -> ApiFuture<'_, Result<NestedStruct1, ApiError>>;

    fn func1(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> NestedStruct1;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: &NestedStruct1,
    );

    fn publisher(&self) -> &NestedStruct1InterfacePublisher;
}
