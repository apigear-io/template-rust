#![allow(unused_imports, dead_code, clippy::never_loop)]
use crate::api::empty_interface::EmptyInterfaceTrait;
use tracing;

/// Trace decorator for EmptyInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct EmptyInterfaceTraced<T: EmptyInterfaceTrait> {
    inner: T,
}

impl<T: EmptyInterfaceTrait> EmptyInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: EmptyInterfaceTrait> EmptyInterfaceTrait for EmptyInterfaceTraced<T> {}
