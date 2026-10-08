# プラットフォーム別アプリ

[한국어](README.md) | [English](README.en.md) | [日本語](README.ja.md)

> 2026-10-08の原文に対応する文書翻訳です。コマンド・固定ID・実装/検証状態を維持しています。

実行可能なユーザーアプリはまだありません。N0検証後に必要なプラットフォームから実装します。

- desktop：Windows・macOS・Linux、既存Web技術を使うPC shell候補を検証
- android：スマートフォン・タブレット、Compose UI + native runtime bridge
- ios：iPhone・iPad、SwiftUI + native runtime bridge

モバイルはlocalhost HTTPサーバーなしで共通Rust AppServiceを呼ぶ設計です。
