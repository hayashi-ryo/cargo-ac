# Summary

CLI境界での基本的なエラーハンドリング方針を実装する。

## Dependencies

* `Define clap command structure` が完了していること
* `Add placeholder command handlers` が完了していること

## Scope

* `cargo-ac` crateにCLI境界のエラー型またはResult aliasを追加する
* `main.rs` でエラーを受け取り、ユーザー向けメッセージとして表示する
* handlerがResultを返す構造にする
* panicではなくResultで失敗を表現できる形にする
* 最小限のテストまたは確認を追加する

## Requirements

* CLIのentrypointで `Result` を扱う
* handler関数はResultを返せる構造にする
* ユーザー向けエラーメッセージはstderrへ出す
* 異常終了時は非ゼロexit codeで終了する
* `anyhow` などを使う場合はCLI crate側に閉じる
* `ac-core` のエラー設計をこのIssueで大きく決めない
* `ac-core` にCLI表示用の依存を入れない
* placeholder handlerは基本的に正常終了でよい
* 将来的に `ac-core` の型付きエラーへ接続できる余地を残す
* `todo!()` や `unimplemented!()` によるpanicは使わない

## Acceptance criteria

* [ ] CLI境界でResultを扱う構造になっている
* [ ] handler関数がResultを返せる
* [ ] エラー時にstderrへメッセージを表示できる
* [ ] エラー時に非ゼロexit codeで終了できる
* [ ] `todo!()` や `unimplemented!()` によるpanicがない
* [ ] `ac-core` にCLI専用依存が追加されていない
* [ ] `cargo run -p cargo-ac -- doctor` が正常終了する
* [ ] エラーを発生させる最小確認手段がある、またはPR本文に未確認理由が書かれている
* [ ] `cargo fmt --all -- --check` が通る
* [ ] `cargo clippy --all-targets --all-features -- -D warnings` が通る
* [ ] `cargo test --all` が通る

## Non-goals

* `ac-core` の詳細なエラー型設計は行わない
* AtCoder連携のエラー設計は行わない
* parser失敗時のdebug HTML保存は実装しない
* ログイン切れ検知は実装しない
* 提出失敗時の詳細エラーは実装しない
* 高度な診断メッセージは実装しない
* ロギング基盤は実装しない

## Notes for Codex

* 作業前に `AGENTS.md` を読んでください。
* このIssueではCLI境界の最小限のエラーハンドリングだけを実装してください。
* `ac-core` の本格的なエラー設計は後続Issueに残してください。
* `ac-core` にCLI表示用の依存を入れないでください。
* Issue範囲外の機能実装は行わないでください。
* 判断に迷う場合は、CLI crate側に閉じた単純なResult処理にしてください。
