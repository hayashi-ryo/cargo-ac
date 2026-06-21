# Summary

`ac-core` に `ac.toml` の読み書き処理を追加する。

## Planning metadata

* Labels: `type: feature`, `area: config`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-ac-toml-read-write`
* Commit: `feat(config): add ac.toml read and write`

## Background

ローカルプロジェクト生成時に設定を保存し、後続コマンドで同じ設定を復元する必要がある。

## Scope

* Issue「Define ac.toml data model」の型をTOMLへ保存する
* `ac.toml` から設定型を読み込む
* I/O失敗と形式不正を型付きエラーとして返す
* round-tripテストと不正TOMLのテストを追加する

## Requirements

* 実装は `ac-core` に置く
* ファイルI/Oとparse失敗でpanicしない
* エラーから対象pathと失敗の種類を判別できるようにする
* テストは一時ディレクトリを利用し、リポジトリを汚さない
* 秘密情報を設定やエラーへ追加しない
* AtCoderへのネットワークアクセスを行わない

## Acceptance criteria

* [ ] 設定を `ac.toml` として保存できる
* [ ] 保存した設定を同じ値として読み込める
* [ ] 存在しないファイルを通常のエラーとして返す
* [ ] 不正TOMLを通常のエラーとして返す
* [ ] エラーにCookieやtokenなどの秘密情報を含めない
* [ ] round-tripと失敗ケースのテストが存在する

## Non-goals

* グローバル設定の読み書き
* sessionやCookieの保存
* `cargo ac new` のCLI統合
* 設定migration
* AtCoderアクセス

## Verification

Docker環境内で以下を実行する。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core
cargo test --all
```

## Notes for implementation

依存crateを追加する場合は `ac-core` に閉じ、最小限の機能だけを有効にする。atomic writeはこのIssueの必須要件にしない。
