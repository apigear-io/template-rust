#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct EnumInterfacePublisher {
    pub prop0_changed: watch::Sender<Enum0Enum>,
    pub prop1_changed: watch::Sender<Enum1Enum>,
    pub prop2_changed: watch::Sender<Enum2Enum>,
    pub prop3_changed: watch::Sender<Enum3Enum>,
    pub sig0: broadcast::Sender<(Enum0Enum,)>,
    pub sig1: broadcast::Sender<(Enum1Enum,)>,
    pub sig2: broadcast::Sender<(Enum2Enum,)>,
    pub sig3: broadcast::Sender<(Enum3Enum,)>,
}

impl Default for EnumInterfacePublisher {
    fn default() -> Self {
        Self { prop0_changed: watch::channel(Default::default()).0, prop1_changed: watch::channel(Default::default()).0, prop2_changed: watch::channel(Default::default()).0, prop3_changed: watch::channel(Default::default()).0, sig0: broadcast::Sender::new(16), sig1: broadcast::Sender::new(16), sig2: broadcast::Sender::new(16), sig3: broadcast::Sender::new(16) }
    }
}

pub trait EnumInterfaceTrait: Send + Sync {
    fn func0(
        &self,
        param0: Enum0Enum,
    ) -> ApiFuture<'_, Result<Enum0Enum, ApiError>>;

    fn func1(
        &self,
        param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>>;

    fn func2(
        &self,
        param2: Enum2Enum,
    ) -> ApiFuture<'_, Result<Enum2Enum, ApiError>>;

    fn func3(
        &self,
        param3: Enum3Enum,
    ) -> ApiFuture<'_, Result<Enum3Enum, ApiError>>;

    /// Gets the value of the prop0 property.
    fn prop0(&self) -> Enum0Enum;
    /// Sets the value of the prop0 property.
    fn set_prop0(
        &self,
        prop0: Enum0Enum,
    );

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Enum1Enum;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> Enum2Enum;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &self,
        prop2: Enum2Enum,
    );

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> Enum3Enum;
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &self,
        prop3: Enum3Enum,
    );

    fn publisher(&self) -> &EnumInterfacePublisher;
}
