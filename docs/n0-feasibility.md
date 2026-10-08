# N0 네이티브 검증 구현·실행 기록

[한국어](n0-feasibility.md) | [English](n0-feasibility.en.md) | [日本語](n0-feasibility.ja.md)

2026-10-08. 제품 전체가 아닌 첫 native feasibility 구현이다. Ollama/Python 프로세스 없이 Rust CLI → 자체 C bridge → 공식 LiteRT-LM C API를 호출한다. 자동 다운로드·사용자 자료 전송 코드는 없다.

## 구현 범위

| 위치 | 실제 구현 |
|---|---|
| `catalog/n0.lock.json` | SDK commit, macOS 라이브러리·헤더·zip hash, 검색/생성 모델 revision·크기·SHA-256 |
| `crates/app-core` | 스트리밍 SHA-256 검증, 모델별 proof 타입, 고정 text prefix, 벡터 차원·finite·norm 검증, 다른 space 비교 거부 |
| `crates/runtime-litert` | thread-confined safe wrapper, C++ 예외 차단 C ABI, 입력/출력 복사, RAII 해제, CPU embedding·동기 generation |
| `crates/n0-probe` | `pin`, `verify`, `probe`, `generate-probe`; JSON 결과와 실패 exit code |
| `fixtures/n0` | 직접 만든 64×64 빨간 PNG; 한국어 positive/negative 문장은 probe 소스에 고정 |
| `scripts/test-n0-macos.sh` | native 빌드·일반 테스트·실모델 테스트·Clippy 재현 |
| `.github/workflows/rust.yml` | Windows/Linux/macOS 공통 Rust 검사 정의; native/device 시험은 아님 |

공통 코어는 `unsafe_code=forbid`다. unsafe는 `runtime-litert/src/native.rs`에 한정하며 ABI 호출마다 소유권·버퍼 수명 근거를 적었다. bridge는 C++ exception을 상태 코드로 바꾸고 외국 allocator의 메모리를 Rust에서 해제하지 않는다. 엔진은 `!Send/!Sync`, 추론은 `&mut self`를 요구한다. 동기 호출 종료 전에 unload할 수 없으며 callback은 아직 사용하지 않는다.

SDK·가중치는 저장소에 포함하지 않는다. 빌드 시 SDK 라이브러리와 사용 헤더를 해시 검증하고, 실행 전 모델 크기·해시를 검증한다. native load 직전에도 모델을 재검증한다. N0는 개발자가 소유한 **변경하지 않는 로컬 파일**을 전제로 하며, 다른 프로세스의 검증 후 파일 교체까지 막는 제품 ModelStore는 미구현이다. 빌드 이후 SDK 파일도 교체하지 않는다.

## 고정 입력과 출처

- LiteRT-LM `0.18.0`, commit `b2f686e2ed4718fb84ec398a61dd59ca0f0aff27`: [공식 패키지와 zip checksum](https://github.com/google-ai-edge/LiteRT-LM/blob/b2f686e2ed4718fb84ec398a61dd59ca0f0aff27/Package.swift).
- 검색: [고정 EmbeddingGemma 2 text-vision 배포](https://huggingface.co/litert-community/embeddinggemma-2-text-vision-440m-litert-lm/tree/e301f74d5551b0c2641bd5cb4652a76239d5c5f8), 387,710,976 bytes. SHA-256은 lock에 기록했다.
- 생성: [고정 Qwen3-0.6B 배포](https://huggingface.co/litert-community/Qwen3-0.6B/tree/a3c5d805ae362dff7f580bc25f2dfb9a5a7eaa76), 614,236,160 bytes. 첫 제품 모델 확정이 아닌 작은 N0 생성 후보다.
- 모델·SDK 라이선스는 각 배포 원본을 따른다. 이 저장소의 MIT가 가중치 라이선스를 대체하지 않는다.

검색 profile은 CPU float32, 2 threads, text signature 128~512, vision 70 tokens/image, SDK special tokens, L2 normalize, output 768, overflow error다. SDK가 이미지를 decode/resize한다. N0의 합성 PNG에는 EXIF가 없다. 제품 EXIF·픽셀 상한·권한 asset adapter는 아직 미구현이므로 사용자 이미지 수집 기능으로 사용하지 않는다.

prefix는 `task: search query | text: `와 `task: search result | text: `를 사용하는 **실험 profile**이다. 문자열 회귀 테스트와 한국어 smoke 순위는 통과했지만 모델 카드의 다른 prefix와 비교하는 품질 corpus 검증은 남았다. 프로덕션 검색 최적값으로 확정하지 않는다. SDK/model/profile·OS/architecture 변경은 embedding-space ID를 바꾸며 생성 모델의 변경만으로 검색 space를 바꾸지는 않는다.

생성은 CPU, context 512, output 최대 32 tokens, top-p sampler(k=1, p=1, temperature=1, seed=0), artifact 내장 template, `enable_thinking=false` 요청을 사용한다. SDK CPU sampler에서 GREEDY와 TOP_K는 실제로 `UNIMPLEMENTED`를 반환해 [고정 소스의 지원 경로](https://github.com/google-ai-edge/LiteRT-LM/blob/b2f686e2ed4718fb84ec398a61dd59ca0f0aff27/runtime/components/sampler_factory.cc)를 확인하고 TOP_P로 수정했다. 응답의 빈 think tag는 제거하지 않고 raw SDK 결과로 기록한다.

## 준비·실행

개발 도구는 Rust 1.95.0, macOS C++ compiler/Xcode command-line tools를 사용했다. 제품 사용자에게 요구하는 의존성이 아니다. SDK가 없는 기본 Rust 빌드는 네이티브 호출을 `Unavailable`로 거부한다.

저장소 루트에서 아래 파일을 받는다. 약 1.05GB 다운로드와 추론 메모리가 필요하다. 다운로드를 원치 않으면 동일 hash의 기존 파일을 직접 지정한다.

```bash
mkdir -p downloads/n0 models .mj-llm/n0
curl -fL https://github.com/google-ai-edge/LiteRT-LM/releases/download/v0.18.0/CLiteRTLM_mac.xcframework.zip -o downloads/n0/sdk.zip
shasum -a 256 downloads/n0/sdk.zip
# 예상: 5f6ee68d95eeccb084c6e66d5ee47255e3020fa0fb29696dd0301ae26d6cfb4f
unzip -q downloads/n0/sdk.zip -d downloads/n0/sdk
curl -fL https://huggingface.co/litert-community/embeddinggemma-2-text-vision-440m-litert-lm/resolve/e301f74d5551b0c2641bd5cb4652a76239d5c5f8/embeddinggemma-2-text-vision-440m.litertlm -o models/embeddinggemma-2-text-vision-440m.litertlm
curl -fL https://huggingface.co/litert-community/Qwen3-0.6B/resolve/a3c5d805ae362dff7f580bc25f2dfb9a5a7eaa76/Qwen3-0.6B.litertlm -o models/Qwen3-0.6B.litertlm
```

zip hash가 다르면 압축을 풀지 않는다. native build는 압축 해제된 라이브러리와 헤더도 lock과 대조해 불일치 시 중단한다. 모델은 CLI가 해시를 검사한다.

```bash
cargo run -p mj-llm-n0 --locked -- pin
cargo run -p mj-llm-n0 --release --locked -- verify models/embeddinggemma-2-text-vision-440m.litertlm
bash scripts/test-n0-macos.sh downloads/n0/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64 models/embeddinggemma-2-text-vision-440m.litertlm models/Qwen3-0.6B.litertlm
```

CLI만 실행할 때:

```bash
export MJ_LITERT_SDK_DIR="$PWD/downloads/n0/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64"
export DYLD_LIBRARY_PATH="$MJ_LITERT_SDK_DIR"
cargo build -p mj-llm-n0 --release --features native-macos --locked
target/release/mj-llm-n0 probe models/embeddinggemma-2-text-vision-440m.litertlm .mj-llm/n0/embedding-cache
target/release/mj-llm-n0 generate-probe models/Qwen3-0.6B.litertlm .mj-llm/n0/generation-cache
```

stdout은 성공 JSON이고 stderr에는 SDK 진단이 추가될 수 있다. 실패는 exit 1이며 성공 JSON을 내지 않는다. `verify`는 무결성만 검사한다. `probe`는 2회 load → 한국어 text embedding 4회 + image embedding 1회 → unload를 실행한다. `generate-probe`는 고정 산술 질문의 비어 있지 않은 답변을 확인한다. 모델 준비 후 추론 경로에 네트워크·Python·Ollama는 필요 없다.

## 테스트와 결과

- 일반 테스트 12개: SHA-256 표준 벡터·변조·잘림, short read/Interrupted/실제 I/O 오류, 잘못된 모델, lock pin, vector dimension/NaN/Inf/zero norm, space 혼합 거부, cosine, prefix, CLI 오류, native 비활성 시 가짜 결과 방지.
- 실제 모델 테스트 3개: text/image·repeat·reload, 손상 이미지·1509-token overflow 오류 후 복구, 소형 모델 생성·해제. `#[ignore]`로 일반 CI와 분리하며 위 shell script가 명시적으로 실행한다. 환경 변수가 없으면 실행을 성공으로 건너뛰지 않고 실패한다.
- `cargo fmt --all -- --check`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings` 통과. native feature 포함 release 테스트와 Clippy도 통과.
- GitHub CI는 작성만 했으며 원격 실행·다른 OS 통과를 주장하지 않는다.

로컬 장비: MacBookPro17,1 (arm64), RAM 16GiB, macOS 27.0 (26A428). 검색 질의의 positive cosine은 약 0.893, negative는 약 0.665, 반복 cosine은 0.9999999999999999, 이미지 norm은 약 0.99999985였다. 생성 응답에는 `2 + 2 = 4`가 포함되었다. 샘플 한 개의 기능 확인이며 검색/답변 정확도 보장이 아니다.

[기계 판독 검증 기록](n0-results/macos-arm64-cpu.json)에 최종 소스 파일 hash·toolchain·SDK/artifact·결과를 기록했다. `time -l`로 두 probe를 순차 실행한 단회 관찰값은 다음과 같다. RSS와 footprint는 다른 OS 지표이므로 합산하지 않는다.

| probe | 최대 RSS (bytes) | peak memory footprint (bytes) |
|---|---:|---:|
| text/image embedding + reload | 654,360,576 | 311,657,576 |
| Qwen3 생성 | 1,700,397,056 | 1,072,269,616 |

처음 sandbox 안의 `time -l`은 자원 계측용 sysctl 권한 때문에 실패했고 추론 자체는 성공했다. 위 값은 계측 권한을 확보한 순차 재실행 결과다. 추론 검증 테스트는 sandbox 안에서도 별도로 통과했다.

`load_and_reverify_ms`에는 모델 해시 재검사 시간이 포함된다. `inference_ms`는 다섯 embedding 호출의 합이며 단일 질의 latency나 p95가 아니다. OS cache·개발 중 부하가 있는 단회 측정을 정식 성능 기준으로 사용하지 않는다. 원시 벡터·사용자 자료는 보고서에 저장하지 않는다.

## 플랫폼 상태와 다음 gate

| 조합 | 구현·검증 상태 |
|---|---|
| macOS arm64 CPU | native adapter 구현, 실제 embedding/generation smoke 통과; N0 전체 accepted는 아님 |
| macOS Intel | universal SDK에 slice 존재; 이 프로젝트의 실행 미검증 |
| Windows x64 / Linux x64 | 공통 Rust CI 정의만 있음; native adapter·실행 미검증 |
| Android arm64 | native host adapter·실기기 미검증 |
| iOS/iPadOS arm64 | native host adapter·실기기 미검증; 로컬 장치 목록의 iPad가 unavailable 상태여서 실기기 실행하지 않음 |

남은 N0: 다른 OS SDK/host bridge의 고정·실기기 실행, streaming 첫 토큰·취소·중복 terminal 방지, 자원·발열·메모리 pressure, 두 모델을 같은 앱에서 순차 교체하는 admission, prefix 대조 corpus와 검색 품질, 서명 패키지 설치. N1 UI·SQLite·자료 색인 구현은 이 gate와 분리해 관리한다. 현재 동기 probe는 강제 취소 API·deadline·GPU fallback을 제공하지 않는다.
