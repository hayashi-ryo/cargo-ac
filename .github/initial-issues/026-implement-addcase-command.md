# Implement `cargo ac addcase`

## Summary

inputとexpected outputを受け取り、衝突しない `custom-N.in` / `custom-N.out` pairとして保存する。

## Planning metadata

* Labels: `type: feature`, `area: testcase`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-addcase-command`
* Commit: `feat(testcase): add custom testcase`

## Background

利用者が手作業でpair名を管理せず、自作caseをPhase 5と同じlayoutへ安全に追加できるcommandが必要である。

## Scope

* Target crates: `ac-core`, `cargo-ac`
* Target files: `crates/ac-core/src/testcase.rs`, `crates/cargo-ac/src/commands/addcase.rs`、必要なCLI test
* `cargo-ac` でinputとexpected outputを対話的に受け取る
* `ac-core` でtask directoryを解決し、未使用の最小正整数Nを選ぶ
* `custom-N.in` と `custom-N.out` をpairとして保存する

## Requirements

* Depends on: `Define testcase file layout and discovery`
* Independent of: runner、comparison、`cargo ac test` CLI接続
* taskは `ac.toml` から解決し、無効なtaskを拒否する
* 片側だけの既存 `custom-N` も使用済みとして扱い、上書きしない
* promptとterminal I/Oは `cargo-ac`、path選択と保存は `ac-core` が担当する

## Acceptance criteria

* [ ] inputとexpected outputをそれぞれ受け取れる
* [ ] `testcases/<task>/custom-N.in` と `custom-N.out` に内容を保存する
* [ ] 両方または片側だけ存在する番号を避け、既存fileを上書きしない
* [ ] 無効なtask、directory読込失敗、file作成・書込失敗がpanicせずCLI errorになる
* [ ] pairの片側作成後に失敗した場合、成功したpairとして報告しない
* [ ] 番号選択と保存のunit test、対話入力境界のtestが一時workspaceで通る

## Non-goals

* testcase編集・削除・一覧command
* sample fileのdownloadまたは命名変更
* runner実行、output比較、`cargo ac test` 接続
* AtCoderアクセス、認証、Cookie、提出

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core testcase
cargo test -p cargo-ac
cargo test --all
```

## Notes for implementation

複雑なeditor連携は追加せず、複数行入力を完了できる最小の対話方式をPR本文で明記する。完全なtransaction実装を要求せず、失敗時に既存fileを壊さないことを優先する。
