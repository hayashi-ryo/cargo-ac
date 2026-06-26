# Display WA diff

## Summary

WA時にexpectedとactualを明確に識別できる、最小限のline-based diffを生成・表示する。

## Planning metadata

* Labels: `type: feature`, `area: runner`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-wa-diff-output`
* Commit: `feat(runner): display wrong answer diff`

## Background

WAというstatusだけでは修正箇所が分からないため、comparison resultからexpected/actualの差を安全に確認できる表示が必要である。

## Scope

* Target crates: `ac-core`, `cargo-ac`
* Target files: comparison moduleと `crates/cargo-ac/src/commands/test.rs` または専用formatter
* `ac-core` でcomparison resultからterminal非依存のdiff modelを作る
* `cargo-ac` でexpected/actual labelと差分行を表示する

## Requirements

* Depends on: `Normalize and compare output`
* Parallel with: `Display AC, WA, RE, and TLE results`
* normalized outputを基準にし、比較規則とdiff内容を矛盾させない
* terminal formatting dependencyを `ac-core` に追加しない

## Acceptance criteria

* [ ] WA時に `expected` と `actual` を識別できるlabelが表示される
* [ ] 相違行と片側だけに存在する行を識別できる
* [ ] multiline、empty output、末尾だけが異なるcaseのunit testがある
* [ ] AC、RE、TLEではWA diffを生成・表示しない
* [ ] diff生成がinvalid external inputでpanicしない

## Non-goals

* full-screen diff viewer、interactive pager、syntax highlighting
* output normalization規則の変更
* status summaryまたは `cargo ac test` CLI接続
* AtCoderアクセス、認証、提出

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core output
cargo test -p cargo-ac
cargo test --all
```

## Notes for implementation

外部diff commandには依存せず、Phase 5で必要なexpected/actual識別に限定する。表示責務の境界はresult formatter Issueと共有する。
