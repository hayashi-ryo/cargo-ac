# Summary

ローカルcontest workspace用の最小 `Cargo.toml` を生成する処理を追加する。

## Planning metadata

* Labels: `type: feature`, `area: core`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-generate-contest-manifest`
* Commit: `feat(core): generate contest Cargo.toml`

## Background

生成したworkspaceで通常のCargoコマンドを利用するには、妥当で再現可能なmanifestが必要である。

## Scope

* contest IDからpackage名を組み立てる
* editionを明示した最小 `Cargo.toml` を生成する
* 生成内容を独立してテストする
* 生成したmanifestがCargo metadataで解釈できることを確認する

## Requirements

* manifest生成は `ac-core` に実装する
* package名に利用できないcontest IDは通常のエラーにする
* Rustのlanguage_idを含めない
* AtCoderのcrate一覧をハードコードしない
* ユーザー環境のグローバルCargo設定を変更しない
* AtCoderへのネットワークアクセスを行わない

## Acceptance criteria

* [ ] contest workspaceに `Cargo.toml` を生成できる
* [ ] package名とeditionが設定されている
* [ ] 生成結果が有効なCargo manifestである
* [ ] 不正なpackage名をエラーとして扱う
* [ ] 正常系と異常系のテストが存在する

## Non-goals

* AtCoderで利用可能な全crateの追加
* Rust toolchainファイルの生成
* source templateの生成
* `cargo ac new` のCLI統合
* AtCoder Rust環境の取得

## Verification

Docker環境内で以下を実行する。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core
cargo test --all
```

生成fixtureに対して、ネットワークを使わず `cargo metadata --no-deps` を実行する。

## Notes for implementation

初期版は標準ライブラリだけでビルド可能なmanifestに限定する。依存crate管理はAtCoder環境情報を扱う後続Issueへ分離する。
