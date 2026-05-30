use crate::api::struct_interface::StructInterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_interface::StructInterfacePublisher;
use parking_lot::RwLock;

pub struct StructInterface {
    prop_bool: RwLock<StructBool>,
    prop_int: RwLock<StructInt>,
    prop_float: RwLock<StructFloat>,
    prop_string: RwLock<StructString>,
    publisher: StructInterfacePublisher,
}

impl Default for StructInterface {
    fn default() -> Self {
        Self { prop_bool: RwLock::new(Default::default()), prop_int: RwLock::new(Default::default()), prop_float: RwLock::new(Default::default()), prop_string: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl StructInterfaceTrait for StructInterface {
    fn func_bool(
        &self,
        _param_bool: &StructBool,
    ) -> ApiFuture<'_, Result<StructBool, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int(
        &self,
        _param_int: &StructInt,
    ) -> ApiFuture<'_, Result<StructInt, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float(
        &self,
        _param_float: &StructFloat,
    ) -> ApiFuture<'_, Result<StructFloat, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_string(
        &self,
        _param_string: &StructString,
    ) -> ApiFuture<'_, Result<StructString, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop_bool(&self) -> StructBool {
        self.prop_bool.read().clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &StructBool,
    ) {
        let new_val = prop_bool.clone();
        let mut value = self.prop_bool.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_bool_changed.send(new_val);
    }

    fn prop_int(&self) -> StructInt {
        self.prop_int.read().clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &StructInt,
    ) {
        let new_val = prop_int.clone();
        let mut value = self.prop_int.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_int_changed.send(new_val);
    }

    fn prop_float(&self) -> StructFloat {
        self.prop_float.read().clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &StructFloat,
    ) {
        let new_val = prop_float.clone();
        let mut value = self.prop_float.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_float_changed.send(new_val);
    }

    fn prop_string(&self) -> StructString {
        self.prop_string.read().clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &StructString,
    ) {
        let new_val = prop_string.clone();
        let mut value = self.prop_string.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_string_changed.send(new_val);
    }

    fn publisher(&self) -> &StructInterfacePublisher {
        &self.publisher
    }
}
