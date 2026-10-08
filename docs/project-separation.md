# 기존 프로젝트와의 분리

[한국어](project-separation.md) | [English](project-separation.en.md) | [日本語](project-separation.ja.md)

2026-10-08, mj_llm_wapper 작업공간에서 작성한 native 멀티모달 설계를 mj-llm으로 분리했다.

- 기존 프로젝트: https://github.com/clevekim00/mj_llm_wapper
- 새 프로젝트: https://github.com/clevekim00/mj-llm
- 가져온 자료: 최신 native 설계 5장, 개발용 EmbeddingGemma 2 비교 스크립트
- 가져오지 않은 것: Ollama 게이트웨이 구현, 기존 Git 이력, 대화 데이터, 모델 가중치, 캐시·빌드 산출물
- 기존 저장소의 파일·미커밋 변경은 그대로 유지한다.

설계의 migration 표에 나타나는 src/api.rs, src/runtime.rs, src/store.rs,
catalog/models.json 등은 **기존 mj_llm_wapper의 경로**이며 새 저장소에 존재한다고 뜻하지 않는다.
기존 코드는 참고하고 새 코어·앱은 새 저장소에서 구현한다. 두 저장소를 연결하는 공통 패키지는 아직 없다.

architecture.md의 기존 Python API 언급은 비교 대상의 계약이다. 이 저장소에 공개 HTTP API는 아직 없다.
후속 개발의 설계 원본은 이 저장소의 docs/architecture.md로 관리한다.

## 2026-10-08 진행 작업 계승 기록

사용자가 지칭한 `mj_llm_wrappper`의 실제 로컬 저장소는 `/Users/youngwhankim/Project/mj_llm_wapper`다. 이번 문서 보완은 원본 HEAD `a23ce0ac2fd0fdc79f40e5fe79891e8f40c65174`와 **미커밋 작업 파일**을 함께 읽은 결과다. HEAD만으로 진행 중인 문서·임베딩 작업을 재현할 수 없으므로 주요 기획 원본의 SHA-256을 아래에 기록한다. 원본 저장소는 변경하지 않았다.

| 원본 | 반영한 내용 | 이 저장소의 목적지 |
|---|---|---|
| `README.md`, `blueprint-local-llm-hub.md`의 구현 기준선 | Ollama 개발판과 native 목표의 구분 | README·로드맵의 상태 표시 |
| `product-plan-local-llm-hub.md` | 사용자, 모델 센터·기기 진단·대화·RAG, 플랫폼 정책·확장 backlog | `product-plan.md` |
| `blueprint-local-llm-hub.md` 5장 | Rust 공통 코어, LiteRT-LM, 미디어·인덱스·FFI·검증 gate | `architecture.md` 기존 설계 및 플랫폼 보완 |
| `mj_llm_wrapper_request.md` | Narmer용 stream/non-stream·구조화 출력·provenance·오류 계약 | 아키텍처 9장, N4 API 검증 |
| `src/device.rs`, `src/domain.rs` | 기기 정보와 문자열 대화 모델 | 기기 진단 재설계·JSON migration 요구 |
| `src/runtime.rs`, `src/api.rs`, `src/store.rs` | runtime trait, API route, JSON 저장 구현 존재 확인 | 재사용할 계약과 새로 구현할 경계 구분 |
| `src/embeddings.rs` | 선택형 외부 로컬 embedding 서비스 adapter 존재 확인 | 개발 비교용 경로와 내장 runtime 분리 |

문서 원본의 작업 파일 해시:

```text
c667bf2eb028f83ad19c6d150d22455c59f9815933a54ea5a83432beeb0a168a  README.md
6602cc293e96428b1a458ff0a721979c2886f2e1375abdf7acac34fc4ca6f434  product-plan-local-llm-hub.md
6a7bfd58452bbfe8259f135132bbea2c1b54f150be7ca9dd9ce2b328625f4a37  blueprint-local-llm-hub.md
53ad2eed795f9e7fedbd62ecbf1486a7e94c28bb9c59f95d93d888c7ce080c22  mj_llm_wrapper_request.md
```

이번 계승은 기획·설계 반영이며 기존 runtime 코드를 복제하거나 Git 이력을 병합한 작업이 아니다. 원본의 테스트와 모델 실행을 이번 작업에서 재실행하지 않았으므로, 원본 문서의 성공 기록을 새 제품의 검증 결과로 표시하지 않는다.

## 계승·재설계·보류 결정

| 분류 | 내용 | 이유 |
|---|---|---|
| 계승 | 기기 기반 추천, 모델 설치 흐름, 스트리밍 대화, 로컬 우선, runtime 추상화 | 독립 제품에서도 필요한 사용자 경험과 계약 |
| 재설계 | Ollama pull → 자체 ModelStore; JSON → SQLite; Python embedding → 내장 SDK; PWA → OS별 앱 | 별도 설치 없는 전체 플랫폼 실행 요구 |
| N1 필수 | 사용자가 선택한 기존 JSON 대화 import | 기존 사용 기록 보존; 자동 데이터 수집 금지 |
| N4 보류 | Narmer API 호환·MCP·신뢰 노드·모델군 확대·프로젝트 이동 | 첫 출시의 로컬 검색·대화와 분리 검증 |
| 확장 backlog | Model Lab/Project Arena·Quick RAG·세션 프로필·agent routing·Portable Workspace | 기존 아이디어를 보존하되 N0 선행 작업을 비대하게 만들지 않음 |
| 제외 | Ollama/Python 제품 의존성, 개인 대화·가중치·cache 자동 복사 | 독립 실행·데이터 경계 유지 |

이후 원본 저장소가 바뀌어도 자동 동기화하지 않는다. 추가로 계승할 때 원본 상태와 요구사항을 다시 비교하며 이 저장소의 기획·설계·구현 상태를 함께 갱신한다.
