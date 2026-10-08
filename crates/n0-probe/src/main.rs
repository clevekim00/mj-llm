use mj_llm_core::pin::{VerifiedModel, n0_pin};
use serde_json::json;
use std::{env, path::Path, process::ExitCode};

fn run() -> Result<serde_json::Value, String> {
    let args = env::args_os().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command] if command == "pin" => Ok(serde_json::to_value(n0_pin()).unwrap()),
        [command, model] if command == "verify" => {
            VerifiedModel::open(Path::new(model))
                .map_err(|e| format!("artifact verification failed: {}", e.kind()))?;
            Ok(
                json!({"status":"artifact_verified","inference_performed":false,"sha256":n0_pin().model.sha256}),
            )
        }
        [command, model, cache] if command == "probe" => {
            mj_llm_n0::probe(Path::new(model), Path::new(cache)).map_err(|e| e.to_string())
        }
        [command, model, cache] if command == "generate-probe" => {
            mj_llm_n0::generation_probe(Path::new(model), Path::new(cache)).map_err(|e| e.to_string())
        }
        _ => Err("usage: mj-llm-n0 pin | verify MODEL | probe MODEL CACHE_DIR | generate-probe MODEL CACHE_DIR".into()),
    }
}
fn main() -> ExitCode {
    match run() {
        Ok(value) => {
            println!("{}", serde_json::to_string_pretty(&value).unwrap());
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!(
                "{}",
                json!({"status":"failed","message":message,"product_verified":false})
            );
            ExitCode::FAILURE
        }
    }
}
