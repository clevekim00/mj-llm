use std::process::Command;

#[test]
fn pin_is_machine_readable_without_an_sdk() {
    let out = Command::new(env!("CARGO_BIN_EXE_mj-llm-n0"))
        .arg("pin")
        .output()
        .unwrap();
    assert!(out.status.success());
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["sdk"]["version"], "0.18.0");
}
#[test]
fn invalid_artifact_exits_nonzero_and_never_claims_inference() {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), b"corrupted").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_mj-llm-n0"))
        .arg("verify")
        .arg(file.path())
        .output()
        .unwrap();
    assert!(!out.status.success());
    let value: serde_json::Value = serde_json::from_slice(&out.stderr).unwrap();
    assert_eq!(value["status"], "failed");
    assert_eq!(value["product_verified"], false);
    assert!(!String::from_utf8_lossy(&out.stderr).contains(&file.path().display().to_string()));
}
#[test]
fn rejects_unknown_and_incomplete_commands() {
    for args in [
        vec![],
        vec!["probe"],
        vec!["verify"],
        vec!["pin", "extra"],
        vec!["unknown"],
    ] {
        assert!(
            !Command::new(env!("CARGO_BIN_EXE_mj-llm-n0"))
                .args(args)
                .output()
                .unwrap()
                .status
                .success()
        );
    }
}
