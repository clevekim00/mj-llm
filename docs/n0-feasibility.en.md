# N0 native feasibility implementation and execution record

[한국어](n0-feasibility.md) | [English](n0-feasibility.en.md) | [日本語](n0-feasibility.ja.md)

> Documentation translation, synchronized with the source on 2026-10-08. Commands, pinned identifiers, and implementation/verification status are preserved.

2026-10-08. This is the first native feasibility implementation, not the whole product. It calls Rust CLI → project C bridge → official LiteRT-LM C API without Ollama/Python processes. There is no automatic downloading or user-data transmission code.

## Implemented scope

| Location | Actual implementation |
|---|---|
| `catalog/n0.lock.json` | SDK commit; macOS library/header/zip hashes; retrieval/generation model revisions, sizes, SHA-256 |
| `crates/app-core` | Streaming SHA-256 verification, model-specific proof types, fixed text prefixes, vector dimension/finite/norm validation, rejection of cross-space comparisons |
| `crates/runtime-litert` | Thread-confined safe wrapper, exception-contained C ABI, input/output copying, RAII cleanup, CPU embedding and synchronous generation |
| `crates/n0-probe` | `pin`, `verify`, `probe`, `generate-probe`; JSON results and failure exit codes |
| `fixtures/n0` | Project-created 64×64 red PNG; Korean positive/negative sentences fixed in probe source |
| `scripts/test-n0-macos.sh` | Reproduce native build, ordinary tests, real-model tests, Clippy |
| `.github/workflows/rust.yml` | Shared Windows/Linux/macOS Rust checks; not native/device testing |

The shared core uses `unsafe_code=forbid`. Unsafe code is confined to `runtime-litert/src/native.rs`, with ownership/buffer-lifetime reasoning at each ABI call. The bridge converts C++ exceptions to status codes and never frees foreign-allocator memory in Rust. Engines are `!Send/!Sync`; inference requires `&mut self`. Unload cannot occur before a synchronous call returns. Callbacks are not used yet.

SDKs and weights are excluded from the repository. Builds verify SDK library/header hashes; execution checks model size/hash and rechecks immediately before native load. N0 assumes **immutable local files owned by the developer**. A production ModelStore preventing another process from replacing a verified file is not implemented. Do not replace SDK files after building either.

## Pinned inputs and sources

- LiteRT-LM `0.18.0`, commit `b2f686e2ed4718fb84ec398a61dd59ca0f0aff27`: [official package and zip checksum](https://github.com/google-ai-edge/LiteRT-LM/blob/b2f686e2ed4718fb84ec398a61dd59ca0f0aff27/Package.swift).
- Retrieval: [pinned EmbeddingGemma 2 text-vision distribution](https://huggingface.co/litert-community/embeddinggemma-2-text-vision-440m-litert-lm/tree/e301f74d5551b0c2641bd5cb4652a76239d5c5f8), 387,710,976 bytes. SHA-256 is in the lock file.
- Generation: [pinned Qwen3-0.6B distribution](https://huggingface.co/litert-community/Qwen3-0.6B/tree/a3c5d805ae362dff7f580bc25f2dfb9a5a7eaa76), 614,236,160 bytes. A small N0 candidate, not a final first-product model decision.
- Model/SDK licenses follow their upstream distributions. This repository's MIT license does not replace weight licenses.

Retrieval profile: CPU float32, 2 threads, text signature 128–512, vision 70 tokens/image, SDK special tokens, L2 normalization, output 768, overflow error. The SDK decodes/resizes images. The synthetic PNG has no EXIF. Product EXIF handling, pixel limits, and permission-aware asset adapters are not implemented; this is not a user-image ingestion feature.

Prefixes `task: search query | text: ` and `task: search result | text: ` form an **experimental profile**. String regression and Korean smoke-ranking tests passed; corpus comparisons with other prefixes in the model card remain. These are not established production-optimal prefixes. SDK/model/profile or OS/architecture changes alter the embedding-space ID; changing only the generation model does not alter the retrieval space.

Generation uses CPU, context 512, maximum 32 output tokens, top-p sampler (k=1, p=1, temperature=1, seed=0), artifact-provided template, and a request for `enable_thinking=false`. GREEDY and TOP_K actually returned `UNIMPLEMENTED` in the SDK CPU sampler; the implementation was changed to TOP_P after checking [supported paths in the pinned source](https://github.com/google-ai-edge/LiteRT-LM/blob/b2f686e2ed4718fb84ec398a61dd59ca0f0aff27/runtime/components/sampler_factory.cc). Empty think tags are preserved in the raw SDK response.

## Setup and execution

Development used Rust 1.95.0 and a macOS C++ compiler/Xcode command-line tools. These are not intended product-user dependencies. The default Rust build without the SDK rejects native calls as `Unavailable`.

Download these files from the repository root. Allow approximately 1.05GB for downloads plus inference memory. You can instead specify existing files with identical hashes.

```bash
mkdir -p downloads/n0 models .mj-llm/n0
curl -fL https://github.com/google-ai-edge/LiteRT-LM/releases/download/v0.18.0/CLiteRTLM_mac.xcframework.zip -o downloads/n0/sdk.zip
shasum -a 256 downloads/n0/sdk.zip
# Expected: 5f6ee68d95eeccb084c6e66d5ee47255e3020fa0fb29696dd0301ae26d6cfb4f
unzip -q downloads/n0/sdk.zip -d downloads/n0/sdk
curl -fL https://huggingface.co/litert-community/embeddinggemma-2-text-vision-440m-litert-lm/resolve/e301f74d5551b0c2641bd5cb4652a76239d5c5f8/embeddinggemma-2-text-vision-440m.litertlm -o models/embeddinggemma-2-text-vision-440m.litertlm
curl -fL https://huggingface.co/litert-community/Qwen3-0.6B/resolve/a3c5d805ae362dff7f580bc25f2dfb9a5a7eaa76/Qwen3-0.6B.litertlm -o models/Qwen3-0.6B.litertlm
```

Do not extract a zip whose hash differs. Native builds also compare extracted libraries/headers with the lock and stop on mismatch. The CLI verifies models.

```bash
cargo run -p mj-llm-n0 --locked -- pin
cargo run -p mj-llm-n0 --release --locked -- verify models/embeddinggemma-2-text-vision-440m.litertlm
bash scripts/test-n0-macos.sh downloads/n0/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64 models/embeddinggemma-2-text-vision-440m.litertlm models/Qwen3-0.6B.litertlm
```

To run only the CLI:

```bash
export MJ_LITERT_SDK_DIR="$PWD/downloads/n0/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64"
export DYLD_LIBRARY_PATH="$MJ_LITERT_SDK_DIR"
cargo build -p mj-llm-n0 --release --features native-macos --locked
target/release/mj-llm-n0 probe models/embeddinggemma-2-text-vision-440m.litertlm .mj-llm/n0/embedding-cache
target/release/mj-llm-n0 generate-probe models/Qwen3-0.6B.litertlm .mj-llm/n0/generation-cache
```

Successful JSON goes to stdout; SDK diagnostics may appear on stderr. Failure exits with code 1 without success JSON. `verify` checks integrity only. `probe` performs two loads, four Korean text embeddings plus one image embedding, and unloads. `generate-probe` checks for a nonempty answer to a fixed arithmetic question. Once models are ready, inference needs no network, Python, or Ollama.

## Tests and results

- 12 ordinary tests: SHA-256 known vectors/tampering/truncation; short reads/Interrupted/real I/O errors; wrong model; lock pins; vector dimension/NaN/Inf/zero norm; mixed-space rejection; cosine; prefixes; CLI errors; no fake results when native support is disabled.
- 3 real-model tests: text/image/repeat/reload; recovery after corrupt image and 1509-token overflow; small-model generation/cleanup. `#[ignore]` separates these from ordinary CI; the shell script explicitly runs them. Missing environment variables fail instead of silently skipping as success.
- Passed `cargo fmt --all -- --check`, `cargo test --workspace --locked`, and `cargo clippy --workspace --all-targets --locked -- -D warnings`. Release tests and Clippy with the native feature also passed.
- GitHub CI is defined only; remote execution and other-OS success are not claimed.

Local machine: MacBookPro17,1 (arm64), 16GiB RAM, macOS 27.0 (26A428). Retrieval positive cosine was about 0.893, negative about 0.665, repeat cosine 0.9999999999999999, and image norm about 0.99999985. Generation included `2 + 2 = 4`. This checks functionality on one sample, not retrieval or answer accuracy generally.

The [machine-readable verification record](n0-results/macos-arm64-cpu.json) contains final source-file hashes, toolchain, SDK/artifacts, and results. Single observations using sequential `time -l` runs are below. RSS and footprint are different OS metrics; do not add them.

| Probe | Maximum RSS (bytes) | Peak memory footprint (bytes) |
|---|---:|---:|
| Text/image embedding + reload | 654,360,576 | 311,657,576 |
| Qwen3 generation | 1,700,397,056 | 1,072,269,616 |

The initial sandboxed `time -l` failed because resource-measurement sysctl access was unavailable, although inference succeeded. These values came from sequential reruns with measurement permission. Inference tests also passed separately inside the sandbox.

`load_and_reverify_ms` includes model hash rechecking. `inference_ms` sums five embedding calls; it is neither single-query latency nor p95. One measurement under OS-cache/development load is not a formal benchmark. Reports do not store raw vectors or user material.

## Platform status and next gates

| Combination | Implementation/verification status |
|---|---|
| macOS arm64 CPU | Native adapter implemented; real embedding/generation smoke passed; not full N0 acceptance |
| macOS Intel | Slice exists in universal SDK; execution here unverified |
| Windows x64 / Linux x64 | Shared Rust CI definitions only; native adapters/execution unverified |
| Android arm64 | Native host adapter and physical-device execution unverified |
| iOS/iPadOS arm64 | Native host adapter/physical device unverified; local iPad was unavailable, so no device execution |

Remaining N0: pin other-OS SDK/host bridges and run on devices; streaming first token/cancellation/exactly one terminal event; resource/thermal/memory pressure; admission for sequential model switching in one app; prefix-comparison corpus/retrieval quality; signed-package installation. Manage N1 UI/SQLite/ingestion separately. Current synchronous probes have no forced-cancellation API, deadline, or GPU fallback.
