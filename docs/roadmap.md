# mj-llm 구현 로드맵

[한국어](roadmap.md) | [English](roadmap.en.md) | [日本語](roadmap.ja.md)

체크되지 않은 항목은 미구현입니다.

- [x] 독립 저장소, 설계서, Rust workspace 골격 준비
- [x] N0 부분: SDK/artifact pin, macOS CPU load·text/image embed·generate·unload probe와 자동 테스트
- [ ] N0: LiteRT-LM SDK·artifact 고정, PC 3 OS/Android/iOS load·text/image embedding feasibility
- [ ] N0: 공식 문서 간 prefix 차이와 전처리 정합성을 golden fixture로 검증
- [ ] N1: 모델 다운로드·검증·import, runtime registry, 메모리 admission
- [ ] N1: PC 문서·사진 색인, 검색, 근거 대화, SQLite 저장·복구
- [ ] N2: Android/iOS/iPadOS 앱, 휴대폰·태블릿, 권한·background·열·강제 종료 복구, 첫 공통 출시
- [ ] N3: 음성·영상 구간 색인, 검색·재생 위치 인용
- [ ] N4: 추가 모델·adapter, 선택형 개발자 API, MCP, 신뢰 노드와 프로젝트 이동

## 첫 구현 작업

N0에서 플랫폼별 SDK/빌드/실기기와 정확한 artifact revision을 선정한다.
이미 사용 가능한 Python 임베딩 비교 도구를 native 지원 증거로 사용하지 않는다.
PC 기능을 크게 구현하기 전에 Android/iOS의 feasibility를 확인한다.

첫 실험은 load → text/image embedding → unload다. 이어서 소형 생성 모델의 load·첫 토큰·완료·취소와 두 모델의 순차 교체를 확인한다. Rust 골격 확장이나 PC UI 구현이 이 판단을 앞서지 않게 한다.

## 단계별 산출물과 종료 조건

| 단계 | 산출물 | 종료 조건 | 현재 상태 |
|---|---|---|---|
| N0 | SDK/artifact lock, 플랫폼별 PoC, golden fixture, 기준 기기·OS·RAM 표 | Windows x64/macOS arm64/Linux x64/Android arm64/iOS arm64에서 load·embed·generate 및 자원 측정; 차단 항목 해소 | 진행 중: macOS CPU smoke 통과; 나머지 플랫폼·취소·품질 gate 미완료 |
| N1 | 공통 코어, 모델 저장소, PC 앱, 색인·검색·대화, JSON import | PC 3 OS의 설치·offline E2E, G1~G10 해당 범위 통과; PC 미리보기 | 미구현 |
| N2 | Android·iOS/iPadOS 앱과 배포 패키지 | Android 휴대폰·태블릿/iPhone/iPad 실기기 E2E와 PC 회귀 통과; 첫 전체 플랫폼 출시 | 미구현 |
| N3 | audio/video decoder와 구간 검색·인용 | modality별 품질, 시간 위치, 장시간 취소·복구 gate | 미구현 |
| N4 | 모델 확장, API conformance, MCP·신뢰 노드·이동 | 기능별 보안·호환성 gate; 자동 외부 전송 없음 | 미구현 |

정확한 SDK·artifact revision/hash는 조사값을 추측해 적지 않고 N0 결과로 고정한다. 각 플랫폼 결과에는 소스 commit·빌드 도구·기기·OS·backend·fixture·측정치·재현 명령을 기록한다. 한 OS의 성공을 다른 OS 결과로 복제하지 않는다.

## 출시 판단

- 필수 플랫폼에서 공통 기능이 모두 통과해야 전체 플랫폼 지원 완료다. PC만 통과하면 PC 미리보기로 표시한다.
- SDK 미지원 또는 메모리 실패는 `blocked`로 남긴다. 내장 대체 adapter를 검토하되 필수 OS 제거·자동 원격 실행으로 성공 처리하지 않는다.
- Windows ARM64/macOS Intel/Linux arm64 등 추가 조합은 별도 검증 후 지원 표에 승격한다.
- 브라우저 단독 실행은 현재 후속 후보라는 가정이다. 필수로 변경되면 N0에 JS/WASM·저장 한도·WebGPU·오프라인 검증을 추가하고 N2 gate도 갱신한다.

## 계승하되 후속으로 남긴 작업

기존 기획의 Model Lab/Project Arena, Quick RAG, 세션 품질 프로필, 대화 분기, 다중 protocol facade, Portable Workspace, 모델별 agent routing은 확장 backlog다. N0를 이 기능들의 구현 작업으로 확대하지 않는다. [제품 기획서](product-plan.md)의 P01~P10과 [계승 기록](project-separation.md)을 기준으로 우선순위를 관리한다.

## 초기 확인 결과

공통 코어에 모델 SHA-256 검사·profile·벡터 검증을 구현했고 별도 crate에서 macOS C ABI를 연결했다. CLI로 실제 embedding/generation을 실행할 수 있다. 일반 테스트 12개와 실제 모델 통합 테스트 3개를 로컬에서 통과했고 fmt·Clippy도 통과했다. Windows/Linux/macOS 공통 CI 정의를 추가했으나 원격 실행 결과는 아직 없다.

사용자 앱·색인·검색 저장소는 아직 없다. 현재 CLI의 성공은 전체 N0 종료나 PC/모바일 제품 완성을 의미하지 않는다. [실행 방법·실측·남은 항목](n0-feasibility.md)을 기준으로 다음 작업을 진행한다.
