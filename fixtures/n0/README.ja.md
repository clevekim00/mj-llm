# N0 合成fixture

[한국어](README.ko.md) | [English](README.en.md) | [日本語](README.ja.md)

> 2026-10-08の原文に対応する文書翻訳です。コマンド・固定ID・実装/検証状態を維持しています。

`red.png`はプロジェクトで作成した64×64 RGB PNGで、全画素が`(255, 0, 0)`です。ユーザー写真・第三者画像は含みません。画像decodeと有限・正規化embedding出力を検査し、意味的な画像検索品質を測るものではありません。

固定の韓国語queryとpositive/negative文は`crates/n0-probe/src/lib.rs`にあります。反復cosineの下限は0.9999、正規化vectorの許容誤差は0.001です。このsmoke検査はG4/G5 corpusを代替しません。

生成fixtureはthinking無効、出力32 tokens、context 512 tokens、top-p sampler（k=1 / p=1 / temperature=1 / seed=0）で`2 + 2`を質問します。空でない構造化SDK出力を検査し、一般的な回答品質やstreaming/cancelは評価しません。
