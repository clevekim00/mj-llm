use mj_llm_core::{
    integrity::{sha256_reader, verify_reader},
    pin::{VerifiedModel, n0_pin},
};
use std::io::{self, Read};

const ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

#[test]
fn sha256_matches_standard_vectors_and_rejects_tampering() {
    assert_eq!(sha256_reader(&b"abc"[..]).unwrap(), (3, ABC.into()));
    assert_eq!(
        sha256_reader(&b""[..]).unwrap().1,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert!(verify_reader(&b"abc"[..], 3, ABC).is_ok());
    for bytes in [&b"abd"[..], &b"ab"[..], &b"abcd"[..]] {
        assert_eq!(
            verify_reader(bytes, 3, ABC).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}

#[test]
fn handles_short_reads_interrupts_and_real_io_errors() {
    struct Short {
        interrupt: bool,
        input: &'static [u8],
    }
    impl Read for Short {
        fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
            if self.interrupt {
                self.interrupt = false;
                return Err(io::ErrorKind::Interrupted.into());
            }
            if self.input.is_empty() {
                return Ok(0);
            }
            out[0] = self.input[0];
            self.input = &self.input[1..];
            Ok(1)
        }
    }
    assert!(
        verify_reader(
            Short {
                interrupt: true,
                input: b"abc"
            },
            3,
            ABC
        )
        .is_ok()
    );
    struct Broken;
    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::ErrorKind::PermissionDenied.into())
        }
    }
    assert_eq!(
        sha256_reader(Broken).unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
}

#[test]
fn corrupt_model_cannot_become_verified() {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), b"not model weights").unwrap();
    assert!(VerifiedModel::open(file.path()).is_err());
    assert!(VerifiedModel::open(file.path().with_extension("missing")).is_err());
}

#[test]
fn lock_pins_revisions_hashes_and_experimental_profile() {
    let pin = n0_pin();
    assert_eq!(pin.schema_version, 1);
    for s in [&pin.sdk.revision, &pin.model.revision] {
        assert_eq!(s.len(), 40);
        assert!(s.bytes().all(|b| b.is_ascii_hexdigit()));
    }
    for s in [
        &pin.sdk.macos_archive_sha256,
        &pin.sdk.macos_library_sha256,
        &pin.model.sha256,
    ] {
        assert_eq!(s.len(), 64);
        assert!(s.bytes().all(|b| b.is_ascii_hexdigit()));
    }
    assert_eq!(pin.profile.dimension, 768);
    assert!(pin.profile.quality_status.starts_with("experimental"));
}
