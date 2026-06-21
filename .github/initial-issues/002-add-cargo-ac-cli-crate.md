# Summary

CLI本体となる `cargo-ac` crateを追加する。

## Scope

* `crates/cargo-ac/` を作成する
* `crates/cargo-ac/Cargo.toml` を作成する
* `crates/cargo-ac/src/main.rs` を作成する
* workspace memberに `crates/cargo-ac` を追加する
* 最小限の `main` 関数を追加する

## Requirements

* crate名は `cargo-ac` とする
* edition、license、repositoryはworkspaceから継承する
* このIssueでは `clap` の本格導入は行わない
* `main.rs` は最小限でよい
* `cargo run -p cargo-ac` が実行できる状態にする

## Acceptance criteria

* [ ] `crates/cargo-ac/Cargo.toml` が存在する
* [ ] `crates/cargo-ac/src/main.rs` が存在する
* [ ] workspace membersに `crates/cargo-ac` が含まれている
* [ ] `cargo run -p cargo-ac` が実行できる
* [ ] `cargo fmt --all -- --check` が通る
* [ ] `cargo test --all` が通る

## Non-goals

* `clap` によるコマンド構造は実装しない
* `ac-core` への依存は追加しない
* AtCoder連携は実装しない
* `cargo ac` としての実行確認はまだ行わない

## Notes for Codex

* 最小限のCLI crate追加に留めてください。
* 余計なコマンドやサブコマンドは実装しないでください。
