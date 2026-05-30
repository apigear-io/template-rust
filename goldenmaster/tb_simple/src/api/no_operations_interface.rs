use tokio::sync::{watch, broadcast};

pub struct NoOperationsInterfacePublisher {
    pub prop_bool_changed: watch::Sender<bool>,
    pub prop_int_changed: watch::Sender<i32>,
    pub sig_void: broadcast::Sender<()>,
    pub sig_bool: broadcast::Sender<(bool,)>,
}

impl Default for NoOperationsInterfacePublisher {
    fn default() -> Self {
        Self { prop_bool_changed: watch::channel(Default::default()).0, prop_int_changed: watch::channel(Default::default()).0, sig_void: broadcast::Sender::new(16), sig_bool: broadcast::Sender::new(16) }
    }
}

pub trait NoOperationsInterfaceTrait: Send + Sync {
    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> bool;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> i32;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &self,
        prop_int: i32,
    );

    fn publisher(&self) -> &NoOperationsInterfacePublisher;
}
