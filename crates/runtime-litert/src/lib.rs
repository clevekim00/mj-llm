//! N0 CPU adapter. Only native-macos links an SDK; default builds never fake inference.
//! All unsafe code is confined to native.rs and the exception-contained C bridge.
#![deny(unsafe_code)]
use mj_llm_core::{
    embedding::{Embedding, TextTask},
    pin::VerifiedModel,
};
use std::{error::Error, fmt, path::Path};

pub mod generation;

#[cfg(feature = "native-macos")]
#[allow(unsafe_code)]
mod native;

#[derive(Debug)]
pub enum RuntimeError {
    Unavailable,
    InvalidInput,
    Native(i32),
    Io(std::io::Error),
    Embedding(mj_llm_core::embedding::EmbeddingError),
}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable => write!(f, "native SDK is not enabled; no inference was performed"),
            Self::InvalidInput => write!(f, "invalid or oversized input"),
            Self::Native(code) => write!(f, "native inference operation failed (code {code})"),
            Self::Io(_) => write!(f, "local model/cache I/O failed"),
            Self::Embedding(e) => write!(f, "embedding validation failed: {e}"),
        }
    }
}
impl Error for RuntimeError {}
impl From<std::io::Error> for RuntimeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<mj_llm_core::embedding::EmbeddingError> for RuntimeError {
    fn from(e: mj_llm_core::embedding::EmbeddingError) -> Self {
        Self::Embedding(e)
    }
}

pub struct EmbeddingEngine {
    #[cfg(feature = "native-macos")]
    inner: native::NativeEngine,
}

impl EmbeddingEngine {
    /// Consumes the owner; native fields are synchronously destroyed before return.
    pub fn unload(self) {}

    /// Synchronous, thread-confined N0 operation. Call from a native worker, never a UI thread.
    pub fn load(model: &VerifiedModel, cache: &Path) -> Result<Self, RuntimeError> {
        #[cfg(feature = "native-macos")]
        {
            Ok(Self {
                inner: native::NativeEngine::load(model, cache)?,
            })
        }
        #[cfg(not(feature = "native-macos"))]
        {
            let _ = (model, cache);
            Err(RuntimeError::Unavailable)
        }
    }
    pub fn embed_text(&mut self, task: TextTask, text: &str) -> Result<Embedding, RuntimeError> {
        let input = mj_llm_core::embedding::prepare_text(task, text)?;
        self.compute(0, input.as_bytes())
    }
    /// Caller supplies already-authorized encoded image bytes, never an external URL.
    /// N0 only: decoded-pixel admission and asset permissions belong to the future PlatformHost.
    pub fn embed_image(&mut self, encoded: &[u8]) -> Result<Embedding, RuntimeError> {
        if encoded.is_empty() || encoded.len() > 20 * 1024 * 1024 {
            return Err(RuntimeError::InvalidInput);
        }
        self.compute(1, encoded)
    }
    fn compute(&mut self, kind: u32, bytes: &[u8]) -> Result<Embedding, RuntimeError> {
        #[cfg(feature = "native-macos")]
        {
            self.inner.compute(kind, bytes)
        }
        #[cfg(not(feature = "native-macos"))]
        {
            let _ = (kind, bytes);
            Err(RuntimeError::Unavailable)
        }
    }
}

#[cfg(all(test, not(feature = "native-macos")))]
mod tests {
    use super::*;
    #[test]
    fn disabled_native_backend_never_returns_placeholder_vectors() {
        let mut engine = EmbeddingEngine {};
        assert!(matches!(
            engine.embed_text(TextTask::Query, "hello"),
            Err(RuntimeError::Unavailable)
        ));
        assert!(matches!(
            engine.embed_image(b"image"),
            Err(RuntimeError::Unavailable)
        ));
        assert!(matches!(
            engine.embed_image(b""),
            Err(RuntimeError::InvalidInput)
        ));
    }
}
