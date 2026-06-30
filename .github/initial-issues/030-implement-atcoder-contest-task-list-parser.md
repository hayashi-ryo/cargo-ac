# Implement AtCoder contest task list parser

## Summary

AtCoder contest tasks pageのHTML文字列から、task id、task title、task URLまたはURL pathを抽出するparserを `ac-core` に追加する。

## Planning metadata

* Labels: `type: feature`, `area: parser`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `feature/<issue-number>-contest-task-list-parser`
* Commit: `feat(parser): parse contest task list`

## Background

download workflowはcontest内のtask一覧を取得してから各task pageを処理する。task list parserはHTTP clientに依存せず、AtCoder HTML fixtureから安定して検証できるcomponentとして実装する。

## Scope

* Target crate: `ac-core`
* Target files: `crates/ac-core/src/parser.rs`、`crates/ac-core/src/lib.rs`、parser fixture test filesまたはinline fixtures
* contest tasks page HTML文字列を入力にして構造化されたtask一覧を返す
* task id、task title、task URLまたはURL pathを抽出する
* parse failure時にoperation、contest id、足りない要素の概要を含むerrorを返す
* HTTP取得、debug HTML保存、file output、CLI接続は実装しない

## Requirements

* Depends on: no implementation dependency; can run in parallel with `Implement AtCoder sample parser for Japanese and English statements`
* Parser inputはHTML文字列とcontest idに限定し、HTTP clientへ依存しない
* AtCoder tasks pageのtable/link構造変更に気づきやすいfixture testを追加する
* 外部HTMLの欠落や想定外構造でpanicしない
* task URLは後続HTTP clientが利用できる形で保持する

## Acceptance criteria

* [ ] fixture HTMLから複数taskのtask idを抽出できる
* [ ] fixture HTMLからtask titleを抽出できる
* [ ] fixture HTMLからtask URLまたはURL pathを抽出できる
* [ ] 空または不完全なHTMLでpanicせずparse errorを返す
* [ ] parser unit/fixture testがHTTP clientやネットワークに依存しない
* [ ] file writer、debug HTML保存、CLI処理が含まれていない

## Non-goals

* task page HTML取得
* sample input/output parser
* sample file保存、`ac.toml`更新
* `cargo ac download` の実装
* login、Cookie、CSRF token、提出、watch

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core parser
cargo test --all
```

## Notes for implementation

既存crate構成に合わせてparser module名は最小限にする。HTML parser crateを追加する場合は、ad hoc string matchingより構造化parserを優先し、依存追加の理由をPR本文に書く。
