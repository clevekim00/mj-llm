# N0 native検証の実装・実行記録

[한국어](n0-feasibility.md) | [English](n0-feasibility.en.md) | [日本語](n0-feasibility.ja.md)

> 2026-10-08の原文に対応する文書翻訳です。コマンド・固定ID・実装/検証状態を維持しています。

2026-10-08。製品全体ではなく、最初のnative実現可能性検証です。Ollama/PythonプロセスなしでRust CLI → 独自C bridge → 公式LiteRT-LM C APIを呼びます。自動ダウンロード・ユーザー資料送信のコードはありません。

## 実装範囲

| 場所 | 実装内容 |
|---|---|
| `catalog/n0.lock.json` | SDK commit、macOS library・header・zip hash、検索/生成モデルrevision・容量・SHA-256 |
| `crates/app-core` | streaming SHA-256検証、モデル別proof型、固定text prefix、ベクトル次元・finite・norm検証、異なるspace比較の拒否 |
| `crates/runtime-litert` | thread限定safe wrapper、C++例外を封じるC ABI、入出力コピー、RAII解放、CPU embedding・同期generation |
| `crates/n0-probe` | `pin`・`verify`・`probe`・`generate-probe`、JSON結果と失敗exit code |
| `fixtures/n0` | 自作64×64赤PNG、韓国語positive/negative文はprobeソースに固定 |
| `scripts/test-n0-macos.sh` | nativeビルド・一般テスト・実モデルテスト・Clippy再現 |
| `.github/workflows/rust.yml` | Windows/Linux/macOS共通Rust検査定義。native/実機テストではない |

共通コアは`unsafe_code=forbid`です。unsafeは`runtime-litert/src/native.rs`に限定し、各ABI呼び出しに所有権・バッファ寿命の根拠を記載しています。bridgeはC++例外をstatus codeへ変換し、外部allocatorのメモリをRustで解放しません。エンジンは`!Send/!Sync`、推論には`&mut self`が必要です。同期呼び出し終了前にはunloadできず、callbackは未使用です。

SDK・重みはリポジトリに含めません。ビルド時にSDK library/headerのhash、実行前にモデル容量・hashを検証し、native load直前にも再検証します。N0は開発者所有の**変更しないローカルファイル**を前提とします。別プロセスによる検証後の差し替えを防ぐ製品ModelStoreは未実装です。ビルド後のSDKも差し替えないでください。

## 固定入力と出典

- LiteRT-LM `0.18.0`、commit `b2f686e2ed4718fb84ec398a61dd59ca0f0aff27`：[公式パッケージとzip checksum](https://github.com/google-ai-edge/LiteRT-LM/blob/b2f686e2ed4718fb84ec398a61dd59ca0f0aff27/Package.swift)。
- 検索：[固定EmbeddingGemma 2 text-vision配布](https://huggingface.co/litert-community/embeddinggemma-2-text-vision-440m-litert-lm/tree/e301f74d5551b0c2641bd5cb4652a76239d5c5f8)、387,710,976 bytes。SHA-256はlockに記録。
- 生成：[固定Qwen3-0.6B配布](https://huggingface.co/litert-community/Qwen3-0.6B/tree/a3c5d805ae362dff7f580bc25f2dfb9a5a7eaa76)、614,236,160 bytes。小型N0候補であり初回製品モデルの確定ではない。
- モデル・SDKには各配布元のライセンスが適用され、本リポジトリのMITは重みのライセンスを置き換えません。

検索profile：CPU float32、2 threads、text signature 128～512、vision 70 tokens/image、SDK特殊token、L2 normalize、出力768、overflow error。SDKが画像をdecode/resizeします。合成PNGにはEXIFがありません。製品のEXIF処理・画素上限・権限付きasset adapterは未実装であり、ユーザー画像収集機能として使いません。

`task: search query | text: `と`task: search result | text: `は**実験profile**です。文字列回帰・韓国語smoke順位は合格しましたが、モデルカードの別prefixとの品質corpus比較は未完了です。本番検索の最適値と確定しません。SDK/model/profile・OS/architecture変更はembedding-space IDを変更しますが、生成モデルだけの変更では検索spaceを変えません。

生成はCPU、context 512、出力最大32 tokens、top-p sampler（k=1、p=1、temperature=1、seed=0）、artifact内蔵template、`enable_thinking=false`要求を使用します。SDK CPU samplerのGREEDY・TOP_Kは実際に`UNIMPLEMENTED`を返したため、[固定ソースの対応経路](https://github.com/google-ai-edge/LiteRT-LM/blob/b2f686e2ed4718fb84ec398a61dd59ca0f0aff27/runtime/components/sampler_factory.cc)を確認してTOP_Pに変更しました。空のthink tagは除去せずraw SDK結果として記録します。

## 準備・実行

開発にはRust 1.95.0、macOS C++ compiler/Xcode command-line toolsを使用しました。製品利用者向けの依存要件ではありません。SDKなしの既定Rustビルドはnative呼び出しを`Unavailable`として拒否します。

リポジトリrootから次のファイルを取得します。約1.05GBのダウンロードと推論用メモリが必要です。同一hashの既存ファイルを指定しても構いません。

```bash
mkdir -p downloads/n0 models .mj-llm/n0
curl -fL https://github.com/google-ai-edge/LiteRT-LM/releases/download/v0.18.0/CLiteRTLM_mac.xcframework.zip -o downloads/n0/sdk.zip
shasum -a 256 downloads/n0/sdk.zip
# 期待値: 5f6ee68d95eeccb084c6e66d5ee47255e3020fa0fb29696dd0301ae26d6cfb4f
unzip -q downloads/n0/sdk.zip -d downloads/n0/sdk
curl -fL https://huggingface.co/litert-community/embeddinggemma-2-text-vision-440m-litert-lm/resolve/e301f74d5551b0c2641bd5cb4652a76239d5c5f8/embeddinggemma-2-text-vision-440m.litertlm -o models/embeddinggemma-2-text-vision-440m.litertlm
curl -fL https://huggingface.co/litert-community/Qwen3-0.6B/resolve/a3c5d805ae362dff7f580bc25f2dfb9a5a7eaa76/Qwen3-0.6B.litertlm -o models/Qwen3-0.6B.litertlm
```

zip hashが異なる場合は展開しません。native buildは展開済みlibrary/headerもlockと照合し、不一致なら停止します。モデルはCLIが検証します。

```bash
cargo run -p mj-llm-n0 --locked -- pin
cargo run -p mj-llm-n0 --release --locked -- verify models/embeddinggemma-2-text-vision-440m.litertlm
bash scripts/test-n0-macos.sh downloads/n0/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64 models/embeddinggemma-2-text-vision-440m.litertlm models/Qwen3-0.6B.litertlm
```

CLIだけを実行する場合：

```bash
export MJ_LITERT_SDK_DIR="$PWD/downloads/n0/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64"
export DYLD_LIBRARY_PATH="$MJ_LITERT_SDK_DIR"
cargo build -p mj-llm-n0 --release --features native-macos --locked
target/release/mj-llm-n0 probe models/embeddinggemma-2-text-vision-440m.litertlm .mj-llm/n0/embedding-cache
target/release/mj-llm-n0 generate-probe models/Qwen3-0.6B.litertlm .mj-llm/n0/generation-cache
```

成功JSONはstdoutへ、SDK診断はstderrへ出る場合があります。失敗はexit 1で成功JSONを返しません。`verify`は整合性のみ検査します。`probe`は2回load、韓国語text embedding 4回とimage embedding 1回、unloadを実行します。`generate-probe`は固定算数質問への空でない回答を確認します。モデル準備後の推論にネットワーク・Python・Ollamaは不要です。

## テストと結果

- 一般テスト12件：SHA-256標準値・改ざん・切断、short read/Interrupted/実I/Oエラー、不正モデル、lock固定、次元/NaN/Inf/zero norm、space混在拒否、cosine、prefix、CLIエラー、native無効時の偽結果防止。
- 実モデルテスト3件：text/image・repeat・reload、破損画像・1509-token overflow後の復旧、小型モデル生成・解放。`#[ignore]`で通常CIと分離し、shell scriptで明示実行します。環境変数なしでは成功扱いのskipにせず失敗します。
- `cargo fmt --all -- --check`、`cargo test --workspace --locked`、`cargo clippy --workspace --all-targets --locked -- -D warnings`に合格。native feature付きreleaseテスト・Clippyも合格。
- GitHub CIは定義のみ。リモート実行・他OSの合格は主張しません。

ローカル機器：MacBookPro17,1（arm64）、RAM 16GiB、macOS 27.0（26A428）。検索positive cosineは約0.893、negativeは約0.665、反復cosineは0.9999999999999999、画像normは約0.99999985でした。生成には`2 + 2 = 4`が含まれました。1サンプルの機能確認であり検索・回答精度の保証ではありません。

[機械可読の検証記録](n0-results/macos-arm64-cpu.json)に最終ソースhash・toolchain・SDK/artifact・結果を記録しています。`time -l`でprobeを順次実行した単回の観察値です。RSSとfootprintは別指標なので合算しません。

| probe | 最大RSS（bytes） | peak memory footprint（bytes） |
|---|---:|---:|
| text/image embedding + reload | 654,360,576 | 311,657,576 |
| Qwen3生成 | 1,700,397,056 | 1,072,269,616 |

最初のsandbox内`time -l`は資源計測用sysctl権限で失敗しましたが、推論自体は成功しました。上記は計測権限を得た順次再実行の値です。推論テストはsandbox内でも別途合格しています。

`load_and_reverify_ms`にはモデルhash再検査時間が含まれます。`inference_ms`は5回のembedding合計で、単一質問の遅延やp95ではありません。OS cache・開発負荷のある単回測定を正式性能基準にしません。rawベクトル・ユーザー資料は報告書へ保存しません。

## プラットフォーム状態と次のgate

| 構成 | 実装・検証状態 |
|---|---|
| macOS arm64 CPU | native adapter実装、実embedding/generation smoke合格。N0全体acceptedではない |
| macOS Intel | universal SDKにsliceあり。本プロジェクトの実行は未検証 |
| Windows x64 / Linux x64 | 共通Rust CI定義のみ。native adapter・実行は未検証 |
| Android arm64 | native host adapter・実機未検証 |
| iOS/iPadOS arm64 | native host adapter・実機未検証。ローカルiPadがunavailableのため実機実行なし |

残るN0：他OSのSDK/host bridge固定と実機実行、streaming最初のtoken・取消・terminal重複防止、資源・発熱・memory pressure、同一アプリの順次モデル切り替えadmission、prefix比較corpus・検索品質、署名package導入。N1 UI・SQLite・資料索引とは分けて管理します。現在の同期probeに強制取消API・deadline・GPU fallbackはありません。
