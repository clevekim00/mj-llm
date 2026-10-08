# 모델 카탈로그

검증된 native artifact manifest를 추가할 위치입니다. 현재 설치 가능한 모델을 선언하지 않습니다.

`n0.lock.json`은 개발용 feasibility probe의 고정 입력입니다. LiteRT-LM 0.18.0과
EmbeddingGemma 2 text-vision, Qwen3-0.6B의 revision/hash를 고정합니다.
제품용 Verified 카탈로그·설치 지원 목록은 아닙니다. 현재 실제 실행 결과는 macOS CPU뿐입니다.

첫 후보는 EmbeddingGemma 2 text-vision과 소형 생성 모델입니다.
정확한 runtime/SDK, artifact revision, 파일 크기·SHA-256, 라이선스,
플랫폼·가속기·전처리 profile, 실기기 근거를 N0에서 고정합니다.
