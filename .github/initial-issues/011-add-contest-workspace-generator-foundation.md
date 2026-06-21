# Summary

AtCoder用ローカルworkspaceを生成するための、ネットワーク非依存の基盤を `ac-core` に追加する。

## Planning metadata

* Labels: `type: feature`, `area: core`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-workspace-generator-foundation`
* Commit: `feat(core): add contest workspace generator foundation`

## Background

`cargo ac new <contest>` から複数種類のファイルを一貫した配置で生成するため、CLIに依存しない生成処理の入口が必要である。

## Scope

* 生成先path、contest ID、task一覧を受け取る生成要求型を追加する
* 生成予定のpathを組み立てる
* 生成先root directoryを作成する
* 既存pathとの衝突を事前検証する
* filesystem成功・失敗ケースのテストを追加する

## Requirements

* 実装は `ac-core` に置く
* CLI出力や対話処理を含めない
* 生成先が既に存在する場合は上書きせずエラーにする
* path操作とI/O失敗でpanicしない
* task一覧は呼び出し側から明示的に渡す
* AtCoderへのネットワークアクセスを行わない

## Acceptance criteria

* [ ] workspace生成要求を表す型が存在する
* [ ] 生成先root directoryを作成できる
* [ ] 既存の生成先を上書きしない
* [ ] I/O失敗を呼び出し元へ返す
* [ ] CLI crateに依存しない
* [ ] 一時ディレクトリを使うテストが存在する

## Non-goals

* `Cargo.toml` 本文の生成
* `src/bin/*.rs` 本文の生成
* `testcases/` 配下の生成
* `ac.toml` の保存
* `--force` オプション
* AtCoderからのtask一覧取得

## Verification

Docker環境内で以下を実行する。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core
cargo test --all
```

## Notes for implementation

後続の各生成処理を順番に呼び出せる小さなAPIにする。失敗時の完全なrollbackはこのIssueでは扱わない。
