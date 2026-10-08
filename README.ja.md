# mj-llm

[한국어](README.md) | [English](README.en.md) | [日本語](README.ja.md)

> 2026-10-08の原文に対応する文書翻訳です。コマンド・固定ID・実装/検証状態を維持しています。

Windows・macOS・Linux・Android・iOS/iPadOSで直接動作するローカルマルチモーダルAIプロジェクトです。OllamaやPythonの別途インストールを必要としない製品を目指しています。

> 現在の状態：N0 macOS CPU検証ツールを実装済み。実モデルによるテキスト・画像の埋め込みと小型モデルの生成テストに合格しました。ユーザー向けアプリと他プラットフォームのnative adapterは未実装です。

## 製品の方向性

- 共通Rustコアと組み込みLiteRT-LMを優先して検証します。
- 検索用EmbeddingGemma 2と回答生成モデルを分離します。
- 初回リリース：文書・写真検索と根拠に基づく対話。
- 将来：音声・動画の区間検索、追加モデル、ツール連携。
- モデルの初回準備後はローカル実行が基本で、自動クラウド送信は行いません。

## ドキュメント

文書は韓国語・英語・日本語で提供し、各文書上部で切り替えられます。文書翻訳はモデルの多言語品質検証を意味しません。N0の固定韓国語入力は維持します。

- [図でわかる使い方ガイド](docs/user-guide.ja.html)
- [アーキテクチャ設計](docs/architecture.ja.md)
- [製品企画・プラットフォーム範囲](docs/product-plan.ja.md)
- [実装ロードマップ](docs/roadmap.ja.md)
- [既存プロジェクトの継承記録と分離境界](docs/project-separation.ja.md)
- [開発用埋め込み比較ツール](tools/reference/README.ja.md)
- [N0の実行・テスト方法と検証結果](docs/n0-feasibility.ja.md)

## リポジトリ構成

```text
apps/                   各プラットフォームのアプリ予定位置と責務
crates/app-core/        モデル整合性・前処理profile・embedding-space検証
crates/runtime-litert/  狭いC ABIとmacOS CPU native adapter
crates/n0-probe/        実際のload/embed/generate/unload検証CLI
contracts/              API・データ契約の予定位置
catalog/                検証済みモデルmanifestの予定位置
catalog/n0.lock.json     N0 SDK・artifact revision/hash（製品カタログではない）
docs/                   設計・製品範囲・ロードマップ
tools/reference/        Python埋め込み比較ツール（製品依存ではない）
```

## 開発時の確認

Rust 2024 edition対応toolchainを使用します。

```bash
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

現在の作業は[N0ランタイム技術検証](docs/roadmap.ja.md)です。SDKなしの共通テストはnative検証の代わりにはなりません。実モデルのテストは[N0案内](docs/n0-feasibility.ja.md)の明示的なコマンドで実行します。

```bash
cargo run -p mj-llm-n0 --locked -- pin
cargo run -p mj-llm-n0 --release --locked -- verify /absolute/path/to/embeddinggemma-2-text-vision-440m.litertlm
```

CLIは自動ダウンロードや外部推論を行いません。既定のビルドはnative機能が無効というエラーを返し、偽のベクトルを成功結果として返しません。

N1はPCプレビュー、N2は全必須プラットフォーム共通の初回正式リリースです。スマートフォン・タブレットを含む実機検証が必要です。最低OS・機器仕様はN0の実測で決定します。ブラウザー単独実行は将来候補です。

## 関連プロジェクト

[mj_llm_wapper](https://github.com/clevekim00/mj_llm_wapper)は既存のローカルLLMゲートウェイです。`mj-llm`は独立したGitリポジトリとリリースサイクルを持ち、現時点では既存コードへのランタイム依存はありません。

## ライセンス

[MIT](LICENSE)。モデルの重み・SDK・外部ライブラリにはそれぞれのライセンスが適用されます。
