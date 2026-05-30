use crate::api::struct_array2_interface::StructArray2InterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_array2_interface::StructArray2InterfacePublisher;
use parking_lot::RwLock;

pub struct StructArray2Interface {
    prop_bool: RwLock<StructBoolWithArray>,
    prop_int: RwLock<StructIntWithArray>,
    prop_float: RwLock<StructFloatWithArray>,
    prop_string: RwLock<StructStringWithArray>,
    prop_enum: RwLock<StructEnumWithArray>,
    publisher: StructArray2InterfacePublisher,
}

impl Default for StructArray2Interface {
    fn default() -> Self {
        Self { prop_bool: RwLock::new(Default::default()), prop_int: RwLock::new(Default::default()), prop_float: RwLock::new(Default::default()), prop_string: RwLock::new(Default::default()), prop_enum: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl StructArray2InterfaceTrait for StructArray2Interface {
    fn func_bool(
        &self,
        _param_bool: &StructBoolWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int(
        &self,
        _param_int: &StructIntWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float(
        &self,
        _param_float: &StructFloatWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_string(
        &self,
        _param_string: &StructStringWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_enum(
        &self,
        _param_enum: &StructEnumWithArray,
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop_bool(&self) -> StructBoolWithArray {
        self.prop_bool.read().clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &StructBoolWithArray,
    ) {
        let new_val = prop_bool.clone();
        let mut value = self.prop_bool.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_bool_changed.send(new_val);
    }

    fn prop_int(&self) -> StructIntWithArray {
        self.prop_int.read().clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &StructIntWithArray,
    ) {
        let new_val = prop_int.clone();
        let mut value = self.prop_int.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_int_changed.send(new_val);
    }

    fn prop_float(&self) -> StructFloatWithArray {
        self.prop_float.read().clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &StructFloatWithArray,
    ) {
        let new_val = prop_float.clone();
        let mut value = self.prop_float.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_float_changed.send(new_val);
    }

    fn prop_string(&self) -> StructStringWithArray {
        self.prop_string.read().clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &StructStringWithArray,
    ) {
        let new_val = prop_string.clone();
        let mut value = self.prop_string.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_string_changed.send(new_val);
    }

    fn prop_enum(&self) -> StructEnumWithArray {
        self.prop_enum.read().clone()
    }
    fn set_prop_enum(
        &self,
        prop_enum: &StructEnumWithArray,
    ) {
        let new_val = prop_enum.clone();
        let mut value = self.prop_enum.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_enum_changed.send(new_val);
    }

    fn publisher(&self) -> &StructArray2InterfacePublisher {
        &self.publisher
    }
}
