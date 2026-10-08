# mj-llm implementation roadmap

[한국어](roadmap.md) | [English](roadmap.en.md) | [日本語](roadmap.ja.md)

> Documentation translation, synchronized with the source on 2026-10-08. Commands, pinned identifiers, and implementation/verification status are preserved.

Unchecked items are not implemented.

- [x] Independent repository, design documents, and Rust workspace skeleton
- [x] Partial N0: SDK/artifact pin, macOS CPU load/text-image embed/generate/unload probes and automated tests
- [ ] N0: pin LiteRT-LM SDK/artifacts; load and text/image embedding feasibility on three PC operating systems, Android, and iOS
- [ ] N0: golden fixtures for prefix differences between official documents and preprocessing consistency
- [ ] N1: model download/verification/import, runtime registry, memory admission
- [ ] N1: PC document/photo indexing, search, grounded chat, SQLite storage/recovery
- [ ] N2: Android/iOS/iPadOS apps; phones/tablets; permissions, background/thermal/forced-termination recovery; first common release
- [ ] N3: audio/video segment indexing, search, and playback-position citations
- [ ] N4: additional models/adapters, optional developer API, MCP, trusted nodes, project transfer

## First implementation task

N0 selects platform SDKs, builds, physical devices, and exact artifact revisions. Existing Python embedding comparison tools are not evidence of native support. Establish Android/iOS feasibility before implementing extensive PC functionality.

The first experiment is load → text/image embedding → unload. Next verify small-model loading, first token, completion, cancellation, and sequential model switching. Rust skeleton expansion or PC UI work must not precede this decision.

## Deliverables and exit criteria

| Stage | Deliverables | Exit criteria | Current status |
|---|---|---|---|
| N0 | SDK/artifact lock, platform PoCs, golden fixtures, baseline device/OS/RAM table | Load/embed/generate and resource measurements on Windows x64/macOS arm64/Linux x64/Android arm64/iOS arm64; blockers resolved | In progress: macOS CPU smoke passed; other platforms, cancellation, and quality gates remain |
| N1 | Shared core, model store, PC app, indexing/search/chat, JSON import | Installation/offline E2E on three PC OSes; applicable G1–G10 gates; PC preview | Not implemented |
| N2 | Android/iOS/iPadOS apps and distribution packages | Physical Android phone/tablet and iPhone/iPad E2E plus PC regression; first cross-platform release | Not implemented |
| N3 | Audio/video decoders, segment retrieval/citations | Modality quality, time locators, long-running cancellation/recovery gates | Not implemented |
| N4 | Model expansion, API conformance, MCP/trusted nodes/transfer | Feature-specific security/compatibility gates; no automatic external transfer | Not implemented |

Pin exact SDK/artifact revisions and hashes from N0 results, not guesses. Record source commit, build tools, device, OS, backend, fixture, measurements, and reproduction commands per platform. Do not copy one OS's success into another OS's result.

## Release decisions

- All shared functions must pass on every required platform before claiming full platform support. PC-only success is a PC preview.
- Mark SDK incompatibility or memory failures as `blocked`. Evaluate embedded alternatives; do not remove required operating systems or silently use remote execution to claim success.
- Promote extra combinations such as Windows ARM64/macOS Intel/Linux arm64 only after separate validation.
- Browser-only execution is assumed to be a later candidate. If required, add JS/WASM, storage limits, WebGPU, and offline checks to N0 and update the N2 gate.

## Inherited but deferred work

Model Lab/Project Arena, Quick RAG, session quality profiles, conversation branching, multiple protocol facades, Portable Workspace, and per-model agent routing remain an extension backlog. Do not expand N0 to implement these features. Prioritize using P01–P10 in the [product plan](product-plan.en.md) and the [inheritance record](project-separation.en.md).

## Initial results

The shared core implements model SHA-256 checks, profiles, and vector validation. A separate crate connects the macOS C ABI, and the CLI executes real embedding/generation. Locally, 12 ordinary tests and 3 real-model integration tests passed, as did fmt and Clippy. A shared Windows/Linux/macOS CI definition exists; remote execution results have not yet been verified.

There is no user application or indexing/search store yet. CLI success does not mean all N0 gates or the PC/mobile product are complete. Continue using the [execution instructions, measurements, and remaining work](n0-feasibility.en.md).
