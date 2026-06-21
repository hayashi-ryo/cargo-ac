# Summary

Phase 4のcontest workspace生成全体を対象とするintegration testを追加する。

## Planning metadata

* Labels: `type: test`, `area: core`, `area: cli`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `test/<issue-number>-workspace-generation`
* Commit: `test(core): cover contest workspace generation`

## Background

個別componentのテストだけでは、`cargo ac new` が生成するファイル間の整合性や既存path保護を十分に確認できない。

## Scope

* 一時ディレクトリでworkspace生成全体を実行する
* 生成されたmanifest、設定、source、testcase directoryを検証する
* 生成workspaceのbuildを検証する
* 既存pathと不正入力の失敗ケースを検証する

## Requirements

* テストは互いに独立させる
* リポジトリやユーザーdirectoryへfixtureを残さない
* ネットワークを利用しない
* filesystem順序やOS固有path表現に依存しない
* エラー時に既存ファイルが変更されないことを確認する

## Acceptance criteria

* [ ] 完全なworkspace生成の正常系テストが存在する
* [ ] 生成された `ac.toml` を読み戻せる
* [ ] 生成されたworkspaceをbuildできる
* [ ] 既存pathを保護するテストが存在する
* [ ] 不正なcontest IDまたはtask名のテストが存在する
* [ ] テストがAtCoderへアクセスしない

## Non-goals

* 新しい生成機能やCLI optionの追加
* performance benchmark
* AtCoder通信のmock
* Phase 5のtest runner実装
* Windows固有CI jobの追加

## Verification

Docker環境内で以下を実行する。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

## Notes for implementation

既存のunit testと重複しすぎないよう、integration testではcomponent間の接続と生成物全体を重視する。
