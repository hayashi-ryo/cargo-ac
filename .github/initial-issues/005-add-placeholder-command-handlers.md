# Summary

`cargo-ac` の各サブコマンドにplaceholder handlerを追加する。

## Dependencies

* `Define clap command structure` が完了していること

## Scope

* `crates/cargo-ac/src/commands/` を作成する
* サブコマンドごとのhandler moduleを追加する
* CLI parserから各handlerへdispatchする
* 各handlerは実処理を行わず、未実装メッセージを返す
* 実装予定のコマンド境界を整理する

## Requirements

* `commands/mod.rs` を追加する
* 必要に応じて以下のmoduleを追加する

  * `new.rs`
  * `download.rs`
  * `test.rs`
  * `addcase.rs`
  * `login.rs`
  * `submit.rs`
  * `watch.rs`
  * `doctor.rs`
  * `selfcheck.rs`
  * `env.rs`
  * `lang.rs`
* 各handlerは最小限の関数を公開する
* 各handlerはまだ実処理を行わない
* 未実装であることがユーザーに分かるメッセージを出す
* `todo!()` や `unimplemented!()` によるpanicは避ける
* CLIから実行したときにpanicしない
* handlerの戻り値は、後続IssueでResult化しやすい形にする

## Acceptance criteria

* [ ] `crates/cargo-ac/src/commands/mod.rs` が存在する
* [ ] 主要サブコマンドに対応するhandler moduleが存在する
* [ ] CLI parserからhandlerへdispatchされている
* [ ] 各handlerはpanicしない
* [ ] 各handlerは未実装であることを表示する
* [ ] `cargo run -p cargo-ac -- new abc400` がpanicせず終了する
* [ ] `cargo run -p cargo-ac -- download abc400` がpanicせず終了する
* [ ] `cargo run -p cargo-ac -- test a` がpanicせず終了する
* [ ] `cargo run -p cargo-ac -- submit a` がpanicせず終了する
* [ ] `cargo run -p cargo-ac -- doctor` がpanicせず終了する
* [ ] `cargo run -p cargo-ac -- env show` がpanicせず終了する
* [ ] `cargo run -p cargo-ac -- lang refresh` がpanicせず終了する
* [ ] `cargo fmt --all -- --check` が通る
* [ ] `cargo clippy --all-targets --all-features -- -D warnings` が通る
* [ ] `cargo test --all` が通る

## Non-goals

* `new` の実処理は実装しない
* `download` の実処理は実装しない
* `test` の実処理は実装しない
* `addcase` の実処理は実装しない
* `login` の実処理は実装しない
* `submit` の実処理は実装しない
* `watch` の実処理は実装しない
* AtCoder連携は実装しない
* 設定ファイル読み書きは実装しない
* ローカルテスト実行は実装しない
* 認証・セッション管理は実装しない
* 提出結果監視は実装しない

## Notes for Codex

* 作業前に `AGENTS.md` を読んでください。
* このIssueではhandlerの器だけを作ってください。
* 実機能は実装しないでください。
* `todo!()` や `unimplemented!()` でpanicさせないでください。
* ユーザーに「まだ未実装」であることが分かる出力にしてください。
* Issue範囲外のリファクタリングは行わないでください。
