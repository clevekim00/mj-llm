#!/bin/bash
# Developer-only, offline native verification. SDK and model downloads are explicit.
set -euo pipefail
if [[ $# -ne 3 ]]; then
  printf '%s\n' 'Usage: bash scripts/test-n0-macos.sh SDK_SLICE_DIR EMBEDDING_MODEL GENERATION_MODEL' >&2
  exit 2
fi
if [[ "$(uname -s)" != Darwin ]]; then
  printf '%s\n' 'This native adapter is currently macOS-only.' >&2
  exit 2
fi
export MJ_LITERT_SDK_DIR="$(cd "$1" && pwd)"
export DYLD_LIBRARY_PATH="$MJ_LITERT_SDK_DIR"
# Resolve before cd so caller-relative paths have unambiguous meaning.
export MJ_N0_MODEL="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"
export MJ_N0_GENERATION_MODEL="$(cd "$(dirname "$3")" && pwd)/$(basename "$3")"
cd "$(dirname "$0")/.."
cargo test --workspace --release --features mj-llm-n0/native-macos --locked
cargo test -p mj-llm-n0 --release --features native-macos --locked --test native -- --ignored --test-threads=1 --nocapture
cargo clippy --workspace --all-targets --features mj-llm-n0/native-macos --locked -- -D warnings
