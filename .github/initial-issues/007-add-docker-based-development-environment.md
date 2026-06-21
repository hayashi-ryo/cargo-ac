# Dockerベースの開発環境を追加する

## 概要

`cargo-ac` をホストOSに依存せずにビルド・テスト・実行確認できるよう、Dockerベースの開発環境を追加する。

特に、macOS環境でローカル生成されたRustバイナリが正常に起動しない場合でも、Linuxコンテナ上で安定して検証できるようにする。

## 背景

ローカルmacOS環境で、生成されたRustバイナリが起動後に停止する事象が発生した。

確認された事象は以下のとおり。

* `cargo run -p cargo-ac -- --help` が、生成バイナリ起動後に停止する
* `/tmp` に作成した最小Rustプロジェクトでも、`cargo run` 後に停止する
* `rustc /tmp/hello.rs -o /tmp/hello-rust` で直接生成したバイナリも、出力前に停止する
* `sample` の結果、Rustの `main()` に到達する前の `_dyld_start` 付近で停止しているように見える
* ad-hoc署名を行っても改善しない

このため、現時点では `cargo-ac` の実装問題ではなく、ローカルmacOS上のRust実行環境、dyld、Gatekeeper、署名、またはtoolchain周辺の問題と考えられる。

今後の開発を止めないため、Docker上で標準的な検証を行える開発環境を整備する。

## 対応範囲

Dockerベースで `cargo-ac` の開発・検証を行うための最小構成を追加する。

このIssueでは、以下を対象とする。

* Dockerベースの開発環境定義を追加する
* Docker環境に入るための手順を追加する
* Docker環境内で標準検証コマンドを実行する手順を追加する
* 必要に応じて、Docker環境を起動する補助スクリプトを追加する
* Docker環境内で `cargo-ac` のhelp表示確認ができるようにする

## 要件

* 公式Rustイメージ、またはそれに準ずるシンプルなRustベースイメージを利用する

* Rustバージョンは、既存のtoolchain方針がある場合はそれに従う

* 明確な方針がない場合は、Rust 1.85以上を利用する

* Docker環境内で以下を実行できるようにする

  * `cargo fmt --all -- --check`
  * `cargo clippy --all-targets --all-features -- -D warnings`
  * `cargo test --all`
  * `cargo run -p cargo-ac -- --help`
  * `cargo run -p cargo-ac -- new --help`
  * `cargo run -p cargo-ac -- env --help`
  * `cargo run -p cargo-ac -- lang --help`

* Docker環境の使い方を `docs/` 配下、またはREADMEから辿れる場所に記載する

* 補助スクリプトを追加する場合は `scripts/` 配下に配置する

* 初期版では、複雑なマルチコンテナ構成にはしない

* `cargo-ac` のCLI挙動は変更しない

## 想定される追加ファイル

実装方法はCodexの判断に任せるが、例えば以下のような構成が考えられる。

* `.devcontainer/devcontainer.json`
* `.devcontainer/Dockerfile`
* `scripts/dev-docker.sh`
* `docs/development-environment.md`

より単純な構成でAcceptance criteriaを満たせる場合は、その構成を優先してよい。

## 受け入れ条件

* [ ] Dockerベースの開発環境が追加されている
* [ ] 開発者がDocker環境に入る手順がドキュメント化されている
* [ ] Docker環境内で `cargo fmt --all -- --check` が実行できる
* [ ] Docker環境内で `cargo clippy --all-targets --all-features -- -D warnings` が実行できる
* [ ] Docker環境内で `cargo test --all` が実行できる
* [ ] Docker環境内で `cargo run -p cargo-ac -- --help` が実行できる
* [ ] Docker環境内で `cargo run -p cargo-ac -- new --help` が実行できる
* [ ] Docker環境内で `cargo run -p cargo-ac -- env --help` が実行できる
* [ ] Docker環境内で `cargo run -p cargo-ac -- lang --help` が実行できる
* [ ] 一部macOS環境でDocker検証が有用になる理由がドキュメントに記載されている
* [ ] このIssueでは `cargo-ac` のCLI挙動を変更していない

## 対象外

* 新しい `cargo-ac` コマンドの追加
* 既存のCLI引数構造の変更
* placeholder handlerの追加
* エラーハンドリング設計の追加
* AtCoderへのネットワークアクセス
* ログイン、提出、問題取得、ローカルテスト実行機能の実装
* GitHub Actions CIの置き換え
* すべての開発者にDocker利用を必須化すること
* 複雑なマルチコンテナ構成の導入

## 検証方法

Docker環境内で以下を実行する。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all

cargo run -p cargo-ac -- --help
cargo run -p cargo-ac -- new --help
cargo run -p cargo-ac -- env --help
cargo run -p cargo-ac -- lang --help
```

## 実装メモ

初期版では、できるだけ小さく始める。

リポジトリをコンテナ内にマウントし、Rust公式イメージ上で通常の `cargo` コマンドを実行できれば十分とする。

このIssueの目的は、ローカルmacOS環境に依存せず、以降のCLI開発を継続できる検証経路を確保することである。
