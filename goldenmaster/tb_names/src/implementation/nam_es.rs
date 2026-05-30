use crate::api::nam_es::NamEsTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nam_es::NamEsPublisher;
use parking_lot::RwLock;

pub struct NamEs {
    switch: RwLock<bool>,
    some_property: RwLock<i32>,
    some_poperty2: RwLock<i32>,
    enum_property: RwLock<Enum_With_Under_scoresEnum>,
    publisher: NamEsPublisher,
}

impl Default for NamEs {
    fn default() -> Self {
        Self { switch: RwLock::new(Default::default()), some_property: RwLock::new(Default::default()), some_poperty2: RwLock::new(Default::default()), enum_property: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl NamEsTrait for NamEs {
    fn some_function(
        &self,
        _some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        Box::pin(async move { Ok(()) })
    }

    fn some_function2(
        &self,
        _some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        Box::pin(async move { Ok(()) })
    }

    fn switch(&self) -> bool {
        *self.switch.read()
    }
    fn set_switch(
        &self,
        switch: bool,
    ) {
        let mut value = self.switch.write();
        if *value == switch {
            return;
        }
        *value = switch;
        let _ = self.publisher.switch_changed.send(switch);
    }

    fn some_property(&self) -> i32 {
        *self.some_property.read()
    }
    fn set_some_property(
        &self,
        some_property: i32,
    ) {
        let mut value = self.some_property.write();
        if *value == some_property {
            return;
        }
        *value = some_property;
        let _ = self.publisher.some_property_changed.send(some_property);
    }

    fn some_poperty2(&self) -> i32 {
        *self.some_poperty2.read()
    }
    fn set_some_poperty2(
        &self,
        some_poperty2: i32,
    ) {
        let mut value = self.some_poperty2.write();
        if *value == some_poperty2 {
            return;
        }
        *value = some_poperty2;
        let _ = self.publisher.some_poperty2_changed.send(some_poperty2);
    }

    fn enum_property(&self) -> Enum_With_Under_scoresEnum {
        *self.enum_property.read()
    }
    fn set_enum_property(
        &self,
        enum_property: Enum_With_Under_scoresEnum,
    ) {
        let mut value = self.enum_property.write();
        if *value == enum_property {
            return;
        }
        *value = enum_property;
        let _ = self.publisher.enum_property_changed.send(enum_property);
    }

    fn publisher(&self) -> &NamEsPublisher {
        &self.publisher
    }
}
