# mj-llm 実装ロードマップ

[한국어](roadmap.md) | [English](roadmap.en.md) | [日本語](roadmap.ja.md)

> 2026-10-08の原文に対応する文書翻訳です。コマンド・固定ID・実装/検証状態を維持しています。

未チェックの項目は未実装です。

- [x] 独立リポジトリ、設計書、Rust workspaceの骨格
- [x] N0の一部：SDK/artifact固定、macOS CPU load・text/image embed・generate・unload probeと自動テスト
- [ ] N0：LiteRT-LM SDK/artifact固定、PC 3 OS・Android・iOSのload・text/image embedding feasibility
- [ ] N0：公式文書間のprefix差異と前処理の整合性をgolden fixtureで検証
- [ ] N1：モデルのダウンロード・検証・import、runtime registry、メモリ割り当て判定
- [ ] N1：PCの文書・写真索引、検索、根拠付き対話、SQLite保存・復旧
- [ ] N2：Android/iOS/iPadOSアプリ、スマートフォン・タブレット、権限・background・発熱・強制終了からの復旧、初回共通リリース
- [ ] N3：音声・動画の区間索引、検索・再生位置の引用
- [ ] N4：追加モデル・adapter、任意の開発者API、MCP、信頼ノード、プロジェクト移行

## 最初の実装作業

N0で各プラットフォームのSDK・ビルド・実機と正確なartifact revisionを選びます。既存のPython埋め込み比較ツールをnative対応の証拠にはしません。PC機能を大きく実装する前にAndroid/iOSの実現可能性を確認します。

最初の実験はload → text/image embedding → unloadです。続いて小型生成モデルのload・最初のtoken・完了・キャンセルと、2モデルの順次切り替えを確認します。Rustの骨格拡張やPC UI実装をこの判断より先行させません。

## 段階別の成果物と終了条件

| 段階 | 成果物 | 終了条件 | 現在の状態 |
|---|---|---|---|
| N0 | SDK/artifact lock、各プラットフォームPoC、golden fixture、基準機器・OS・RAM表 | Windows x64/macOS arm64/Linux x64/Android arm64/iOS arm64でload・embed・generateと資源測定、阻害要因解消 | 進行中：macOS CPU smoke合格。他プラットフォーム・キャンセル・品質gateは未完了 |
| N1 | 共通コア、モデルストア、PCアプリ、索引・検索・対話、JSON import | PC 3 OSのインストール・offline E2E、該当G1～G10合格、PCプレビュー | 未実装 |
| N2 | Android・iOS/iPadOSアプリと配布パッケージ | Androidスマートフォン・タブレット/iPhone/iPad実機E2EとPC回帰に合格、初回全プラットフォームリリース | 未実装 |
| N3 | audio/video decoder、区間検索・引用 | modality別品質、時間位置、長時間キャンセル・復旧gate | 未実装 |
| N4 | モデル拡張、API conformance、MCP・信頼ノード・移行 | 機能別セキュリティ・互換性gate、自動外部送信なし | 未実装 |

SDK/artifact revision/hashは推測せず、N0結果で固定します。各結果にソースcommit・ビルドツール・機器・OS・backend・fixture・測定値・再現コマンドを記録します。あるOSの成功を別OSの結果として複製しません。

## リリース判断

- 全必須プラットフォームで共通機能が合格して初めて全プラットフォーム対応完了です。PCだけならPCプレビューと表示します。
- SDK非対応やメモリ不足は`blocked`として残します。組み込み代替adapterを検討し、必須OSの削除や自動リモート実行で成功扱いにしません。
- Windows ARM64/macOS Intel/Linux arm64などは別途検証後に正式対応へ昇格します。
- ブラウザー単独実行は将来候補です。必須になればN0にJS/WASM・保存容量制限・WebGPU・offline検証を追加し、N2 gateも更新します。

## 継承しつつ後続に残す作業

Model Lab/Project Arena、Quick RAG、セッション品質profile、対話分岐、複数protocol facade、Portable Workspace、モデル別agent routingは拡張backlogです。N0をこれらの実装まで広げません。[製品企画](product-plan.ja.md)のP01～P10と[継承記録](project-separation.ja.md)を基準に優先順位を管理します。

## 初期確認結果

共通コアにモデルSHA-256検査・profile・ベクトル検証を実装し、別crateからmacOS C ABIを接続しました。CLIで実embedding/generationを実行できます。一般テスト12件と実モデル統合テスト3件がローカルで合格し、fmt・Clippyも合格しました。Windows/Linux/macOS共通CI定義は追加しましたが、リモート実行結果は未確認です。

ユーザーアプリ・索引・検索ストアはまだありません。CLIの成功はN0全体やPC/モバイル製品の完成を意味しません。[実行方法・実測・残項目](n0-feasibility.ja.md)を基準に進めます。
