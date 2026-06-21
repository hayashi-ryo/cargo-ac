# Summary

`cargo-ac` のCLI引数構造を `clap` で定義する。

## Dependencies

* `Add cargo-ac CLI crate` が完了していること
* `Add ac-core library crate` が完了していること

## Scope

* `cargo-ac` crateに `clap` を追加する
* CLI定義用の `cli.rs` を追加する
* サブコマンド定義を追加する
* `main.rs` からCLI parserを呼び出す
* 各サブコマンドはまだ実処理を持たない
* placeholder handlerへdispatchできる構造にする

## Requirements

* Cargoサブコマンドとして `cargo ac ...` で使う前提のCLI名にする
* 最初に定義するサブコマンドは以下とする

  * `new`
  * `download`
  * `test`
  * `addcase`
  * `login`
  * `submit`
  * `watch`
  * `doctor`
  * `selfcheck`
  * `env`
  * `lang`
* `env` は subcommand として `show` / `update` を持つ
* `lang` は subcommand として `refresh` を持つ
* 短縮aliasはこのIssueでは実装しない
* 各サブコマンドの引数は最小限に留める
* 実処理は行わず、placeholder handlerに渡すだけにする
* `ac-core` の機能実装は行わない

## Initial command shape

以下のようなコマンドを将来的に扱える構造にする。

```text
cargo ac new <contest>
cargo ac download <contest>
cargo ac test <task>
cargo ac addcase <task>
cargo ac login
cargo ac submit <task>
cargo ac submit <task> --watch
cargo ac watch
cargo ac doctor
cargo ac selfcheck
cargo ac env show
cargo ac env update
cargo ac lang refresh
```

## Acceptance criteria

* [ ] `crates/cargo-ac/src/cli.rs` が存在する
* [ ] `clap` によるCLI parserが定義されている
* [ ] `new` サブコマンドが定義されている
* [ ] `download` サブコマンドが定義されている
* [ ] `test` サブコマンドが定義されている
* [ ] `addcase` サブコマンドが定義されている
* [ ] `login` サブコマンドが定義されている
* [ ] `submit` サブコマンドが定義されている
* [ ] `watch` サブコマンドが定義されている
* [ ] `doctor` サブコマンドが定義されている
* [ ] `selfcheck` サブコマンドが定義されている
* [ ] `env show` / `env update` が定義されている
* [ ] `lang refresh` が定義されている
* [ ] `main.rs` からCLI parserを呼び出している
* [ ] `cargo run -p cargo-ac -- --help` が実行できる
* [ ] `cargo run -p cargo-ac -- new --help` が実行できる
* [ ] `cargo run -p cargo-ac -- env --help` が実行できる
* [ ] `cargo fmt --all -- --check` が通る
* [ ] `cargo clippy --all-targets --all-features -- -D warnings` が通る
* [ ] `cargo test --all` が通る

## Non-goals

* 各サブコマンドの実処理は実装しない
* AtCoder連携は実装しない
* 設定ファイル読み書きは実装しない
* ローカルテスト実行は実装しない
* 提出処理は実装しない
* 短縮aliasは実装しない
* Shell completionは実装しない
* `ac-core` に `clap` 依存を追加しない

## Notes for Codex

* 作業前に `AGENTS.md` を読んでください。
* このIssueではCLI構造の定義に集中してください。
* 実処理はplaceholder handlerへ渡すだけにしてください。
* `ac-core` に `clap` 依存を入れないでください。
* Issue範囲外の実装を追加しないでください。
* 判断に迷う場合は、最も単純なCLI定義にしてください。
