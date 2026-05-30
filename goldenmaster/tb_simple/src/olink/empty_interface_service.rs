#![allow(unused_imports, dead_code, clippy::never_loop)]
use crate::api::empty_interface::EmptyInterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for EmptyInterface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct EmptyInterfaceOlinkService {
    impl_: Arc<dyn EmptyInterfaceTrait>,
}

impl EmptyInterfaceOlinkService {
    pub fn new(impl_: Arc<dyn EmptyInterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for EmptyInterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "tb.simple.EmptyInterface"
    }

    #[allow(clippy::get_first)]
    fn olink_invoke(
        &self,
        method_id: &str,
        args: Value,
    ) -> Value {
        let member = Name::member_name(method_id);
        #[allow(unused_variables)]
        let arr = args.as_array();
        match member {
            _ => {
                tracing::warn!("Unknown method: {}", method_id);
                json!(null)
            }
        }
    }

    fn olink_set_property(
        &self,
        property_id: &str,
        value: Value,
    ) {
        let member = Name::member_name(property_id);
        match member {
            _ => {
                tracing::warn!("Unknown property: {}", property_id);
            }
        }
    }

    fn olink_linked(
        &self,
        _object_id: &str,
        _node: &dyn IRemoteNode,
    ) {
        tracing::info!("EmptyInterface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("EmptyInterface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({})
    }
}
