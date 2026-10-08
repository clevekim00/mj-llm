# 기존 프로젝트와의 분리

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
