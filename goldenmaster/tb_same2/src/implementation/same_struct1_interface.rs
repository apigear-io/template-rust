use crate::api::same_struct1_interface::SameStruct1InterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_struct1_interface::SameStruct1InterfacePublisher;
use parking_lot::RwLock;

pub struct SameStruct1Interface {
    prop1: RwLock<Struct1>,
    publisher: SameStruct1InterfacePublisher,
}

impl Default for SameStruct1Interface {
    fn default() -> Self {
        Self { prop1: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl SameStruct1InterfaceTrait for SameStruct1Interface {
    fn func1(
        &self,
        _param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop1(&self) -> Struct1 {
        self.prop1.read().clone()
    }
    fn set_prop1(
        &self,
        prop1: &Struct1,
    ) {
        let new_val = prop1.clone();
        let mut value = self.prop1.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop1_changed.send(new_val);
    }

    fn publisher(&self) -> &SameStruct1InterfacePublisher {
        &self.publisher
    }
}
