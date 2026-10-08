//! Streamed file verification; never loads model weights into a Rust buffer.
use sha2::{Digest, Sha256};
use std::io::{self, Read};

pub fn sha256_reader(mut input: impl Read) -> io::Result<(u64, String)> {
    let mut hash = Sha256::new();
    let mut bytes = 0u64;
    let mut buffer = [0u8; 65536];
    loop {
        let n = match input.read(&mut buffer) {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
        bytes = bytes
            .checked_add(n as u64)
            .ok_or_else(|| io::Error::other("file too large"))?;
    }
    Ok((bytes, format!("{:x}", hash.finalize())))
}

pub fn verify_reader(input: impl Read, expected_bytes: u64, expected_hash: &str) -> io::Result<()> {
    let (bytes, hash) = sha256_reader(input)?;
    if bytes != expected_bytes || hash != expected_hash {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "artifact size or SHA-256 mismatch",
        ));
    }
    Ok(())
}
