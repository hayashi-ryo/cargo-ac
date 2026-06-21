# Summary

Rust workspaceを作成し、今後 `cargo-ac` と `ac-core` を配置できる土台を用意する。

## Scope

* ルート `Cargo.toml` を作成する
* workspace設定を追加する
* `resolver = "2"` を設定する
* workspace共通package metadataを設定する
* `crates/` ディレクトリを作成する

## Requirements

* ルートに `Cargo.toml` を作成する
* workspace membersとして、将来的に以下を配置できる構成にする

  * `crates/cargo-ac`
  * `crates/ac-core`
* `edition = "2021"` をworkspace packageに設定する
* `license = "MIT OR Apache-2.0"` をworkspace packageに設定する
* repository URLを設定する
* まだcrate本体は作成しない

## Acceptance criteria

* [ ] ルート `Cargo.toml` が存在する
* [ ] `[workspace]` が定義されている
* [ ] `resolver = "2"` が設定されている
* [ ] `[workspace.package]` が定義されている
* [ ] `license = "MIT OR Apache-2.0"` が設定されている
* [ ] `crates/` ディレクトリが存在する
* [ ] `cargo fmt --all -- --check` が実行できる、または未実行理由がPRに記載されている
* [ ] `cargo test --all` が実行できる、または未実行理由がPRに記載されている

## Non-goals

* `cargo-ac` crateは作成しない
* `ac-core` crateは作成しない
* CLIコマンドは実装しない
* AtCoder連携は実装しない

## Notes for Codex

* Issue範囲外のcrate作成やCLI実装は行わないでください。
* 変更はworkspace初期化に限定してください。
* `Cargo.toml` のlicenseは `MIT OR Apache-2.0` としてください。
