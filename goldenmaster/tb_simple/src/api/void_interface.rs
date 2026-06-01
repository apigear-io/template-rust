use crate::api::{ApiError, ApiFuture};
use tokio::sync::{broadcast};

pub struct VoidInterfacePublisher {
    pub sig_void: broadcast::Sender<()>,
}

impl Default for VoidInterfacePublisher {
    fn default() -> Self {
        Self { sig_void: broadcast::Sender::new(16) }
    }
}

pub trait VoidInterfaceTrait: Send + Sync {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>>;

    fn publisher(&self) -> &VoidInterfacePublisher;
}

/// Async convenience wrappers for [`VoidInterfaceTrait`] operations.
/// Provided for every implementor (including `dyn VoidInterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait VoidInterfaceTraitAsync: VoidInterfaceTrait {
    fn func_void_async(&self) -> impl std::future::Future<Output = Result<(), ApiError>> + Send {
        async move { self.func_void().await }
    }
}

impl<T: VoidInterfaceTrait + ?Sized> VoidInterfaceTraitAsync for T {}
