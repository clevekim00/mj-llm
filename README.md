# mj-llm

[한국어](README.md) | [English](README.en.md) | [日本語](README.ja.md)

Windows·macOS·Linux·Android·iOS/iPadOS에서 직접 실행하는 로컬 멀티모달 AI 프로젝트입니다.
Ollama나 Python을 별도로 설치하지 않는 제품을 목표로 합니다.

> 현재 상태: N0 macOS CPU 검증 도구 구현. 실제 텍스트·이미지 임베딩과 소형 모델 생성 테스트를 통과했습니다. 사용자 앱과 다른 플랫폼의 native adapter는 아직 구현되지 않았습니다.

## 제품 방향

- 공통 Rust 코어 + 내장 LiteRT-LM을 우선 검증합니다.
- EmbeddingGemma 2 검색 모델과 답변 생성 모델을 분리합니다.
- 첫 출시: 문서·사진 검색과 근거 기반 대화.
- 후속: 음성·영상 구간 검색, 추가 모델과 도구 연결.
- 최초 모델 준비 후 로컬 실행이 기본이며 자동 클라우드 전송은 하지 않습니다.

## 문서

문서는 한국어·영어·일본어로 제공하며 각 문서 상단에서 전환할 수 있습니다. 문서 번역은 모델의 다국어 품질 검증을 뜻하지 않으며, N0의 고정 한국어 시험 입력은 그대로 유지합니다.

- [그림으로 보는 쉬운 사용 가이드](docs/user-guide.html)
- [아키텍처 설계](docs/architecture.md)
- [제품 기획서·플랫폼 범위](docs/product-plan.md)
- [구현 로드맵](docs/roadmap.md)
- [기존 프로젝트 계승 기록·분리 경계](docs/project-separation.md)
- [개발용 임베딩 비교 도구](tools/reference/README.md)
- [N0 실행·테스트 방법과 검증 결과](docs/n0-feasibility.md)

## 저장소 구조

```text
apps/                   플랫폼별 앱의 예정 위치와 책임
crates/app-core/        모델 무결성·전처리 profile·embedding-space 검증
crates/runtime-litert/  좁은 C ABI + macOS CPU native adapter
crates/n0-probe/        실제 load/embed/generate/unload 검증 CLI
contracts/              API·데이터 계약의 예정 위치
catalog/                검증된 모델 manifest의 예정 위치
catalog/n0.lock.json     N0 SDK·artifact revision/hash (제품 카탈로그 아님)
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

현재 작업은 [N0 런타임 기술 검증](docs/roadmap.md)입니다. SDK 없이 실행하는 공통 테스트는 native 검증을 대신하지 않습니다. 실제 모델 테스트는 [N0 안내](docs/n0-feasibility.md)의 명시적 실행 명령으로 수행합니다.

```bash
cargo run -p mj-llm-n0 --locked -- pin
cargo run -p mj-llm-n0 --release --locked -- verify /absolute/path/to/embeddinggemma-2-text-vision-440m.litertlm
```

CLI는 자동 다운로드·외부 추론을 하지 않습니다. 기본 빌드는 native 미활성 오류를 반환하며 가짜 벡터로 성공 처리하지 않습니다.

N1은 PC 미리보기, N2는 모든 필수 플랫폼의 공통 정식 출시입니다. 휴대폰·태블릿을 포함한 실기기 검증을 요구하며, 최소 OS와 기기 사양은 N0 실측으로 확정합니다. 브라우저 단독 실행은 현재 후속 후보로 둡니다.

## 프로젝트 관계

[mj_llm_wapper](https://github.com/clevekim00/mj_llm_wapper)는 기존 로컬 LLM 게이트웨이입니다.
`mj-llm`은 별도 Git 저장소와 출시 주기를 가지며, 현재 기존 프로젝트 코드에 런타임 의존성을 두지 않습니다.

## 라이선스

[MIT](LICENSE). 모델 가중치·SDK·외부 라이브러리는 각각의 라이선스를 따릅니다.
