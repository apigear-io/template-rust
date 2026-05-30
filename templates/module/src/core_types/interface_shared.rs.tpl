use std::sync::Arc;
use crate::api::{{snake .Interface.Name}}::{{Camel .Interface.Name}}Trait;
use crate::implementation::{{snake .Interface.Name}}::{{Camel .Interface.Name}};

/// Shared reference to a {{Camel .Interface.Name}} implementation.
pub type Shared{{Camel .Interface.Name}} = Arc<dyn {{Camel .Interface.Name}}Trait>;

/// Creates a new shared {{Camel .Interface.Name}} with the default implementation.
pub fn new_shared_{{snake .Interface.Name}}() -> Shared{{Camel .Interface.Name}} {
    Arc::new({{Camel .Interface.Name}}::default())
}
