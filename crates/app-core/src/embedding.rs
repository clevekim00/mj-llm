//! Versioned embedding spaces: comparison never silently mixes profiles.
use crate::pin::n0_pin;
use sha2::{Digest, Sha256};
use std::{error::Error, fmt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddingSpace {
    id: String,
    dimension: usize,
}

impl EmbeddingSpace {
    pub fn n0() -> Self {
        let pin = n0_pin();
        // Includes SDK, artifact, prefix and preprocessing, not just model family.
        let identity = serde_json::to_vec(&(
            &pin.sdk,
            &pin.model,
            &pin.profile,
            std::env::consts::OS,
            std::env::consts::ARCH,
        ))
        .expect("pin serialization");
        Self {
            id: format!("sha256:{:x}", Sha256::digest(identity)),
            dimension: pin.profile.dimension,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn dimension(&self) -> usize {
        self.dimension
    }
}

#[derive(Clone, Debug)]
pub struct Embedding {
    space: EmbeddingSpace,
    values: Vec<f32>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum EmbeddingError {
    Dimension,
    NonFinite,
    NotNormalized,
    SpaceMismatch,
    EmptyInput,
    InputTooLarge,
}

impl fmt::Display for EmbeddingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for EmbeddingError {}

impl Embedding {
    pub fn new(space: EmbeddingSpace, values: Vec<f32>) -> Result<Self, EmbeddingError> {
        if values.len() != space.dimension || values.is_empty() {
            return Err(EmbeddingError::Dimension);
        }
        if values.iter().any(|v| !v.is_finite()) {
            return Err(EmbeddingError::NonFinite);
        }
        let norm = values
            .iter()
            .map(|&v| f64::from(v).powi(2))
            .sum::<f64>()
            .sqrt();
        if (norm - 1.0).abs() > 0.001 {
            return Err(EmbeddingError::NotNormalized);
        }
        Ok(Self { space, values })
    }
    pub fn values(&self) -> &[f32] {
        &self.values
    }
    pub fn space(&self) -> &EmbeddingSpace {
        &self.space
    }
    pub fn cosine(&self, other: &Self) -> Result<f64, EmbeddingError> {
        if self.space != other.space {
            return Err(EmbeddingError::SpaceMismatch);
        }
        let dot: f64 = self
            .values
            .iter()
            .zip(&other.values)
            .map(|(&a, &b)| f64::from(a) * f64::from(b))
            .sum();
        let norm = |v: &[f32]| v.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>().sqrt();
        Ok((dot / (norm(&self.values) * norm(&other.values))).clamp(-1.0, 1.0))
    }
}

#[derive(Clone, Copy)]
pub enum TextTask {
    Query,
    Document,
}

/// Experimental N0 profile, not yet a production retrieval-quality decision.
pub fn prepare_text(task: TextTask, input: &str) -> Result<String, EmbeddingError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(EmbeddingError::EmptyInput);
    }
    // This is an input byte guard; SDK overflow=error enforces actual token limits.
    if input.len() > 8192 {
        return Err(EmbeddingError::InputTooLarge);
    }
    let profile = n0_pin().profile;
    let prefix = match task {
        TextTask::Query => profile.query_prefix,
        TextTask::Document => profile.document_prefix,
    };
    Ok(prefix + input)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn space(id: &str, dimension: usize) -> EmbeddingSpace {
        EmbeddingSpace {
            id: id.into(),
            dimension,
        }
    }
    #[test]
    fn rejects_invalid_native_output() {
        for (values, error) in [
            (vec![], EmbeddingError::Dimension),
            (vec![f32::NAN, 0.0], EmbeddingError::NonFinite),
            (vec![f32::INFINITY, 0.0], EmbeddingError::NonFinite),
            (vec![0.0, 0.0], EmbeddingError::NotNormalized),
            (vec![2.0, 0.0], EmbeddingError::NotNormalized),
        ] {
            assert_eq!(Embedding::new(space("a", 2), values).unwrap_err(), error);
        }
    }
    #[test]
    fn rejects_different_spaces_even_at_same_dimension() {
        let a = Embedding::new(space("sdk-a", 2), vec![1.0, 0.0]).unwrap();
        let b = Embedding::new(space("sdk-b", 2), vec![1.0, 0.0]).unwrap();
        assert_eq!(a.cosine(&b), Err(EmbeddingError::SpaceMismatch));
    }
    #[test]
    fn cosine_orders_known_directions() {
        let a = Embedding::new(space("a", 2), vec![1.0, 0.0]).unwrap();
        let b = Embedding::new(space("a", 2), vec![0.0, 1.0]).unwrap();
        let c = Embedding::new(space("a", 2), vec![-1.0, 0.0]).unwrap();
        assert_eq!(a.cosine(&a), Ok(1.0));
        assert_eq!(a.cosine(&b), Ok(0.0));
        assert_eq!(a.cosine(&c), Ok(-1.0));
    }
    #[test]
    fn prefixes_are_golden_and_trim_unicode_whitespace() {
        assert_eq!(
            prepare_text(TextTask::Query, "\u{3000}고양이\n").unwrap(),
            "task: search query | text: 고양이"
        );
        assert_eq!(
            prepare_text(TextTask::Document, " 문서 ").unwrap(),
            "task: search result | text: 문서"
        );
        assert_eq!(
            prepare_text(TextTask::Query, " \n"),
            Err(EmbeddingError::EmptyInput)
        );
        assert_eq!(
            prepare_text(TextTask::Query, &"a".repeat(8193)),
            Err(EmbeddingError::InputTooLarge)
        );
    }
}
