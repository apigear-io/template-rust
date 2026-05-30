use crate::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nested_struct3_interface::NestedStruct3InterfacePublisher;
use parking_lot::RwLock;

pub struct NestedStruct3Interface {
    prop1: RwLock<NestedStruct1>,
    prop2: RwLock<NestedStruct2>,
    prop3: RwLock<NestedStruct3>,
    publisher: NestedStruct3InterfacePublisher,
}

impl Default for NestedStruct3Interface {
    fn default() -> Self {
        Self { prop1: RwLock::new(Default::default()), prop2: RwLock::new(Default::default()), prop3: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl NestedStruct3InterfaceTrait for NestedStruct3Interface {
    fn func1(
        &self,
        _param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func2(
        &self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func3(
        &self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
        _param3: &NestedStruct3,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop1(&self) -> NestedStruct1 {
        self.prop1.read().clone()
    }
    fn set_prop1(
        &self,
        prop1: &NestedStruct1,
    ) {
        let new_val = prop1.clone();
        let mut value = self.prop1.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop1_changed.send(new_val);
    }

    fn prop2(&self) -> NestedStruct2 {
        self.prop2.read().clone()
    }
    fn set_prop2(
        &self,
        prop2: &NestedStruct2,
    ) {
        let new_val = prop2.clone();
        let mut value = self.prop2.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop2_changed.send(new_val);
    }

    fn prop3(&self) -> NestedStruct3 {
        self.prop3.read().clone()
    }
    fn set_prop3(
        &self,
        prop3: &NestedStruct3,
    ) {
        let new_val = prop3.clone();
        let mut value = self.prop3.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop3_changed.send(new_val);
    }

    fn publisher(&self) -> &NestedStruct3InterfacePublisher {
        &self.publisher
    }
}
