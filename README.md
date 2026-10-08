# mj-llm

PC·Android·iOS에서 직접 실행하는 로컬 멀티모달 AI 프로젝트입니다.
Ollama나 Python을 별도로 설치하지 않는 제품을 목표로 합니다.

> 현재 상태: 설계 및 Rust 작업공간 초기화. 내장 추론 엔진·사용자 앱은 아직 구현되지 않았습니다.

## 제품 방향

- 공통 Rust 코어 + 내장 LiteRT-LM을 우선 검증합니다.
- EmbeddingGemma 2 검색 모델과 답변 생성 모델을 분리합니다.
- 첫 출시: 문서·사진 검색과 근거 기반 대화.
- 후속: 음성·영상 구간 검색, 추가 모델과 도구 연결.
- 최초 모델 준비 후 로컬 실행이 기본이며 자동 클라우드 전송은 하지 않습니다.

## 문서

- [아키텍처 설계](docs/architecture.md)
- [제품 범위](docs/product-plan.md)
- [구현 로드맵](docs/roadmap.md)
- [기존 프로젝트와의 분리 경계](docs/project-separation.md)
- [개발용 임베딩 비교 도구](tools/reference/README.md)

## 저장소 구조

```text
apps/                   플랫폼별 앱의 예정 위치와 책임
crates/app-core/        최소 Rust 라이브러리 골격
contracts/              API·데이터 계약의 예정 위치
catalog/                검증된 모델 manifest의 예정 위치
docs/                   설계·제품 범위·로드맵
tools/reference/        Python 임베딩 비교 도구 (제품 의존성 아님)
```

## 개발 확인

Rust 2024 edition을 지원하는 toolchain으로 실행합니다.

```bash
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

첫 작업은 [N0 런타임 기술 검증](docs/roadmap.md)입니다. 골격의 빌드 성공은 실제 모델 실행이나 모바일 지원 완료를 의미하지 않습니다.

## 프로젝트 관계

[mj_llm_wapper](https://github.com/clevekim00/mj_llm_wapper)는 기존 로컬 LLM 게이트웨이입니다.
`mj-llm`은 별도 Git 저장소와 출시 주기를 가지며, 현재 기존 프로젝트 코드에 런타임 의존성을 두지 않습니다.

## 라이선스

[MIT](LICENSE). 모델 가중치·SDK·외부 라이브러리는 각각의 라이선스를 따릅니다.
