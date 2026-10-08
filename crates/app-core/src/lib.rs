//! Shared application core for mj-llm.
//!
//! Safe, framework-independent contracts used by the N0 native probe.

pub mod embedding;
pub mod integrity;
pub mod pin;

/// Product identifier; no inference runtime is initialized by this crate.
pub const PRODUCT_ID: &str = "mj-llm";
