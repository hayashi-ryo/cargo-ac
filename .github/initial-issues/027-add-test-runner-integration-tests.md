# Add test runner integration tests

## Summary

一時contest workspaceとfixture binaryを使い、Phase 5の主要CLI経路とAC、WA、RE、TLEをend-to-endで検証する。

## Planning metadata

* Labels: `type: test`, `area: runner`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `test/<issue-number>-test-runner-integration`
* Commit: `test(runner): cover local test runner`

## Background

component unit testだけでは、config、testcase files、Cargo binary、CLI表示、終了statusの接続を保証できない。

## Scope

* Target crate: `cargo-ac`
* Target files: `crates/cargo-ac/tests/test_command.rs`, `crates/cargo-ac/tests/addcase_command.rs`、必要なtest fixture
* 一時workspaceに最小の `ac.toml`、fixture binary、testcase pairを作る
* `cargo ac test <task>`、`all`、`--release` と `cargo ac addcase <task>` の主要経路を検証する
* unit testで保証済みの細かなnormalization組合せは重複させない

## Requirements

* Depends on: `Connect cargo ac test to the local test runner`, `Implement cargo ac addcase`
* AC、WA、RE、TLEを実processとCLI boundaryを通して検証する
* testはAtCoderアクセス、認証情報、利用者の既存contest workspaceへ依存しない
* 一時workspaceとfixtureをtest終了後に残さない
* timeout caseは不必要に長く待たず、flakyにならない余裕を持つ

## Acceptance criteria

* [ ] ACでstatus表示、summary、exit status 0を検証する
* [ ] WAでexpected/actual diffとnon-zero exitを検証する
* [ ] REでstderr、exit status表示、CLIのnon-zero exitを検証する
* [ ] TLEでtimeout表示、non-zero exit、子process回収を検証する
* [ ] `test all` が複数taskを実行し、`--release` がrelease経路を使うことを検証する
* [ ] testcase不在と無効taskのCLI errorを検証する
* [ ] `addcase` が衝突しないpairを一時workspaceへ作成する経路を検証する
* [ ] test suiteがネットワークと利用者環境に依存しない

## Non-goals

* unit testで網羅済みの全error variantの重複検証
* performance benchmark、全platform固有process test
* Phase 6以降のcommand、AtCoderアクセス、認証、Cookie、提出
* production codeのrefactorまたは新機能追加

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p cargo-ac --test test_command
cargo test -p cargo-ac --test addcase_command
cargo test --all
```

## Notes for implementation

既存の `new_command` integration testの一時workspace patternを再利用する。fixture binaryは入力反映、non-zero exit、sleepを小さく制御できるものに限定する。
