#![cfg(feature = "native-macos")]

#[test]
#[ignore = "requires pinned SDK and MJ_N0_MODEL; performs real native inference"]
fn pinned_model_loads_embeds_text_and_image_then_reloads() {
    let model = std::env::var_os("MJ_N0_MODEL").expect("set MJ_N0_MODEL to pinned model path");
    let cache = tempfile::tempdir().unwrap();
    let report =
        mj_llm_n0::probe(std::path::Path::new(&model), cache.path()).expect("native probe");
    assert_eq!(report["status"], "embedding_smoke_passed");
    assert_eq!(report["runs"].as_array().unwrap().len(), 2);
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}

#[test]
#[ignore = "requires pinned SDK and MJ_N0_MODEL; exercises real SDK failures"]
fn native_errors_do_not_poison_the_engine() {
    use mj_llm_core::{embedding::TextTask, pin::VerifiedModel};
    use mj_llm_runtime_litert::EmbeddingEngine;
    let path = std::env::var_os("MJ_N0_MODEL").expect("MJ_N0_MODEL");
    let model = VerifiedModel::open(path).unwrap();
    let cache = tempfile::tempdir().unwrap();
    let mut engine = EmbeddingEngine::load(&model, cache.path()).unwrap();
    assert!(engine.embed_image(b"not a PNG or JPEG").is_err());
    assert!(
        engine
            .embed_text(TextTask::Query, &"cat ".repeat(1500))
            .is_err(),
        "SDK must reject token overflow, never silently truncate"
    );
    let valid = engine.embed_text(TextTask::Query, "hello").unwrap();
    assert_eq!(valid.values().len(), 768);
    assert!(
        engine
            .embed_image(include_bytes!("../../../fixtures/n0/red.png"))
            .is_ok()
    );
}

#[test]
#[ignore = "requires pinned SDK and MJ_N0_GENERATION_MODEL; real CPU generation"]
fn pinned_generation_model_returns_content_and_unloads() {
    let model = std::env::var_os("MJ_N0_GENERATION_MODEL").expect("MJ_N0_GENERATION_MODEL");
    let cache = tempfile::tempdir().unwrap();
    let report = mj_llm_n0::generation_probe(std::path::Path::new(&model), cache.path())
        .expect("native generator");
    assert_eq!(report["status"], "generation_smoke_passed");
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
