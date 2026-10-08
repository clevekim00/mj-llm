//! Reproducible N0 smoke fixtures. Success is embedding feasibility, not product acceptance.
use mj_llm_core::{
    embedding::{EmbeddingSpace, TextTask},
    pin::{VerifiedModel, n0_pin},
};
use mj_llm_runtime_litert::{EmbeddingEngine, RuntimeError};
use serde_json::{Value, json};
use std::{path::Path, time::Instant};

pub fn probe(model_path: &Path, cache: &Path) -> Result<Value, RuntimeError> {
    let started = Instant::now();
    let model = VerifiedModel::open(model_path)?;
    let verify_ms = started.elapsed().as_millis();
    let mut runs = Vec::new();
    let mut first = None;
    for _ in 0..2 {
        let started = Instant::now();
        let mut engine = EmbeddingEngine::load(&model, cache)?;
        let load_ms = started.elapsed().as_millis();
        let started = Instant::now();
        let query = engine.embed_text(TextTask::Query, "고양이는 어디에서 자고 있나요?")?;
        let positive =
            engine.embed_text(TextTask::Document, "고양이가 소파에서 잠을 자고 있습니다.")?;
        let negative = engine.embed_text(
            TextTask::Document,
            "기차표를 구매하려면 역 매표소로 가세요.",
        )?;
        let positive_score = query.cosine(&positive)?;
        let negative_score = query.cosine(&negative)?;
        if positive_score <= negative_score {
            return Err(RuntimeError::Native(4));
        }
        let image = engine.embed_image(include_bytes!("../../../fixtures/n0/red.png"))?;
        // Image smoke only: a synthetic solid color is not a semantic retrieval benchmark.
        let image_norm = image
            .values()
            .iter()
            .map(|&v| f64::from(v).powi(2))
            .sum::<f64>()
            .sqrt();
        let repeat = engine.embed_text(TextTask::Query, "고양이는 어디에서 자고 있나요?")?;
        let repeat_cosine = query.cosine(&repeat)?;
        if repeat_cosine < 0.9999 {
            return Err(RuntimeError::Native(4));
        }
        if let Some(previous) = &first
            && query.cosine(previous)? < 0.9999
        {
            return Err(RuntimeError::Native(4));
        }
        first = Some(query);
        let inference_ms = started.elapsed().as_millis();
        let started = Instant::now();
        engine.unload(); // synchronous: no cleanup hidden from the report
        runs.push(json!({"load_and_reverify_ms":load_ms,"inference_ms":inference_ms,
            "unload_ms":started.elapsed().as_millis(),"positive_cosine":positive_score,
            "negative_cosine":negative_score,"repeat_cosine":repeat_cosine,"image_norm":image_norm}));
    }
    let pin = n0_pin();
    Ok(json!({
        "schema_version":1,"status":"embedding_smoke_passed","product_verified":false,
        "os":std::env::consts::OS,"architecture":std::env::consts::ARCH,
        "sdk_revision":pin.sdk.revision,"sdk_version":pin.sdk.version,
        "artifact_revision":pin.model.revision,"artifact_sha256":pin.model.sha256,
        "profile":pin.profile.id,"profile_quality_status":pin.profile.quality_status,
        "embedding_space_id":EmbeddingSpace::n0().id(),"dimension":768,
        "backend":"cpu","fixture":"n0-synthetic-v1","verification_ms":verify_ms,"runs":runs,
        "remaining":["cancellation","retrieval quality corpus","resource pressure", "other platform devices"]
    }))
}

pub fn generation_probe(model_path: &Path, cache: &Path) -> Result<Value, RuntimeError> {
    let model = mj_llm_core::pin::VerifiedGenerationModel::open(model_path)?;
    let started = Instant::now();
    let mut engine = mj_llm_runtime_litert::generation::GenerationEngine::load(&model, cache)?;
    let load_ms = started.elapsed().as_millis();
    let started = Instant::now();
    let response = engine.generate("Reply with just the result of 2 + 2. /no_think")?;
    let generation_ms = started.elapsed().as_millis();
    let content = response
        .get("content")
        .and_then(Value::as_array)
        .ok_or(RuntimeError::Native(4))?;
    if !content.iter().any(|part| {
        part.get("text")
            .and_then(Value::as_str)
            .is_some_and(|text| !text.trim().is_empty())
    }) {
        return Err(RuntimeError::Native(4));
    }
    let started = Instant::now();
    engine.unload();
    let pin = n0_pin();
    Ok(
        json!({"schema_version":1,"status":"generation_smoke_passed","product_verified":false,
        "os":std::env::consts::OS,"architecture":std::env::consts::ARCH,"backend":"cpu",
        "sdk_revision":pin.sdk.revision,"artifact_revision":pin.generation_model.revision,
        "artifact_sha256":pin.generation_model.sha256,"load_and_reverify_ms":load_ms,"generation_ms":generation_ms,
        "unload_ms":started.elapsed().as_millis(),"max_output_tokens":32,"sampler":"top_p_sampler,k=1,p=1,temperature=1,seed=0", "context_tokens":512, "thinking":false,"response":response,
        "remaining":["streaming first-token timing","cancellation","generation quality corpus","other platform devices"]}),
    )
}
