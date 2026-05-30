#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct NestedStruct3InterfacePublisher {
    pub prop1_changed: watch::Sender<NestedStruct1>,
    pub prop2_changed: watch::Sender<NestedStruct2>,
    pub prop3_changed: watch::Sender<NestedStruct3>,
    pub sig1: broadcast::Sender<(NestedStruct1,)>,
    pub sig2: broadcast::Sender<(NestedStruct1, NestedStruct2)>,
    pub sig3: broadcast::Sender<(NestedStruct1, NestedStruct2, NestedStruct3)>,
}

impl Default for NestedStruct3InterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, prop2_changed: watch::channel(Default::default()).0, prop3_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16), sig2: broadcast::Sender::new(16), sig3: broadcast::Sender::new(16) }
    }
}

pub trait NestedStruct3InterfaceTrait: Send + Sync {
    fn func1(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>>;

    fn func2(
        &self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>>;

    fn func3(
        &self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
        param3: &NestedStruct3,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> NestedStruct1;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: &NestedStruct1,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> NestedStruct2;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &self,
        prop2: &NestedStruct2,
    );

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> NestedStruct3;
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &self,
        prop3: &NestedStruct3,
    );

    fn publisher(&self) -> &NestedStruct3InterfacePublisher;
}
