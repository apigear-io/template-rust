use crate::api::struct_array_interface::StructArrayInterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::struct_array_interface::StructArrayInterfacePublisher;
use parking_lot::RwLock;

pub struct StructArrayInterface {
    prop_bool: RwLock<Vec<StructBool>>,
    prop_int: RwLock<Vec<StructInt>>,
    prop_float: RwLock<Vec<StructFloat>>,
    prop_string: RwLock<Vec<StructString>>,
    prop_enum: RwLock<Vec<Enum0Enum>>,
    publisher: StructArrayInterfacePublisher,
}

impl Default for StructArrayInterface {
    fn default() -> Self {
        Self { prop_bool: RwLock::new(Default::default()), prop_int: RwLock::new(Default::default()), prop_float: RwLock::new(Default::default()), prop_string: RwLock::new(Default::default()), prop_enum: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl StructArrayInterfaceTrait for StructArrayInterface {
    fn func_bool(
        &self,
        _param_bool: &[StructBool],
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int(
        &self,
        _param_int: &[StructInt],
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float(
        &self,
        _param_float: &[StructFloat],
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_string(
        &self,
        _param_string: &[StructString],
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_enum(
        &self,
        _param_enum: &[Enum0Enum],
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop_bool(&self) -> Vec<StructBool> {
        self.prop_bool.read().clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &[StructBool],
    ) {
        let new_val = prop_bool.to_vec();
        let mut value = self.prop_bool.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_bool_changed.send(new_val);
    }

    fn prop_int(&self) -> Vec<StructInt> {
        self.prop_int.read().clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &[StructInt],
    ) {
        let new_val = prop_int.to_vec();
        let mut value = self.prop_int.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_int_changed.send(new_val);
    }

    fn prop_float(&self) -> Vec<StructFloat> {
        self.prop_float.read().clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &[StructFloat],
    ) {
        let new_val = prop_float.to_vec();
        let mut value = self.prop_float.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_float_changed.send(new_val);
    }

    fn prop_string(&self) -> Vec<StructString> {
        self.prop_string.read().clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &[StructString],
    ) {
        let new_val = prop_string.to_vec();
        let mut value = self.prop_string.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_string_changed.send(new_val);
    }

    fn prop_enum(&self) -> Vec<Enum0Enum> {
        self.prop_enum.read().clone()
    }
    fn set_prop_enum(
        &self,
        prop_enum: &[Enum0Enum],
    ) {
        let new_val = prop_enum.to_vec();
        let mut value = self.prop_enum.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_enum_changed.send(new_val);
    }

    fn publisher(&self) -> &StructArrayInterfacePublisher {
        &self.publisher
    }
}
