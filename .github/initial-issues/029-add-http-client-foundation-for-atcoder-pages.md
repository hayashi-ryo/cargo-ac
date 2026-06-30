# Add HTTP client foundation for AtCoder pages

## Summary

AtCoderの公開contest pageとtask pageを取得するための、Phase 6向けHTTP client foundationを `ac-core` に追加する。

## Planning metadata

* Labels: `type: feature`, `area: atcoder`, `area: network`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `feature/<issue-number>-http-client-foundation`
* Commit: `feat(atcoder): add HTTP client foundation`

## Background

Phase 6では `cargo ac download <contest>` がAtCoderのcontest tasks pageと各task pageを取得する。HTTP accessをparserやCLI handlerに混ぜるとfixture testが難しくなるため、まず公開HTML取得に必要な最小限のclient foundationを分離して用意する。

## Scope

* Target crate: `ac-core`
* Target files: `crates/ac-core/src/atcoder.rs` または同等のcore module、`crates/ac-core/src/lib.rs`、必要なunit test
* 公開contest tasks pageとtask pageのURLを組み立てる
* HTML取得処理、HTTP status確認、user-agentなど最小限のrequest設定を実装する
* HTTP失敗時にoperation、contest id、task idまたはURL、HTTP statusを含むerrorを返す
* parser、file writer、CLI出力は実装しない

## Requirements

* Depends on: Phase 5 workspace/test runner completion
* `ac-core` に実装し、`cargo-ac` や `clap` に依存しない
* 公開ページ取得に限定し、login、Cookie、CSRF token、submit用POSTは扱わない
* AtCoderへの過剰アクセスを避けるため、download workflowから呼び出しやすい境界にする
* 外部入力、URL組み立て、HTTP失敗、HTTP status errorでpanicしない
* response bodyはHTML文字列として返し、parserには依存しない

## Acceptance criteria

* [ ] contest idからtasks page URLを作れる
* [ ] contest idとtask idまたはtask pathからtask page URLを作れる
* [ ] 公開HTML取得の成功時にbodyを返す
* [ ] non-success HTTP statusをoperation、URL、status付きerrorとして返す
* [ ] network errorをoperation、URL付きerrorとして返す
* [ ] `ac-core` が `cargo-ac` と `clap` に依存していない
* [ ] login、Cookie、CSRF token、提出処理が含まれていない

## Non-goals

* contest task list parser、sample parser、file writerの実装
* `cargo ac download` へのCLI接続
* 認証、session永続化、Cookie、CSRF token、submit、watch
* retry policyや高度なcache機構の実装

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core
cargo test --all
```

## Notes for implementation

実サイトへの手動確認が必要な場合はunit testから分離し、PR本文のManual verificationに記載する。unit testはURL組み立て、error変換、mock可能なHTTP境界を中心にする。
