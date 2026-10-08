# mj-llm product plan

[한국어](product-plan.md) | [English](product-plan.en.md) | [日本語](product-plan.ja.md)

> Documentation translation, synchronized with the source on 2026-10-08. Commands, pinned identifiers, and implementation/verification status are preserved.

> Baseline: 2026-10-08. An independent product inheriting ongoing planning and design from `mj_llm_wapper`.
> Current state: design plus an N0 macOS CPU inference probe. The requirements below are not a list of completed product features. See the [N0 record](n0-feasibility.en.md) for actual verification coverage.

## Goal

Search personal material and converse with visible evidence on PC, Android, and iOS without separately installing Ollama/Python. Use a shared core and platform applications, not one identical binary for every device.

Preserve the existing hub's device-aware model selection/installation flow and add personal-data retrieval and grounded conversations in one app. Users should not need to configure an engine server or install developer tools.

## Users and problems

- Individuals/researchers: find documents and photos in natural language and inspect evidence without sending them outside the device.
- Local AI users: distinguish download size from actual memory needs and install models verified for their devices.
- Developers: later use the same execution contracts through an optional local API. Inherit existing Narmer integration requirements as compatibility test material.

## Required platforms and the meaning of support

Installed apps for Windows, macOS, Linux, Android, and iOS/iPadOS are required. Test tablet layouts, file selection, and app return on Android tablets and iPad as well as phones. Browser-only execution is currently a later candidate; a PC UI using web technologies is distinct from inference running independently inside a browser.

| Target | Baseline for first common release | Additional validation |
|---|---|---|
| Windows | x86_64 | ARM64 |
| macOS | Apple Silicon arm64 | Intel x86_64 |
| Linux | x86_64, baseline distribution selected in N0 | arm64 and other distributions |
| Android | arm64, phones and tablets | Additional vendors/accelerators |
| iOS/iPadOS | arm64, iPhone and iPad | Additional device generations |

Architecture priorities are design defaults. Measure and pin minimum OS/RAM/storage and the Linux distribution in N0. Supporting all operating systems does not guarantee all old devices, CPUs, or models. Label extra combinations unverified rather than advertising official support.

After model setup, every required platform must add material, search documents/photos, hold text-grounded conversations, and save/recover state without another PC, server, or cloud. Semantic photo search is required. Answers about photo content must indicate whether generation-model vision or actual OCR/captions are available. Identical models, speed, or accelerators are not required; disclose functional limitations before installation.

## First-release assumptions

- Personal use, local first, Korean retrieval quality validated first
- PDF/Markdown/TXT/photo import and incremental indexing
- Open original pages/photos from semantic results
- Evidence-grounded conversation with valid source references
- One retrieval and one generation model verified per platform
- Offline use, cancellation, app termination/restart recovery

## Priorities and acceptance criteria

| ID | Requirement | Completion evidence | Stage |
|---|---|---|---|
| P01 | Standalone installation, device diagnosis, verified package recommendations | Install without Ollama/Python; explain incompatible-model blocks | N0–N2 |
| P02 | Download/resume/local import/hash verification/deletion | Recover interrupted downloads, reject corrupt files, defer deletion while in use | N1–N2 |
| P03 | Projects and PDF/Markdown/TXT/photo management | Handle revoked permissions/corruption per asset; show success/pending/failure counts | N1–N2 |
| P04 | Incremental indexing, semantic/keyword retrieval | Open source locators; immediately exclude deleted assets; zero cross-project exposure | N1–N2 |
| P05 | General and evidence-grounded streaming chat | Stop, save partial responses, restore after restart, validate citation locators | N1–N2 |
| P06 | Memory/thermal/lifecycle protection | Repeated physical-device queries; checkpoint recovery after forced termination | N0–N2 |
| P07 | Import legacy JSON conversations | Only user-selected files; duplicate/corrupt/failure rollback tests | N1 |
| P08 | Optional developer API | Local authentication; capability/error/stream/non-stream conformance | N4 |
| P09 | Audio/video retrieval and time citations | Separate modality, codec, and time-alignment evaluation | N3 |
| P10 | MCP, additional models, trusted nodes, project transfer | Feature-specific security and compatibility gates | N4 |

Conversation renaming/deletion is basic management. Message branching, full conversation search, Quick RAG, advanced session profiles, Model Lab/Project Arena, and automatic agent execution remain extension backlog items, not first common-release requirements.

## Main screens and user flows

1. **Start/model center:** diagnose device → recommend verified retrieval/generation packages → review size/licenses → install or import locally. Distinguish estimates from measurements on the device.
2. **Projects/material:** create project → select files/photos → monitor/cancel/resume indexing → retry failed assets only.
3. **Search:** natural-language query → page/photo results → open original → ask using selected evidence.
4. **Conversation:** distinguish general chat from project-grounded chat; show tokens, sources, stop, and partial-response status. State insufficient evidence when applicable.
5. **Settings/device:** model/asset storage, available capabilities, verification status, local-data deletion.

PC uses keyboard and large screens; mobile uses touch and OS file/photo pickers. Common UX requirements include screen-reader labels, adjustable text size, keyboard navigation, and textual status. Stretching a phone layout does not count as tablet validation.

## Later scope

Audio/video segment search, playback at original timestamps, additional models, MCP, optional trusted nodes, and project transfer.

Inherit Qwen/Gemma/DeepSeek/Mistral/Phi/gpt-oss families as an evaluation list. Old Ollama tags, catalog entries, or successful execution in another engine are not native-support evidence. Verify artifacts and generation/embedding capabilities separately per platform. MLX is a macOS-only adapter candidate, not a required shared-core dependency.

## Exclusions

Initial model training/fine-tuning, audio/video generation, automatic cloud uploads/device sync, automatic conversion of every Hugging Face model, and guaranteed large-model execution on mobile.

## Definition of done

Verify basic search/chat on all three PC OSes and physical Android/iOS devices without Ollama/Python. Determine exact minimum OS/devices, SDK/artifact versions, and performance budgets through N0 measurements. Follow the [architecture](architecture.en.md) for detailed contracts and gates.

N1 is a PC preview; N2 is the first common general release. If any required platform fails a gate, do not claim the all-platform release is complete. Korean retrieval Recall@5, latency/memory targets, and fixtures follow architecture section 11. Quality/performance figures are goals, not achieved results.

## Open decisions

- Browser-only release timing and separate storage, permission, and offline policies
- Minimum OS/devices/Linux distribution and timing for additional CPU architectures
- First generation model, pinned SDK/artifacts, distribution formats/channels
- Extension ordering and whether automatic synchronization is needed

Proceed with explicit assumptions for unresolved items. Failed measurements must not silently remove required platforms or replace local execution with remote inference.
