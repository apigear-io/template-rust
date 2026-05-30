#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::nam_es::NamEsTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for NamEs.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct NamEsOlinkService {
    impl_: Arc<dyn NamEsTrait>,
}

impl NamEsOlinkService {
    pub fn new(impl_: Arc<dyn NamEsTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for NamEsOlinkService {
    fn olink_object_name(&self) -> &str {
        "tb.names.Nam_Es"
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
            "SOME_FUNCTION" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.some_function(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "Some_Function2" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.some_function2(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
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
            "Switch" => {
                if let Ok(v) = serde_json::from_value::<bool>(value) {
                    self.impl_.set_switch(v);
                }
            }
            "SOME_PROPERTY" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_some_property(v);
                }
            }
            "Some_Poperty2" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_some_poperty2(v);
                }
            }
            "enum_property" => {
                if let Ok(v) = serde_json::from_value::<Enum_With_Under_scoresEnum>(value) {
                    self.impl_.set_enum_property(v);
                }
            }
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
        tracing::info!("NamEs linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("NamEs unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "Switch": self.impl_.switch(),
            "SOME_PROPERTY": self.impl_.some_property(),
            "Some_Poperty2": self.impl_.some_poperty2(),
            "enum_property": self.impl_.enum_property()
        })
    }
}
