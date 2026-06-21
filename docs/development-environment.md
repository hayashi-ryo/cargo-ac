# Docker開発環境

`cargo-ac` は、公式Rustイメージを利用したDocker開発環境を提供しています。Dockerは必須ではありませんが、ホストOSのRust実行環境に問題がある場合にも、Linuxコンテナ上で同じビルド・検証手順を実行できます。

一部のmacOS環境では、ローカルで生成したRustバイナリが `main` の実行前に停止する事象が確認されています。このような場合、Docker環境を利用することでホストのdyld、Gatekeeper、署名、toolchain周辺の影響を切り分けながら開発を継続できます。

## 前提条件

Docker EngineまたはDocker Desktopをインストールし、Dockerデーモンを起動してください。

## Docker環境に入る

リポジトリルートで次のコマンドを実行します。

```bash
./scripts/dev-docker.sh
```

スクリプトは `rust:1.89-slim-bookworm` をベースとする開発イメージをビルドし、リポジトリを `/workspace` にマウントした対話シェルを起動します。ホスト側で編集した内容はコンテナ内に即時反映されます。終了するには `exit` を実行します。

## 標準検証

コンテナ内で次のコマンドを実行します。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all

cargo run -p cargo-ac -- --help
cargo run -p cargo-ac -- new --help
cargo run -p cargo-ac -- env --help
cargo run -p cargo-ac -- lang --help
```

初回実行時は、依存crateの取得とビルドに時間がかかる場合があります。
