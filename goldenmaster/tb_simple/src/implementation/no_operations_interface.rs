use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use crate::api::no_operations_interface::NoOperationsInterfacePublisher;
use parking_lot::RwLock;

pub struct NoOperationsInterface {
    prop_bool: RwLock<bool>,
    prop_int: RwLock<i32>,
    publisher: NoOperationsInterfacePublisher,
}

impl Default for NoOperationsInterface {
    fn default() -> Self {
        Self { prop_bool: RwLock::new(Default::default()), prop_int: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl NoOperationsInterfaceTrait for NoOperationsInterface {
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

    fn publisher(&self) -> &NoOperationsInterfacePublisher {
        &self.publisher
    }
}
