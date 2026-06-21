# Summary

ネットワークを使わず `cargo ac new <contest>` でローカルcontest workspaceを生成できるようにする。

## Planning metadata

* Labels: `type: feature`, `area: cli`, `area: core`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-implement-new-command`
* Commit: `feat(new): generate contest workspace`

## Background

Phase 4の各生成componentをCLIから呼び出し、初期MVPの最初の利用可能なコマンドとして統合する必要がある。

## Scope

* `new` placeholder handlerをworkspace生成処理へ接続する
* contest IDから生成先を決定する
* 初期task一覧を `a` から `f` とする
* `Cargo.toml`、`ac.toml`、source、testcase directoryを一度の操作で生成する
* 成功結果と失敗理由をCLI向けに表示する

## Requirements

* core処理は `ac-core`、表示は `cargo-ac` に置く
* 生成先が存在する場合は上書きせず非ゼロで終了する
* 生成前に入力と衝突を検証する
* panicせずResultで失敗を返す
* `--force` や対話promptを追加しない
* AtCoderへのネットワークアクセスを行わない

## Acceptance criteria

* [ ] `cargo ac new abc400` が正常終了する
* [ ] `abc400/Cargo.toml` と `abc400/ac.toml` が生成される
* [ ] `abc400/src/bin/a.rs` から `f.rs` が生成される
* [ ] `abc400/testcases/a/` から `f/` が生成される
* [ ] 生成workspaceで `cargo check --bins` が成功する
* [ ] 同名pathが存在する場合は上書きせず非ゼロ終了する
* [ ] AtCoderへのHTTPリクエストを行わない

## Non-goals

* AtCoderからcontestやtaskを取得すること
* `--force`、`--template`、task数指定option
* sample testcaseの取得
* 生成後の自動editor起動
* 複数OJ対応

## Verification

Docker環境内の一時ディレクトリで以下を実行する。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo run -p cargo-ac -- new abc400
cargo check --manifest-path abc400/Cargo.toml --bins
```

同じ `new` コマンドを再実行し、既存workspaceが変更されず非ゼロ終了することも確認する。

## Notes for implementation

初期task一覧を `a`〜`f` とする前提をPR本文に記載する。task一覧のAtCoderからの取得はPhase 6へ残す。
