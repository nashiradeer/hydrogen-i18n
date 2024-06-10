//! Different builders for the [`I18n`] struct.

mod sync;
pub use sync::I18nBuilder;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
mod tokio;

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub use tokio::TokioI18nBuilder;
