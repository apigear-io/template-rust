use crate::api::many_param_interface::ManyParamInterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::many_param_interface::ManyParamInterfacePublisher;
use parking_lot::RwLock;

pub struct ManyParamInterface {
    prop1: RwLock<i32>,
    prop2: RwLock<i32>,
    prop3: RwLock<i32>,
    prop4: RwLock<i32>,
    publisher: ManyParamInterfacePublisher,
}

impl Default for ManyParamInterface {
    fn default() -> Self {
        Self { prop1: RwLock::new(Default::default()), prop2: RwLock::new(Default::default()), prop3: RwLock::new(Default::default()), prop4: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl ManyParamInterfaceTrait for ManyParamInterface {
    fn func1(
        &self,
        _param1: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func2(
        &self,
        _param1: i32,
        _param2: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func3(
        &self,
        _param1: i32,
        _param2: i32,
        _param3: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func4(
        &self,
        _param1: i32,
        _param2: i32,
        _param3: i32,
        _param4: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop1(&self) -> i32 {
        *self.prop1.read()
    }
    fn set_prop1(
        &self,
        prop1: i32,
    ) {
        let mut value = self.prop1.write();
        if *value == prop1 {
            return;
        }
        *value = prop1;
        let _ = self.publisher.prop1_changed.send(prop1);
    }

    fn prop2(&self) -> i32 {
        *self.prop2.read()
    }
    fn set_prop2(
        &self,
        prop2: i32,
    ) {
        let mut value = self.prop2.write();
        if *value == prop2 {
            return;
        }
        *value = prop2;
        let _ = self.publisher.prop2_changed.send(prop2);
    }

    fn prop3(&self) -> i32 {
        *self.prop3.read()
    }
    fn set_prop3(
        &self,
        prop3: i32,
    ) {
        let mut value = self.prop3.write();
        if *value == prop3 {
            return;
        }
        *value = prop3;
        let _ = self.publisher.prop3_changed.send(prop3);
    }

    fn prop4(&self) -> i32 {
        *self.prop4.read()
    }
    fn set_prop4(
        &self,
        prop4: i32,
    ) {
        let mut value = self.prop4.write();
        if *value == prop4 {
            return;
        }
        *value = prop4;
        let _ = self.publisher.prop4_changed.send(prop4);
    }

    fn publisher(&self) -> &ManyParamInterfacePublisher {
        &self.publisher
    }
}
