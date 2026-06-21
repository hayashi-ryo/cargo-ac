# Summary

`ac-core` にローカルプロジェクト設定 `ac.toml` のデータモデルを追加する。

## Planning metadata

* Labels: `type: feature`, `area: config`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-define-ac-toml-model`
* Commit: `feat(config): define ac.toml data model`

## Background

`cargo ac new` が生成する設定と後続コマンドが読む設定で、同じ型を利用できる土台が必要である。

## Scope

* `ac-core` にプロジェクト設定の型を追加する
* contest ID、source directory、testcase directory、language、Rust edition、task情報を表現する
* 必要最小限のconstructorと参照用APIを追加する

## Requirements

* CLIに依存しない型として実装する
* public APIは後続Issueで必要になる最小限にする
* 不正な外部入力でpanicしない設計にする
* Rustのlanguage_idをモデルへ含めない
* AtCoderへのネットワークアクセスを行わない

## Acceptance criteria

* [ ] `ac-core` に `ac.toml` 用データモデルが存在する
* [ ] contest IDと各ディレクトリを表現できる
* [ ] languageとRust editionを表現できる
* [ ] task IDとbin名を表現できる
* [ ] `ac-core` がCLI依存を持たない
* [ ] モデルの単体テストが存在する

## Non-goals

* `ac.toml` のファイル読み書き
* `cargo ac new` の実装
* ディレクトリやソースファイルの生成
* AtCoderからのtask取得
* language_idの管理

## Verification

Docker環境内で以下を実行する。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core
cargo test --all
```

## Notes for implementation

後続の読み書き実装でserializationを追加できる構造にする。フィールドや型を将来用途だけを理由に増やさない。
