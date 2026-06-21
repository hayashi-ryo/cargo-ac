# cargo-ac

`cargo-ac` は、RustでAtCoderに参加する人のためのCLIツールです。

Rustの開発体験に自然になじむ形で、AtCoderの問題取得、ローカルテスト、提出、提出結果の確認までを一気通貫で扱えることを目指します。

## コンセプト

Rustを書く人にとって気持ちいい、現代的なAtCoder CLI。

既存のAtCoder向けツールには便利なものがありますが、長期間更新されていないものもあり、AtCoder側のHTML構造変更、言語ID変更、Rust実行環境の更新に追従しづらい場合があります。

本プロジェクトでは、単にAtCoderへ提出できるだけではなく、以下を重視します。

* Cargo標準の開発体験に近い操作感
* AtCoderの仕様変更に追従しやすい設計
* AtCoderのRust実行環境に追従しやすい設計
* ローカルテストから提出、結果確認までの一貫した操作
* 壊れにくく、壊れたときに直しやすい実装
* Codexなどの開発支援ツールに作業を依頼しやすいIssue・PR運用

## 開発ステータス

このプロジェクトは現在、初期開発中です。

まずは以下の基本機能を実装することを目標にしています。

| コマンド | 目的 |
| ----------------------- | ------------------------ |
| `cargo ac new` | AtCoder用のRustプロジェクトを生成する |
| `cargo ac download` | 問題ページからサンプルを取得する |
| `cargo ac test` | ローカルでサンプルテストを実行する |
| `cargo ac addcase` | 自作テストケースを追加する |
| `cargo ac login` | AtCoderにログインする |
| `cargo ac submit` | コードを提出する |
| `cargo ac watch` | 提出結果を監視する |
| `cargo ac lang refresh` | Rustの提出用language_idを更新する |
| `cargo ac doctor` | ローカル環境や設定を簡易診断する |
| `cargo ac selfcheck` | AtCoder側の構造変更を検知する |
| `cargo ac env` | AtCoder Rust環境情報を扱う |

## 想定する利用例

```bash
cargo ac login
cargo ac new abc400
cd abc400

cargo ac download abc400
cargo ac test a
cargo ac submit a --watch
```

最初の実装目標は、AtCoder連携よりも先に、以下の2つを気持ちよく動かすことです。

```bash
cargo ac new abc400
cargo ac test a
```

その後、問題ダウンロード、ログイン、提出、結果監視、環境診断を段階的に追加していきます。

## 設計方針

| 方針 | 内容 |
| ---------------------- | -------------------------------------------- |
| Cargo-native | Cargoサブコマンドとして自然に使えるCLIを目指す |
| AtCoder-aware | AtCoderのHTML構造、ログイン、提出、言語ID変更を考慮する |
| Rust-environment-aware | AtCoderのRustバージョン、edition、利用可能crateを扱えるようにする |
| Maintenance-first | 仕様変更に気づきやすく、修正しやすい構成にする |
| Codex-friendly | 小さなIssue単位で安全に開発を進められる運用にする |

## 初期段階で対象外とするもの

初期開発では、スコープを広げすぎないために以下は対象外とします。

| 項目 | 理由 |
| -------- | --------------------------- |
| 複数OJ対応 | まずはAtCoderとRustに集中するため |
| GUI | CLIとしての体験を優先するため |
| AI解説生成 | AtCoderのコンテスト中ルールとの関係が難しいため |
| VSCode拡張 | CLIが安定してから検討するため |
| ブラウザ自動操作 | HTTP/HTMLベースの実装を優先するため |
| 高度な統計機能 | 初期MVPの範囲外とするため |

## AtCoderへの配慮

本ツールでは、AtCoderに対して過剰なアクセスを行わないことを重視します。

* 取得済みデータは可能な限りキャッシュする
* 提出結果の監視では短すぎる間隔でポーリングしない
* 不要な再取得を避ける
* 提出連打を防ぐ設計を検討する
* AtCoderの利用規約やコンテストルールに反する使い方を推奨しない

## 開発環境

ホストOSのRust環境に依存せずビルドと検証を行うため、Docker開発環境を利用できます。セットアップと標準検証コマンドは [Docker開発環境](docs/development-environment.md) を参照してください。

## ライセンス

このプロジェクトは、以下のいずれかのライセンスの下で利用できます。

* MIT License
* Apache License, Version 2.0

利用者は、どちらか一方のライセンス条件を選択して利用できます。
