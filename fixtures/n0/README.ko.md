# N0 합성 fixture

[한국어](README.ko.md) | [English](README.en.md) | [日本語](README.ja.md)

`red.png`는 프로젝트에서 직접 만든 64×64 RGB PNG이며 모든 픽셀은 `(255, 0, 0)`입니다. 사용자 사진이나 제3자 이미지는 포함하지 않습니다. 이미지 디코딩과 유한한 정규화 임베딩 출력을 검사하며 의미 기반 이미지 검색 품질을 평가하지 않습니다.

고정 한국어 질의와 positive/negative 문장은 `crates/n0-probe/src/lib.rs`에 있습니다. 반복 cosine 하한은 0.9999, 정규화 벡터 허용 오차는 0.001입니다. 이 smoke 검사는 G4/G5 corpus를 대체하지 않습니다.

생성 fixture는 thinking 비활성, 출력 32 tokens, context 512 tokens, top-p sampler(k=1 / p=1 / temperature=1 / seed=0)로 `2 + 2`를 질문합니다. 비어 있지 않은 구조화 SDK 출력을 검사하며 일반 답변 품질이나 streaming/cancellation을 평가하지 않습니다.
