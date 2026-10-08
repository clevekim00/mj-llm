# 開発用EmbeddingGemma 2比較ツール

[한국어](README.md) | [English](README.en.md) | [日本語](README.ja.md)

> 2026-10-08の原文に対応する文書翻訳です。コマンド・固定ID・実装/検証状態を維持しています。

mj_llm_wapperから継承したtext-only Python referenceです。製品runtimeではなく、native/mobile対応完了を意味しません。このdirectoryの任意依存はRust workspaceに含まれません。

```bash
python3 -m venv .venv-reference
.venv-reference/bin/python -m pip install -r tools/reference/requirements-embeddings.txt
python3 -m unittest discover -s tools/reference -p 'test_embedding_server.py' -v
```

`embedding_server.py`は共通tokenを要求する開発用loopback serverです。optionは`python3 tools/reference/embedding_server.py --help`を参照してください。このHTTP facadeを将来の製品API実装とは見なしません。model cache・重み・ユーザー資料をGitに追加しません。
