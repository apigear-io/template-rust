use crate::api::{ApiError, ApiFuture};
use tokio::sync::{broadcast};

pub struct NoPropertiesInterfacePublisher {
    pub sig_void: broadcast::Sender<()>,
    pub sig_bool: broadcast::Sender<(bool,)>,
}

impl Default for NoPropertiesInterfacePublisher {
    fn default() -> Self {
        Self { sig_void: broadcast::Sender::new(16), sig_bool: broadcast::Sender::new(16) }
    }
}

pub trait NoPropertiesInterfaceTrait: Send + Sync {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>>;

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>>;

    fn publisher(&self) -> &NoPropertiesInterfacePublisher;
}

/// Async convenience wrappers for [`NoPropertiesInterfaceTrait`] operations.
/// Provided for every implementor (including `dyn NoPropertiesInterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait NoPropertiesInterfaceTraitAsync: NoPropertiesInterfaceTrait {
    fn func_void_async(&self) -> impl std::future::Future<Output = Result<(), ApiError>> + Send {
        async move { self.func_void().await }
    }

    fn func_bool_async(
        &self,
        param_bool: bool,
    ) -> impl std::future::Future<Output = Result<bool, ApiError>> + Send {
        async move { self.func_bool(param_bool).await }
    }
}

impl<T: NoPropertiesInterfaceTrait + ?Sized> NoPropertiesInterfaceTraitAsync for T {}
