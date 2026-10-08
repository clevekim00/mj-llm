# mj-llm

[한국어](README.md) | [English](README.en.md) | [日本語](README.ja.md)

> Documentation translation, synchronized with the source on 2026-10-08. Commands, pinned identifiers, and implementation/verification status are preserved.

A local multimodal AI project that runs directly on Windows, macOS, Linux, Android, and iOS/iPadOS. The product is intended to require no separate Ollama or Python installation.

> Current status: an N0 macOS CPU verification tool is implemented. Real text/image embedding and small-model generation tests passed. User applications and native adapters for other platforms are not implemented yet.

## Product direction

- First validate a shared Rust core and embedded LiteRT-LM.
- Keep the EmbeddingGemma 2 retrieval model separate from the answer-generation model.
- First release: document/photo search and evidence-grounded conversations.
- Later: audio/video segment search, additional models, and tool integrations.
- Run locally after initial model setup; no automatic cloud uploads.

## Documents

Documentation is available in Korean, English, and Japanese; use the links at the top of each document. Translated documentation does not establish multilingual model quality. Fixed Korean N0 test inputs remain unchanged.

- [Illustrated beginner guide](docs/user-guide.en.html)
- [Architecture](docs/architecture.en.md)
- [Product plan and platform scope](docs/product-plan.en.md)
- [Implementation roadmap](docs/roadmap.en.md)
- [Project inheritance and separation](docs/project-separation.en.md)
- [Development embedding comparison tools](tools/reference/README.en.md)
- [N0 execution, testing, and results](docs/n0-feasibility.en.md)

## Repository structure

```text
apps/                   Planned platform applications and responsibilities
crates/app-core/        Model integrity, preprocessing profiles, embedding spaces
crates/runtime-litert/  Narrow C ABI and macOS CPU native adapter
crates/n0-probe/        Real load/embed/generate/unload verification CLI
contracts/              Planned API and data contracts
catalog/                Planned verified model manifests
catalog/n0.lock.json     N0 SDK/artifact revision/hash (not a product catalog)
docs/                   Architecture, product scope, roadmap
tools/reference/        Python embedding comparison tools (not a product dependency)
```

## Development checks

Use a toolchain supporting Rust edition 2024.

```bash
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

The current milestone is [N0 runtime feasibility](docs/roadmap.en.md). Shared tests without the SDK do not replace native verification. Run real-model tests explicitly using the [N0 instructions](docs/n0-feasibility.en.md).

```bash
cargo run -p mj-llm-n0 --locked -- pin
cargo run -p mj-llm-n0 --release --locked -- verify /absolute/path/to/embeddinggemma-2-text-vision-440m.litertlm
```

The CLI does not download automatically or run remote inference. The default build reports native support as unavailable instead of returning fake vectors.

N1 is a PC preview; N2 is the first general release across all required platforms. Physical phone/tablet testing is required. Minimum OS and device specifications will be determined by N0 measurements. Browser-only execution remains a possible later extension.

## Related project

[mj_llm_wapper](https://github.com/clevekim00/mj_llm_wapper) is the existing local LLM gateway. `mj-llm` has its own Git repository and release cycle, with no current runtime dependency on that project's code.

## License

[MIT](LICENSE). Model weights, SDKs, and external libraries retain their respective licenses.
