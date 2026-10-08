use std::{env, fs::File, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=MJ_LITERT_SDK_DIR");
    println!("cargo:rerun-if-changed=native/bridge.cc");
    println!("cargo:rerun-if-changed=../../catalog/n0.lock.json");
    if env::var_os("CARGO_FEATURE_NATIVE_MACOS").is_none() {
        return;
    }
    assert_eq!(
        env::var("CARGO_CFG_TARGET_OS").unwrap(),
        "macos",
        "native-macos requires macOS; other platform adapters are unverified"
    );
    let root = PathBuf::from(
        env::var_os("MJ_LITERT_SDK_DIR")
            .expect("set MJ_LITERT_SDK_DIR to the pinned XCFramework macos-arm64_x86_64 directory"),
    );
    let root = root.canonicalize().expect("SDK directory must exist");
    let sdk = mj_llm_core::pin::n0_pin().sdk;
    let mut files = sdk
        .macos_headers
        .into_iter()
        .map(|(name, hash)| (root.join("Headers").join(name), hash))
        .collect::<Vec<_>>();
    files.push((
        root.join("libCLiteRTLM_mac.dylib"),
        sdk.macos_library_sha256,
    ));
    for (path, expected) in files {
        println!("cargo:rerun-if-changed={}", path.display());
        let (_, actual) = mj_llm_core::integrity::sha256_reader(
            File::open(&path).expect("missing pinned SDK file"),
        )
        .expect("cannot hash SDK");
        assert_eq!(
            actual,
            expected,
            "SDK checksum mismatch: {}",
            path.display()
        );
    }
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .warnings(true)
        .warnings_into_errors(true)
        .include(root.join("Headers"))
        .file("native/bridge.cc")
        .compile("mj_litert_bridge");
    println!("cargo:rustc-link-search=native={}", root.display());
    println!("cargo:rustc-link-lib=dylib=CLiteRTLM_mac");
}
