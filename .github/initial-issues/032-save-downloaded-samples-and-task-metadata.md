# Save downloaded samples and task metadata

## Summary

parserが返す構造化task/sample情報を、既存workspace layoutの `testcases/<task>/sample-N.in/out` と `ac.toml` に保存する処理を `ac-core` に追加する。

## Planning metadata

* Labels: `type: feature`, `area: testcase`, `area: config`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `feature/<issue-number>-save-downloaded-samples`
* Commit: `feat(download): save samples and task metadata`

## Background

Phase 4/5で `src/bin/<task>.rs` と `testcases/<task>/` を前提にしたworkspace layoutとlocal test runnerが整備されている。download結果はこのlayoutに保存し、既存の `cargo ac test <task>` でそのまま使える必要がある。

## Scope

* Target crate: `ac-core`
* Target files: `crates/ac-core/src/testcase.rs`、`crates/ac-core/src/config.rs`、必要ならdownload storage module、unit test
* 構造化されたtask/sample情報を受け取り、`testcases/<task>/sample-N.in` と `sample-N.out` に保存する
* 番号は1始まりにする
* `ac.toml` にdownloadしたtask情報を書き込む、または既存task情報と整合するよう更新する
* file I/O失敗、config read/write失敗をpanicではなくerrorとして返す
* parserとHTTP clientには依存しすぎず、構造化データを入力にする

## Requirements

* Depends on: `Implement AtCoder contest task list parser`, `Implement AtCoder sample parser for Japanese and English statements`
* Existing components: Phase 4 workspace layout、Phase 5 testcase discovery
* `testcases/<task>/` を保存先とし、既存のlocal test runnerが発見できる命名にする
* `sample-N.in` と `sample-N.out` のNは1始まりの連番にする
* `ac.toml` へのtask情報書き込み範囲を実装内で明確にし、既存 `new` 生成済みtaskを壊さない
* 既存の `cargo ac new`、`cargo ac test`、`cargo ac addcase` の挙動を壊さない

## Acceptance criteria

* [ ] sample pairが `testcases/<task>/sample-1.in` と `sample-1.out` として保存される
* [ ] 複数sampleが1始まりの連番で保存される
* [ ] 保存後のtestcase filesを既存testcase discoveryが扱える
* [ ] `ac.toml` にtask id、bin nameなどdownload workflowに必要なtask情報が反映される
* [ ] 既存task情報がある場合の更新方針がtestまたはNotesで明確になっている
* [ ] file I/O失敗とconfig更新失敗でpanicせずerrorを返す
* [ ] HTTP client、HTML parser、CLI handlerをこのIssueで新規設計していない

## Non-goals

* AtCoder HTML取得
* contest task list parser、sample parser
* debug HTML保存
* `cargo ac download` のCLI接続
* `cargo ac new --download` のCLI接続
* login、Cookie、CSRF token、提出、watch

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core testcase
cargo test -p ac-core config
cargo test --all
```

## Notes for implementation

`ac.toml` のschemaを広げる必要がある場合は最小限にする。既存project生成で作られる `a` から `f` のtaskとdownload結果が食い違う場合の扱いは、最も単純な更新方針を採用してPR本文に前提を書く。
