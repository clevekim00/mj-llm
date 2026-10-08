# 개발용 EmbeddingGemma 2 비교 도구

기존 mj_llm_wapper에서 가져온 text-only Python reference이다.
제품 런타임이 아니며 native/mobile 지원 완료를 의미하지 않는다.
이 디렉터리의 선택 의존성은 Rust workspace에 포함되지 않는다.

```bash
python3 -m venv .venv-reference
.venv-reference/bin/python -m pip install -r tools/reference/requirements-embeddings.txt
python3 -m unittest discover -s tools/reference -p 'test_embedding_server.py' -v
```

embedding_server.py는 공통 token을 요구하는 개발용 loopback 서버다.
세부 사용 옵션은 `python3 tools/reference/embedding_server.py --help`로 확인한다.
이 서버의 HTTP facade는 향후 제품 API의 구현으로 간주하지 않는다.
모델 캐시·가중치·사용자 자료는 Git에 추가하지 않는다.
