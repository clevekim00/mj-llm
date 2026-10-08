# mj-llm 구현 로드맵

체크되지 않은 항목은 미구현입니다.

- [x] 독립 저장소, 설계서, Rust workspace 골격 준비
- [ ] N0: LiteRT-LM SDK·artifact 고정, PC 3 OS/Android/iOS load·text/image embedding feasibility
- [ ] N0: 공식 문서 간 prefix 차이와 전처리 정합성을 golden fixture로 검증
- [ ] N1: 모델 다운로드·검증·import, runtime registry, 메모리 admission
- [ ] N1: PC 문서·사진 색인, 검색, 근거 대화, SQLite 저장·복구
- [ ] N2: Android/iOS 앱, 권한·background·열·강제 종료 복구, 첫 공통 출시
- [ ] N3: 음성·영상 구간 색인, 검색·재생 위치 인용
- [ ] N4: 추가 모델·adapter, MCP, 신뢰 노드와 프로젝트 이동

## 첫 구현 작업

N0에서 플랫폼별 SDK/빌드/실기기와 정확한 artifact revision을 선정한다.
이미 사용 가능한 Python 임베딩 비교 도구를 native 지원 증거로 사용하지 않는다.
PC 기능을 크게 구현하기 전에 Android/iOS의 feasibility를 확인한다.

## 초기 확인 결과

Rust workspace는 외부 crate 의존성 없는 골격이다. 빌드·포맷·Clippy 검사를 제공한다.
모델 런타임과 앱을 아직 연결하지 않았으므로 현재 실행 가능한 AI 제품은 아니다.
