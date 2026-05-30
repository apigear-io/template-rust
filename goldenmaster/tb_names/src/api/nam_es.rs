#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct NamEsPublisher {
    pub switch_changed: watch::Sender<bool>,
    pub some_property_changed: watch::Sender<i32>,
    pub some_poperty2_changed: watch::Sender<i32>,
    pub enum_property_changed: watch::Sender<Enum_With_Under_scoresEnum>,
    pub some_signal: broadcast::Sender<(bool,)>,
    pub some_signal2: broadcast::Sender<(bool,)>,
}

impl Default for NamEsPublisher {
    fn default() -> Self {
        Self { switch_changed: watch::channel(Default::default()).0, some_property_changed: watch::channel(Default::default()).0, some_poperty2_changed: watch::channel(Default::default()).0, enum_property_changed: watch::channel(Default::default()).0, some_signal: broadcast::Sender::new(16), some_signal2: broadcast::Sender::new(16) }
    }
}

pub trait NamEsTrait: Send + Sync {
    fn some_function(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>>;

    fn some_function2(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>>;

    /// Gets the value of the Switch property.
    fn switch(&self) -> bool;
    /// Sets the value of the Switch property.
    fn set_switch(
        &self,
        switch: bool,
    );

    /// Gets the value of the SOME_PROPERTY property.
    fn some_property(&self) -> i32;
    /// Sets the value of the SOME_PROPERTY property.
    fn set_some_property(
        &self,
        some_property: i32,
    );

    /// Gets the value of the Some_Poperty2 property.
    fn some_poperty2(&self) -> i32;
    /// Sets the value of the Some_Poperty2 property.
    fn set_some_poperty2(
        &self,
        some_poperty2: i32,
    );

    /// Gets the value of the enum_property property.
    fn enum_property(&self) -> Enum_With_Under_scoresEnum;
    /// Sets the value of the enum_property property.
    fn set_enum_property(
        &self,
        enum_property: Enum_With_Under_scoresEnum,
    );

    fn publisher(&self) -> &NamEsPublisher;
}
