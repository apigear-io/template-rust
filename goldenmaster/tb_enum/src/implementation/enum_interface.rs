use crate::api::enum_interface::EnumInterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::enum_interface::EnumInterfacePublisher;
use parking_lot::RwLock;

pub struct EnumInterface {
    prop0: RwLock<Enum0Enum>,
    prop1: RwLock<Enum1Enum>,
    prop2: RwLock<Enum2Enum>,
    prop3: RwLock<Enum3Enum>,
    publisher: EnumInterfacePublisher,
}

impl Default for EnumInterface {
    fn default() -> Self {
        Self { prop0: RwLock::new(Default::default()), prop1: RwLock::new(Default::default()), prop2: RwLock::new(Default::default()), prop3: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl EnumInterfaceTrait for EnumInterface {
    fn func0(
        &self,
        _param0: Enum0Enum,
    ) -> ApiFuture<'_, Result<Enum0Enum, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func1(
        &self,
        _param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func2(
        &self,
        _param2: Enum2Enum,
    ) -> ApiFuture<'_, Result<Enum2Enum, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func3(
        &self,
        _param3: Enum3Enum,
    ) -> ApiFuture<'_, Result<Enum3Enum, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop0(&self) -> Enum0Enum {
        *self.prop0.read()
    }
    fn set_prop0(
        &self,
        prop0: Enum0Enum,
    ) {
        let mut value = self.prop0.write();
        if *value == prop0 {
            return;
        }
        *value = prop0;
        let _ = self.publisher.prop0_changed.send(prop0);
    }

    fn prop1(&self) -> Enum1Enum {
        *self.prop1.read()
    }
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    ) {
        let mut value = self.prop1.write();
        if *value == prop1 {
            return;
        }
        *value = prop1;
        let _ = self.publisher.prop1_changed.send(prop1);
    }

    fn prop2(&self) -> Enum2Enum {
        *self.prop2.read()
    }
    fn set_prop2(
        &self,
        prop2: Enum2Enum,
    ) {
        let mut value = self.prop2.write();
        if *value == prop2 {
            return;
        }
        *value = prop2;
        let _ = self.publisher.prop2_changed.send(prop2);
    }

    fn prop3(&self) -> Enum3Enum {
        *self.prop3.read()
    }
    fn set_prop3(
        &self,
        prop3: Enum3Enum,
    ) {
        let mut value = self.prop3.write();
        if *value == prop3 {
            return;
        }
        *value = prop3;
        let _ = self.publisher.prop3_changed.send(prop3);
    }

    fn publisher(&self) -> &EnumInterfacePublisher {
        &self.publisher
    }
}
