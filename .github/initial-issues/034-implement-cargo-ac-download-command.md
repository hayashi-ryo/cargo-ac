# Implement `cargo ac download <contest>`

## Summary

既存のHTTP client、parser、sample保存、debug HTML diagnosticsを統合し、`cargo ac download <contest>` をCLIから実行できるようにする。

## Planning metadata

* Labels: `type: feature`, `area: cli`, `area: download`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `feature/<issue-number>-download-command`
* Commit: `feat(cli): implement download command`

## Background

`cargo-ac` には `download` placeholderがある。Phase 6のCLI接続Issueでは、HTTP client、contest task list parser、sample parser、file writer、debug HTML保存をcore処理として利用し、CLI引数解析、終了status、ユーザー向け出力に責務を絞る。

## Scope

* Target crates: `cargo-ac`, `ac-core`
* Target files: `crates/cargo-ac/src/cli.rs`, `crates/cargo-ac/src/commands/download.rs`, core download workflow module、必要なtest
* `cargo ac download <contest>` からcore download workflowを呼び出す
* contest tasks pageを取得し、task一覧をparseし、各task pageからsampleをparseし、workspaceへ保存する
* errorをCLI向けに表示し、debug HTML pathがある場合は表示する
* CLI handlerでHTTP client、parser、file writerを新規設計しない

## Requirements

* Depends on: `Add HTTP client foundation for AtCoder pages`, `Implement AtCoder contest task list parser`, `Implement AtCoder sample parser for Japanese and English statements`, `Save downloaded samples and task metadata`, `Save debug HTML on parse failure`
* `cargo-ac` はCLI層に限定し、download workflow本体は `ac-core` に置く
* `ac-core` から `clap` に依存させない
* `cargo ac download abc400` を少なくとも完了条件に含める
* HTTP失敗、parse failure、file I/O失敗、config更新失敗をpanicではなくerrorとして表示する
* 既存の `cargo ac new`、`cargo ac test`、`cargo ac addcase` の挙動を壊さない

## Acceptance criteria

* [ ] `cargo ac download <contest>` がplaceholderではなくdownload workflowを実行する
* [ ] `cargo ac download abc400` で公開contest/task pageからsample取得を試せる
* [ ] 取得したsampleが `testcases/<task>/sample-N.in/out` に保存される
* [ ] task情報が `ac.toml` に反映される
* [ ] parse failure時にdebug HTML pathがCLI error出力に含まれる
* [ ] HTTP client、parser、file writerの責務がCLI handlerに混ざっていない
* [ ] login、Cookie、CSRF token、提出、watchが含まれていない

## Non-goals

* HTTP client、parser、file writerの大規模な再設計
* `cargo ac new <contest> --download`
* 認証が必要なページのdownload
* submit、judge結果取得、watch
* AtCoderへの過剰アクセスを伴うbulk機能

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core
cargo test -p cargo-ac
cargo test --all
cargo run -p cargo-ac -- download abc400
```

## Notes for implementation

実サイト確認は手動verificationとして扱い、CI testはfixtureやmock HTTP layerを優先する。失敗時は「どのcontest/task/URLで失敗したか」と「次に確認すべきdebug HTML path」が分かる表示にする。
