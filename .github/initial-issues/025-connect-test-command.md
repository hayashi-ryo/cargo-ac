# Connect `cargo ac test` to the local test runner

## Summary

既存のcore runnerとCLI formatterを `cargo ac test <task>`、`all`、`--release` から利用可能にする。

## Planning metadata

* Labels: `type: feature`, `area: cli`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-test-command`
* Commit: `feat(cli): connect test command`

## Background

現在のCLIには `test <task>` placeholderだけがある。runner、比較、表示を再設計せず、command引数と終了statusを既存componentへ接続する必要がある。

## Scope

* Target crate: `cargo-ac`
* Target files: `crates/cargo-ac/src/cli.rs`, `crates/cargo-ac/src/commands/test.rs`、必要なCLI integration test
* `cargo ac test <task>` で1 task、`cargo ac test all` で設定済み全taskを実行する
* `--release` をcore runnerのrelease profileへ渡す
* `ac.toml` 読込、task解決、testcase取得、runner呼出し、formatter呼出し、process exit statusを接続する

## Requirements

* Depends on: testcase discovery、pair validation、task runner、output comparison、result output、WA diffの各Issue
* `ac-core` から `clap` へ依存させない
* task名は `ac.toml` の `bin_name` に完全一致させ、未知taskを明示的に拒否する
* 外部入力、config/file I/O、子process failureをpanicさせない

## Acceptance criteria

* [ ] `cargo ac test <task>` が指定taskの全testcaseを実行する
* [ ] `cargo ac test all` が `ac.toml` の全taskを設定順に実行する
* [ ] `cargo ac test <task> --release` と `all --release` がrelease profileを使う
* [ ] 無効なtaskが利用可能なtaskを確認できるCLI errorになる
* [ ] testcaseが0件のtaskは成功扱いにせず、case不在を示すerrorまたはfailure statusになる
* [ ] 全caseがACの場合だけ終了status 0、WA、RE、TLEまたはsetup errorが1件でもあればnon-zeroになる
* [ ] CLI parsingと主要dispatch経路のtestがネットワークなしで通る

## Non-goals

* runner、comparison、result model、WA diffの新規設計
* `test` 以外のcommand変更または短縮alias追加
* parallel execution、watch mode、custom timeout CLI option
* AtCoderアクセス、認証、Cookie、提出

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p cargo-ac
cargo test --all
```

## Notes for implementation

既存の `Command::Test { task }` とhandler境界を維持し、`release: bool` の追加に限定する。`all` はreserved selectorとして扱い、task名との衝突時の規則をtestで固定する。
