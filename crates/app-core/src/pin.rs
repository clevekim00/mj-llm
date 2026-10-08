use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::File,
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Pin {
    pub schema_version: u32,
    pub sdk: Sdk,
    pub model: Model,
    pub generation_model: Model,
    pub profile: Profile,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sdk {
    pub version: String,
    pub revision: String,
    pub macos_archive_url: String,
    pub macos_archive_sha256: String,
    pub macos_library_sha256: String,
    pub macos_headers: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub repository: String,
    pub revision: String,
    pub filename: String,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub id: String,
    pub dimension: usize,
    pub query_prefix: String,
    pub document_prefix: String,
    pub preprocessing: String,
    pub quality_status: String,
}

pub fn n0_pin() -> Pin {
    serde_json::from_str(include_str!("../../../catalog/n0.lock.json"))
        .expect("checked-in N0 lock must be valid")
}

/// Proof of the pinned artifact's content at verification time.
/// The caller must keep the file immutable while the SDK uses it.
#[derive(Debug)]
pub struct VerifiedModel {
    path: PathBuf,
}

/// Separate proof type prevents supplying an embedding model to a generator.
pub struct VerifiedGenerationModel {
    path: PathBuf,
}
impl VerifiedGenerationModel {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = verify_path(path.as_ref(), &n0_pin().generation_model)?;
        Ok(Self { path })
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl VerifiedModel {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = verify_path(path.as_ref(), &n0_pin().model)?;
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn verify_path(path: &Path, model: &Model) -> io::Result<PathBuf> {
    let path = path.canonicalize()?;
    let file = File::open(&path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "model must be a regular file",
        ));
    }
    if metadata.len() != model.bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "artifact size mismatch",
        ));
    }
    crate::integrity::verify_reader(file, model.bytes, &model.sha256)?;
    Ok(path)
}
