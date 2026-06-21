# Summary

contest workspaceに標準のtask sourceとtestcase directoryを生成する。

## Planning metadata

* Labels: `type: feature`, `area: core`, `area: testcase`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-generate-task-layout`
* Commit: `feat(core): generate task sources and testcase directories`

## Background

`cargo ac new` の直後から各taskのRustコードを書き、sample testcaseを配置できるdirectory構造が必要である。

## Scope

* taskごとに `src/bin/<task>.rs` を生成する
* taskごとに `testcases/<task>/` を生成する
* 標準入力を読む最小Rust templateを用意する
* task名とpathの検証を追加する
* 生成内容のテストを追加する

## Requirements

* task一覧は生成要求から受け取る
* task名からworkspace外へ書き出せないよう検証する
* 既存ファイルを上書きしない
* source templateへ外部crateを追加しない
* sample testcase自体は生成しない
* AtCoderへのネットワークアクセスを行わない

## Acceptance criteria

* [ ] 各taskの `src/bin/<task>.rs` が生成される
* [ ] 各taskの `testcases/<task>/` が生成される
* [ ] 生成sourceがRustコードとしてcompileできる
* [ ] 不正なtask名でworkspace外へ書き込まない
* [ ] 既存ファイルを上書きしない
* [ ] 生成layoutのテストが存在する

## Non-goals

* ユーザー定義template
* `--template` オプション
* sample input/outputの生成や取得
* `cargo ac addcase` の実装
* AtCoder task一覧の取得

## Verification

Docker環境内で以下を実行する。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core
cargo test --all
```

生成fixtureで `cargo check --bins` を実行し、templateがcompileできることを確認する。

## Notes for implementation

default task一覧そのものをこの処理へ埋め込まず、呼び出し側から渡す。高度なtemplate管理は後続Issueへ残す。
