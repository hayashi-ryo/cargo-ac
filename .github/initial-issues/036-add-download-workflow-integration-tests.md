# Add download workflow integration tests

## Summary

fixture HTML、temporary workspace、mock HTTP layerを使い、download commandと `new --download` の主要経路をintegration testで検証する。

## Planning metadata

* Labels: `type: test`, `area: download`, `area: parser`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `test/<issue-number>-download-integration-tests`
* Commit: `test(download): cover download workflow`

## Background

unit/fixture testだけでは、HTTP boundary、parser、sample保存、`ac.toml`更新、CLI statusの接続を保証できない。一方でCI testがAtCoder実サイト、認証、Cookie、利用者環境に依存すると不安定になるため、mockとfixture中心で検証する。

## Scope

* Target crate: `cargo-ac`
* Target files: `crates/cargo-ac/tests/download_command.rs`, `crates/cargo-ac/tests/new_download_command.rs`、必要なfixture HTML
* 一時workspaceを使って `cargo ac download <contest>` の成功経路を検証する
* `cargo ac new <contest> --download` がproject生成後に同じdownload workflowを使うことを検証する
* parse failure時のdebug HTML path表示を検証する
* AtCoder実サイト、認証、Cookie、利用者の既存workspaceに依存しないtest構成にする

## Requirements

* Depends on: `Implement cargo ac download <contest>`, `Implement cargo ac new <contest> --download`
* fixture HTMLやlocal mock HTTP layerを使い、network accessを必須にしない
* 日本語UIと英語UIのsample抽出はparser fixture testと重複しすぎない範囲でworkflow接続を確認する
* temporary workspaceはtest終了後に残さない
* failure caseはdebug HTML保存とCLI errorのpath表示を検証する

## Acceptance criteria

* [ ] `cargo ac download <contest>` がmock HTTPからtask listとtask pageを取得する経路を検証する
* [ ] sample filesが `testcases/<task>/sample-N.in/out` に作成されることを検証する
* [ ] `ac.toml` にtask metadataが反映されることを検証する
* [ ] `cargo ac new <contest> --download` がproject生成後にdownload workflowを実行することを検証する
* [ ] parse failure時にdebug HTMLが保存され、CLI outputまたはerrorにpathが出ることを検証する
* [ ] test suiteがAtCoder実サイト、認証、Cookie、利用者環境に依存しない
* [ ] login、CSRF token、提出、watchの検証が含まれていない

## Non-goals

* AtCoder実サイトへのCI access
* parser unit testの全fixture組合せの重複検証
* login、Cookie、CSRF token、submit、watch
* performance benchmark
* production codeの不要なrefactor

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p cargo-ac --test download_command
cargo test -p cargo-ac --test new_download_command
cargo test --all
```

## Notes for implementation

実サイトを使った確認が必要な場合はManual verificationとしてPR本文に分離する。CIで動かすintegration testはfixture HTMLとmock HTTP layerを使い、AtCoderのページ変更やネットワーク状態でflakyにならないようにする。
