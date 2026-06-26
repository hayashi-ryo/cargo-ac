# Display AC, WA, RE, and TLE results

## Summary

coreのstructured resultを、caseごとのAC、WA、RE、TLEと全体summaryとしてCLIへ表示する。

## Planning metadata

* Labels: `type: feature`, `area: cli`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-test-result-output`
* Commit: `feat(test): display testcase results`

## Background

runnerのprocess制御や比較処理にCLI formattingを混在させず、利用者が失敗種別と次の確認箇所を識別できる表示境界が必要である。

## Scope

* Target crate: `cargo-ac`
* Target files: `crates/cargo-ac/src/commands/test.rs` または専用のCLI formatter module
* coreのexecution/comparison resultをAC、WA、RE、TLEへ分類してcase名とともに表示する
* task単位と全体の件数summaryを表示する
* REではexit statusと秘密情報を含まないstderr、TLEではtimeoutであることを表示する

## Requirements

* Depends on: `Execute task binaries with stdin and timeout`, `Normalize and compare output`
* Parallel with: `Display WA diff`
* result modelは `ac-core`、文言とterminal出力は `cargo-ac` が担当する
* Cookie、tokenなどの秘密情報を扱わない

## Acceptance criteria

* [ ] 各caseにcase nameとAC、WA、RE、TLEのいずれかが表示される
* [ ] REにexit statusとstderrが表示される
* [ ] TLEが通常のREと区別される
* [ ] taskごとのAC/WA/RE/TLE件数とtotalをsummary表示する
* [ ] 0件を含むsummary formattingのunit testがある
* [ ] formatterがprocess実行やclap parsingを行わない

## Non-goals

* WAのexpected/actual diff本文
* `cargo ac test` の `<task>`、`all`、`--release` 接続
* runner、comparison rule、exit status policyの新規設計
* color theme、高度なprogress UI、AtCoderアクセス、認証、提出

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p cargo-ac
cargo test --all
```

## Notes for implementation

出力先をtest可能にする最小限のwriter境界を使う。表示のためだけにcoreへterminal依存を追加しない。
