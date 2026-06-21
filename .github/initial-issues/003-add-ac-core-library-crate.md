# Summary

共通処理を配置するための `ac-core` library crateを追加する。

## Scope

* `crates/ac-core/` を作成する
* `crates/ac-core/Cargo.toml` を作成する
* `crates/ac-core/src/lib.rs` を作成する
* workspace memberに `crates/ac-core` を追加する
* 最小限のlibrary crateとしてビルドできる状態にする

## Requirements

* crate名は `ac-core` とする
* edition、license、repositoryはworkspaceから継承する
* `ac-core` はCLIに依存しない
* `clap` には依存しない
* まだ設定・runner・AtCoder連携は実装しない

## Acceptance criteria

* [ ] `crates/ac-core/Cargo.toml` が存在する
* [ ] `crates/ac-core/src/lib.rs` が存在する
* [ ] workspace membersに `crates/ac-core` が含まれている
* [ ] `cargo test -p ac-core` が実行できる
* [ ] `cargo fmt --all -- --check` が通る
* [ ] `cargo test --all` が通る

## Non-goals

* CLI crateからの利用は実装しない
* 設定ファイル読み書きは実装しない
* テストランナーは実装しない
* AtCoder連携は実装しない

## Notes for Codex

* `ac-core` にCLI専用依存を入れないでください。
* 特に `clap` は追加しないでください。
* crate追加のみの小さな変更にしてください。

## Additional maintenance task

`Add cargo-ac CLI crate` のPR作成時に、`scripts/finish-codex-issue.sh` の末尾にある `gh pr create` が行継続なしで改行されていたため、`--base: command not found` が発生した。

このIssueでは、`ac-core` crate追加に加えて、開発補助スクリプトの軽微修正として `scripts/finish-codex-issue.sh` も修正する。

### Additional scope

* `scripts/finish-codex-issue.sh` の `gh pr create` 呼び出しを修正する
* `gh pr create` が対話モードに入らないようにする
* 既存PRがある場合は、再作成せず既存PRのURLを表示する
* 行継続 `\` に依存しにくい形にする

### Additional acceptance criteria

* [ ] `gh pr create` が1つのコマンドとして実行される
* [ ] `--base: command not found` が発生しない
* [ ] 既存PRがある場合は既存PRのURLを表示する
* [ ] `bash -n scripts/finish-codex-issue.sh` が通る

### Additional non-goals

* `scripts/prepare-codex-issue.sh` は変更しない
* PR本文生成ロジックの大幅な変更は行わない
* GitHub Projects操作の自動化は行わない
