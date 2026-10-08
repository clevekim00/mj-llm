# Separation from the existing project

[한국어](project-separation.md) | [English](project-separation.en.md) | [日本語](project-separation.ja.md)

> Documentation translation, synchronized with the source on 2026-10-08. Commands, pinned identifiers, and implementation/verification status are preserved.

On 2026-10-08, the native multimodal design written in the mj_llm_wapper workspace was separated into mj-llm.

- Existing project: https://github.com/clevekim00/mj_llm_wapper
- New project: https://github.com/clevekim00/mj-llm
- Imported: chapter 5 of the latest native design and development EmbeddingGemma 2 comparison scripts.
- Not imported: Ollama gateway implementation, Git history, conversations, model weights, caches/build outputs.
- Existing repository files and uncommitted changes were preserved.

Paths such as `src/api.rs`, `src/runtime.rs`, `src/store.rs`, and `catalog/models.json` in migration tables belong to **the existing mj_llm_wapper repository**; they do not imply those files exist here. Existing code is reference material; the new core/apps are implemented in this repository. No shared package connects the two repositories yet.

Python API references in architecture.md describe comparison contracts. This repository does not yet expose a public HTTP API. The authoritative design for subsequent development is `docs/architecture.md` here.

## Ongoing-work inheritance record: 2026-10-08

The actual local repository referred to as `mj_llm_wrappper` is `/Users/youngwhankim/Project/mj_llm_wapper`. The documentation update examined both source HEAD `a23ce0ac2fd0fdc79f40e5fe79891e8f40c65174` and **uncommitted working files**. HEAD alone cannot reproduce the ongoing documentation/embedding work, so SHA-256 hashes of key planning sources are recorded below. The source repository was not modified.

| Source | Inherited content | Destination here |
|---|---|---|
| Implementation baseline in `README.md`, `blueprint-local-llm-hub.md` | Distinction between Ollama development version and native goal | README/roadmap status |
| `product-plan-local-llm-hub.md` | Users, model center/device diagnosis/chat/RAG, platform policy, extension backlog | `product-plan.md` |
| Chapter 5 of `blueprint-local-llm-hub.md` | Shared Rust core, LiteRT-LM, media/index/FFI/gates | Existing architecture and platform additions |
| `mj_llm_wrapper_request.md` | Narmer stream/non-stream, structured output, provenance, error contracts | Architecture section 9; N4 API tests |
| `src/device.rs`, `src/domain.rs` | Device information, string conversation model | Device diagnosis redesign and JSON migration |
| `src/runtime.rs`, `src/api.rs`, `src/store.rs` | Confirmed runtime trait, routes, JSON persistence | Reusable contracts versus newly implemented boundaries |
| `src/embeddings.rs` | Confirmed optional external local embedding-service adapter | Separate development comparison from embedded runtime |

Working-file hashes:

```text
c667bf2eb028f83ad19c6d150d22455c59f9815933a54ea5a83432beeb0a168a  README.md
6602cc293e96428b1a458ff0a721979c2886f2e1375abdf7acac34fc4ca6f434  product-plan-local-llm-hub.md
6a7bfd58452bbfe8259f135132bbea2c1b54f150be7ca9dd9ce2b328625f4a37  blueprint-local-llm-hub.md
53ad2eed795f9e7fedbd62ecbf1486a7e94c28bb9c59f95d93d888c7ce080c22  mj_llm_wrapper_request.md
```

This inherited planning/design, not runtime code or Git history. Source-project tests/model executions were not rerun during that work; their historical success is not reported as verification of the new product.

## Inherit, redesign, defer

| Category | Content | Reason |
|---|---|---|
| Inherit | Device-aware recommendations, installation flow, streaming chat, local first, runtime abstraction | User experience and contracts needed by the independent product |
| Redesign | Ollama pull → ModelStore; JSON → SQLite; Python embedding → embedded SDK; PWA → OS apps | Independent operation on all platforms without separate installation |
| Required in N1 | User-selected legacy JSON conversation import | Preserve history without automatic data collection |
| Defer to N4 | Narmer compatibility, MCP, trusted nodes, model-family expansion, project transfer | Validate separately from initial local search/chat |
| Extension backlog | Model Lab/Project Arena, Quick RAG, session profiles, agent routing, Portable Workspace | Preserve ideas without expanding prerequisite N0 work |
| Exclude | Product dependencies on Ollama/Python; automatic copying of private chats/weights/cache | Preserve standalone execution and data boundaries |

Changes to the source repository will not synchronize automatically. Future inheritance requires comparing source state and requirements again and updating this repository's planning, design, and implementation status together.
