# Development EmbeddingGemma 2 comparison tools

[한국어](README.md) | [English](README.en.md) | [日本語](README.ja.md)

> Documentation translation, synchronized with the source on 2026-10-08. Commands, pinned identifiers, and implementation/verification status are preserved.

Text-only Python reference imported from mj_llm_wapper. This is not the product runtime and does not establish native/mobile support. Optional dependencies in this directory are outside the Rust workspace.

```bash
python3 -m venv .venv-reference
.venv-reference/bin/python -m pip install -r tools/reference/requirements-embeddings.txt
python3 -m unittest discover -s tools/reference -p 'test_embedding_server.py' -v
```

`embedding_server.py` is a development loopback server requiring a shared token. See `python3 tools/reference/embedding_server.py --help` for options. Its HTTP facade is not an implementation of the future product API. Do not commit model caches, weights, or user material.
