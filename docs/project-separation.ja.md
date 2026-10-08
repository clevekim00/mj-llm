# 既存プロジェクトとの分離

[한국어](project-separation.md) | [English](project-separation.en.md) | [日本語](project-separation.ja.md)

> 2026-10-08の原文に対応する文書翻訳です。コマンド・固定ID・実装/検証状態を維持しています。

2026-10-08、mj_llm_wapper作業領域のnativeマルチモーダル設計をmj-llmへ分離しました。

- 既存：https://github.com/clevekim00/mj_llm_wapper
- 新規：https://github.com/clevekim00/mj-llm
- 継承：最新native設計の第5章、開発用EmbeddingGemma 2比較スクリプト。
- 非継承：Ollamaゲートウェイ実装、Git履歴、対話データ、モデル重み、cache・ビルド成果物。
- 元リポジトリのファイル・未commit変更は維持しました。

migration表の`src/api.rs`、`src/runtime.rs`、`src/store.rs`、`catalog/models.json`などは**既存mj_llm_wapperのパス**です。新リポジトリに存在するという意味ではありません。既存コードを参考に、新コア・アプリはこちらで実装します。両リポジトリを結ぶ共通パッケージはまだありません。

architecture.mdのPython APIは比較用契約への言及です。本リポジトリに公開HTTP APIはまだありません。今後の設計原本はこちらの`docs/architecture.md`です。

## 2026-10-08 進行中作業の継承記録

ユーザーが`mj_llm_wrappper`と呼んだ実際のローカルリポジトリは`/Users/youngwhankim/Project/mj_llm_wapper`です。今回の補完は元HEAD `a23ce0ac2fd0fdc79f40e5fe79891e8f40c65174`と**未commit作業ファイル**を併せて確認したものです。HEADだけでは進行中の文書・embedding作業を再現できないため、主要企画原本のSHA-256を記録します。元リポジトリは変更していません。

| 原本 | 反映した内容 | 移行先 |
|---|---|---|
| `README.md`、`blueprint-local-llm-hub.md`の実装基準 | Ollama開発版とnative目標の区別 | README・ロードマップの状態表示 |
| `product-plan-local-llm-hub.md` | ユーザー、モデルセンター・機器診断・対話・RAG、プラットフォーム方針・拡張backlog | `product-plan.md` |
| `blueprint-local-llm-hub.md`第5章 | 共通Rustコア、LiteRT-LM、media・index・FFI・gate | architectureの既存設計とプラットフォーム補完 |
| `mj_llm_wrapper_request.md` | Narmer用stream/non-stream・構造化出力・provenance・エラー契約 | 設計9章、N4 API検証 |
| `src/device.rs`、`src/domain.rs` | 機器情報と文字列対話モデル | 機器診断再設計・JSON migration要件 |
| `src/runtime.rs`、`src/api.rs`、`src/store.rs` | runtime trait・route・JSON保存実装の存在確認 | 再利用契約と新実装境界の区別 |
| `src/embeddings.rs` | 任意の外部ローカルembeddingサービスadapterの存在確認 | 開発比較と組み込みruntimeの分離 |

原本作業ファイルのhash：

```text
c667bf2eb028f83ad19c6d150d22455c59f9815933a54ea5a83432beeb0a168a  README.md
6602cc293e96428b1a458ff0a721979c2886f2e1375abdf7acac34fc4ca6f434  product-plan-local-llm-hub.md
6a7bfd58452bbfe8259f135132bbea2c1b54f150be7ca9dd9ce2b328625f4a37  blueprint-local-llm-hub.md
53ad2eed795f9e7fedbd62ecbf1486a7e94c28bb9c59f95d93d888c7ce080c22  mj_llm_wrapper_request.md
```

今回は企画・設計の継承であり、runtimeコードの複製やGit履歴のマージではありません。元のテスト・モデル実行を再実行していないため、元文書の成功を新製品の検証結果として扱いません。

## 継承・再設計・保留

| 分類 | 内容 | 理由 |
|---|---|---|
| 継承 | 機器に基づく推薦、モデル導入、streaming対話、ローカル優先、runtime抽象化 | 独立製品にも必要な体験・契約 |
| 再設計 | Ollama pull → ModelStore、JSON → SQLite、Python embedding → 組み込みSDK、PWA → OS別アプリ | 別途インストールなしの全プラットフォーム実行 |
| N1必須 | ユーザーが選んだ既存JSON対話import | 履歴保存、自動データ収集禁止 |
| N4保留 | Narmer API互換・MCP・信頼ノード・モデル拡大・移行 | 初期のローカル検索・対話とは別検証 |
| 拡張backlog | Model Lab/Project Arena・Quick RAG・セッションprofile・agent routing・Portable Workspace | アイデアを保ちN0の前提作業を膨らませない |
| 除外 | Ollama/Python製品依存、個人対話・重み・cacheの自動コピー | 独立実行・データ境界を維持 |

今後も元リポジトリと自動同期しません。追加継承時には原本状態と要件を再比較し、本リポジトリの企画・設計・実装状態をまとめて更新します。
