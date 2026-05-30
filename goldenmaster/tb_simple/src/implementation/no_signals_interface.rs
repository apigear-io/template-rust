use crate::api::no_signals_interface::NoSignalsInterfaceTrait;
use apigear::{ApiError, ApiFuture};
use crate::api::no_signals_interface::NoSignalsInterfacePublisher;
use parking_lot::RwLock;

pub struct NoSignalsInterface {
    prop_bool: RwLock<bool>,
    prop_int: RwLock<i32>,
    publisher: NoSignalsInterfacePublisher,
}

impl Default for NoSignalsInterface {
    fn default() -> Self {
        Self { prop_bool: RwLock::new(Default::default()), prop_int: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl NoSignalsInterfaceTrait for NoSignalsInterface {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        Box::pin(async move { Ok(()) })
    }

    fn func_bool(
        &self,
        _param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop_bool(&self) -> bool {
        *self.prop_bool.read()
    }
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    ) {
        let mut value = self.prop_bool.write();
        if *value == prop_bool {
            return;
        }
        *value = prop_bool;
        let _ = self.publisher.prop_bool_changed.send(prop_bool);
    }

    fn prop_int(&self) -> i32 {
        *self.prop_int.read()
    }
    fn set_prop_int(
        &self,
        prop_int: i32,
    ) {
        let mut value = self.prop_int.write();
        if *value == prop_int {
            return;
        }
        *value = prop_int;
        let _ = self.publisher.prop_int_changed.send(prop_int);
    }

    fn publisher(&self) -> &NoSignalsInterfacePublisher {
        &self.publisher
    }
}
