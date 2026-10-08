use crate::RuntimeError;
use mj_llm_core::pin::VerifiedGenerationModel;
use std::path::Path;

pub struct GenerationEngine {
    #[cfg(feature = "native-macos")]
    inner: crate::native::NativeGenerator,
}
impl GenerationEngine {
    /// Consumes the owner after all synchronous generation calls have returned.
    pub fn unload(self) {}

    pub fn load(model: &VerifiedGenerationModel, cache: &Path) -> Result<Self, RuntimeError> {
        #[cfg(feature = "native-macos")]
        {
            Ok(Self {
                inner: crate::native::NativeGenerator::load(model, cache)?,
            })
        }
        #[cfg(not(feature = "native-macos"))]
        {
            let _ = (model, cache);
            Err(RuntimeError::Unavailable)
        }
    }
    /// A fresh bounded conversation per call. Synchronous N0 probe, not a streaming chat API.
    pub fn generate(&mut self, prompt: &str) -> Result<serde_json::Value, RuntimeError> {
        if prompt.trim().is_empty() || prompt.len() > 1024 {
            return Err(RuntimeError::InvalidInput);
        }
        #[cfg(feature = "native-macos")]
        {
            self.inner.generate(prompt)
        }
        #[cfg(not(feature = "native-macos"))]
        {
            Err(RuntimeError::Unavailable)
        }
    }
}
