# mj-llm 작업 지침

- 먼저 README.md, docs/architecture.md, docs/roadmap.md를 읽는다.
- 이 저장소는 독립 PC·Android·iOS 제품이다. mj_llm_wapper의 Ollama 게이트웨이를 복제하지 않는다.
- 현재는 설계와 Rust 골격 단계다. 구현·실기기 검증·계획을 구분해서 보고한다.
- 첫 구현 작업은 N0: 고정 LiteRT-LM SDK/artifact로 플랫폼별 로드·임베딩 feasibility 검증이다.
- 제품 실행에 Ollama나 Python 설치를 요구하지 않는다. tools/reference는 개발 비교용이다.
- Rust 코어는 UI/HTTP 프레임워크와 분리한다. 플랫폼 SDK/FFI의 unsafe는 별도 좁은 경계에서 검토한다.
- 미디어 입력은 권한 있는 asset으로 처리하고 사용자 자료를 자동 외부 전송하지 않는다.
- artifact revision/hash, 전처리와 prefix profile을 고정한다. 다른 embedding-space 벡터를 혼합하지 않는다.
- 모델 가중치, 개인 자료, 비밀값, 빌드 산출물을 커밋하지 않는다.
- 설계를 변경하면 문서와 실제 구현 상태를 함께 갱신한다.
- Rust 변경 후 cargo fmt --all -- --check, cargo test --workspace --locked,
  cargo clippy --workspace --all-targets --locked -- -D warnings를 실행한다.
- 단일 에이전트 작업을 기본으로 한다. 사용자 요청 없는 별도 작업/서브에이전트는 만들지 않는다.
