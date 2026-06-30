# Implement `cargo ac new <contest> --download`

## Summary

`cargo ac new <contest>` のproject生成後に既存download workflowを実行できる `--download` optionを追加する。

## Planning metadata

* Labels: `type: feature`, `area: cli`, `area: download`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `feature/<issue-number>-new-download-option`
* Commit: `feat(cli): add new --download option`

## Background

利用者はcontest workspace生成とsample取得を一度に行いたい場合がある。`new --download` はdownload処理を複製せず、`cargo ac download <contest>` と同じ内部workflowをproject生成後に呼び出す。

## Scope

* Target crates: `cargo-ac`, `ac-core`
* Target files: `crates/cargo-ac/src/cli.rs`, `crates/cargo-ac/src/commands/new.rs`, core download workflow boundary、必要なtest
* `cargo ac new <contest> --download` のCLI引数を追加する
* project生成成功後に同じdownload workflowを呼び出す
* project生成失敗とdownload失敗を区別して表示する
* download workflowを複製しない

## Requirements

* Depends on: `Implement cargo ac download <contest>`
* Existing components: `cargo ac new <contest>` project generation
* `--download` 未指定時の既存 `cargo ac new <contest>` の挙動を壊さない
* project生成後にdownload workflowを実行する
* エラー時に「project生成前/中の失敗」か「project生成は完了したがdownloadに失敗」かが分かる
* `ac-core` から `clap` に依存させない

## Acceptance criteria

* [ ] `cargo ac new <contest> --download` がparseできる
* [ ] `--download` 未指定時の既存new commandの挙動が維持される
* [ ] project生成後に既存download workflowが呼び出される
* [ ] download処理がnew command内に複製されていない
* [ ] download失敗時にproject生成済みかどうかがCLI outputまたはerrorで分かる
* [ ] login、Cookie、CSRF token、提出、watchが含まれていない

## Non-goals

* download workflow本体の新規設計
* sample parser、contest task list parser、file writerの変更
* `cargo ac download` とは異なる保存layoutの追加
* 認証、Cookie、CSRF token、submit、watch

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p cargo-ac
cargo test --all
cargo run -p cargo-ac -- new abc400 --download
```

## Notes for implementation

既存 `new` のtask source生成とdownload結果のtask metadataが重なる場合は、download workflow側の方針を再利用する。`new` command側で独自のsample保存処理を持たない。
