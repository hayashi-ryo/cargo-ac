# アーキテクチャ

## 目的

このドキュメントは、`cargo-ac` の設計方針とcrate構成を整理するためのものです。

本プロジェクトでは、CLIとしての使いやすさだけでなく、AtCoder側の仕様変更やRust実行環境の更新に追従しやすい構成を重視します。

## 基本方針

| 方針                     | 内容                                           |
| ---------------------- | -------------------------------------------- |
| Cargo-native           | Cargoサブコマンドとして自然に使えるCLIを目指す                  |
| AtCoder-aware          | AtCoderのHTML構造、ログイン、提出、言語ID変更を考慮する           |
| Rust-environment-aware | AtCoderのRustバージョン、edition、利用可能crateを扱えるようにする |
| Maintenance-first      | 仕様変更に気づきやすく、修正しやすい構成にする                      |
| Codex-friendly         | 小さなIssue単位で安全に開発を進められる運用にする                  |

## 初期crate構成

初期段階では、以下の2crate構成を想定します。

| crate      | 役割                        |
| ---------- | ------------------------- |
| `cargo-ac` | CLI本体                     |
| `ac-core`  | 設定、テスト実行、AtCoder連携などの共通処理 |

## 将来的なcrate分離候補

実装が大きくなった場合、以下のcrate分離を検討します。

| crate     | 役割                  |
| --------- | ------------------- |
| `ac-html` | AtCoder HTML parser |
| `ac-env`  | AtCoder Rust環境情報管理  |

## crate境界の方針

### cargo-ac

`cargo-ac` はCLI層を担当します。

主な責務は以下です。

* コマンドライン引数の解析
* ユーザー向け出力
* 対話プロンプト
* CLI境界でのエラーハンドリング
* コマンドごとの処理呼び出し

CLI専用の依存関係は、基本的に `cargo-ac` 側に閉じ込めます。

たとえば、以下のようなcrateは `cargo-ac` 側で利用する想定です。

| crate       | 用途               |
| ----------- | ---------------- |
| `clap`      | コマンドライン引数解析      |
| `anyhow`    | CLI境界でのエラーハンドリング |
| `dialoguer` | 対話プロンプト          |
| `console`   | CLI出力の装飾         |
| `indicatif` | progress表示       |

### ac-core

`ac-core` は中核処理を担当します。

主な責務は以下です。

* 設定ファイルの読み書き
* テストケース管理
* ローカルテスト実行
* AtCoder HTTP client
* AtCoder HTML parser
* ログイン・セッション管理
* 提出処理
* 提出結果監視
* 環境診断処理
* AtCoder Rust環境情報の管理

`ac-core` はCLIに依存しないようにします。

特に、`clap` などのCLI専用crateには依存しない方針とします。

## 主要ディレクトリ構成

```text
cargo-ac/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── .gitignore
├── AGENTS.md
├── .devcontainer/
│   └── Dockerfile
├── docs/
│   ├── ROADMAP.md
│   ├── ARCHITECTURE.md
│   ├── development-environment.md
│   └── tasks.md
├── crates/
│   ├── cargo-ac/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── cli.rs
│   │   │   ├── error.rs
│   │   │   └── commands/
│   │   │       ├── mod.rs
│   │   │       ├── new.rs
│   │   │       ├── test.rs
│   │   │       ├── download.rs
│   │   │       ├── login.rs
│   │   │       ├── submit.rs
│   │   │       ├── watch.rs
│   │   │       ├── doctor.rs
│   │   │       ├── selfcheck.rs
│   │   │       ├── env.rs
│   │   │       └── lang.rs
│   │   └── tests/
│   │       └── new_command.rs
│   └── ac-core/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── config.rs
│           ├── manifest.rs
│           ├── task_layout.rs
│           └── workspace.rs
├── scripts/
│   ├── dev-docker.sh
│   ├── fetch-issue.sh
│   ├── finish-codex-issue.sh
│   └── prepare-codex-issue.sh
└── .github/
    ├── workflows/
    │   └── ci.yml
    ├── ISSUE_TEMPLATE/
    │   ├── feature.yml
    │   └── bug.yml
    └── pull_request_template.md
```

このtreeはPhase 4完了時点の構成を示す。test runner、AtCoder client、認証、提出などのmoduleは、対応する後続Phaseで必要になった時点で追加する。

## crate依存関係の方針

依存関係は以下の方向に限定します。

```text
cargo-ac
   ↓
ac-core
```

`ac-core` から `cargo-ac` へ依存してはいけません。

また、`ac-core` はCLIに依存しないため、将来的に以下の用途でも再利用しやすくなります。

* 別CLIへの流用
* テスト用ライブラリとしての利用
* 将来的なGUIやエディタ拡張からの利用
* parserや環境情報管理のcrate分離

## エラーハンドリング方針

### 基本方針

* ライブラリ側では、可能な範囲で型付きエラーを返す
* CLI境界では、ユーザーに分かりやすいメッセージへ変換する
* 外部入力、HTML構造、ファイルI/O、ネットワーク失敗ではpanicしない
* AtCoder側の構造変更が疑われる場合は、原因調査しやすい情報を出す

### ac-core側

`ac-core` では、ユーザー入力や外部サービスに起因する失敗を通常のエラーとして扱います。

例:

* 設定ファイルが存在しない
* `ac.toml` の形式が不正
* テストケースの `.in` / `.out` の対応が崩れている
* AtCoderへのHTTPリクエストに失敗した
* AtCoderのHTML parserに失敗した
* ログインセッションが切れている
* Rustのlanguage_idを解決できない

これらはpanicではなく、呼び出し元へエラーとして返します。

### cargo-ac側

`cargo-ac` では、`ac-core` から返されたエラーをCLI向けに表示します。

表示時には以下を意識します。

* 何が失敗したか分かること
* ユーザーが次に何をすればよいか分かること
* parser失敗時は、debug HTMLの保存場所が分かること
* ログイン切れが疑われる場合は、再ログインを促すこと
* 認証情報やCookieなどの秘密情報を表示しないこと

## 設定ファイル方針

### ac.toml

AtCoder用プロジェクトには、`ac.toml` を配置します。

`ac.toml` では、以下のような情報を管理します。

| 項目                 | 内容             |
| ------------------ | -------------- |
| contest ID         | `abc400` など    |
| source directory   | `src/bin` など   |
| testcase directory | `testcases` など |
| language name      | `rust`         |
| Rust edition       | `2021` など      |
| task ID            | `abc400_a` など  |
| bin名               | `a` など         |

`ac.toml` は、ローカルプロジェクト単位の設定を扱います。

### グローバル設定

ログインセッション、language_id cache、AtCoder Rust環境情報などは、ユーザー単位のグローバル設定として扱います。

保存場所はOSごとの標準ディレクトリを利用する方針です。

例:

| 種別             | 内容               |
| -------------- | ---------------- |
| session        | AtCoderログインセッション |
| language cache | Rustのlanguage_id |
| env data       | AtCoder Rust環境情報 |
| cache          | 取得済みHTMLや問題情報    |

## AtCoder連携方針

AtCoder連携では、以下を重視します。

* language_idを固定値にしない
* 提出ページからRustのlanguage_idを解決する
* AtCoderへの過剰アクセスを避ける
* 提出結果の監視では短すぎる間隔でポーリングしない
* HTML parserはfixture testを重視する
* parser失敗時はdebug HTML保存を検討する
* AtCoderの利用規約やコンテストルールに反する使い方を推奨しない

## HTML parser方針

AtCoderのHTML構造は変更される可能性があるため、parserは壊れる前提で設計します。

### 方針

* parser処理を `ac-core` 内で分離する
* 可能なら将来的に `ac-html` crateとして分離できるようにする
* 日本語UIと英語UIの両方を考慮する
* `入力例` / `出力例` に対応する
* `Sample Input` / `Sample Output` に対応する
* parser fixture testを用意する
* parser失敗時にdebug HTMLを保存できるようにする
* debug HTMLには認証情報が含まれないよう注意する

## ローカルテスト実行方針

ローカルテスト実行では、AtCoderのサンプルテストを快適に確認できることを重視します。

### 方針

* `cargo ac test a` のように短く実行できる
* `src/bin/a.rs` と `testcases/a/` を対応させる
* `.in` / `.out` のペアをテストケースとして扱う
* 標準入力に `.in` の内容を渡す
* 標準出力と `.out` の内容を比較する
* 末尾改行や空白差分の扱いを整理する
* WA時には差分を表示する
* RE時にはstderrやexit statusを表示する
* TLE時にはtimeoutとして表示する

## 提出方針

提出機能では、誤提出や過剰アクセスを避けることを重視します。

### 方針

* 提出前に対象ファイルの存在を確認する
* 提出前に `cargo check` を実行する
* 提出前にローカルサンプルテストを実行する
* Rustのlanguage_idを固定値なしで解決する
* 提出前に確認プロンプトを表示する
* `--yes` 指定時のみ確認を省略する
* `--no-test` 指定時のみローカルテストを省略する
* 提出後の結果監視では適切な間隔でポーリングする
* 提出連打を防ぐ仕組みを検討する

## 環境情報管理方針

AtCoderのRust環境は更新される可能性があるため、ツール本体と環境定義を分離します。

### 管理する情報

| 項目           | 内容                   |
| ------------ | -------------------- |
| Rust version | AtCoderのRustバージョン    |
| Rust edition | AtCoderで使うedition    |
| crates       | AtCoderで利用可能なcrate一覧 |
| updated_at   | 環境定義の更新日             |

### 方針

* 初期段階ではstatic JSONとして管理する
* 将来的にはGitHub上の環境定義ファイルから更新できるようにする
* `cargo ac env show` で現在の環境情報を表示する
* `cargo ac env update` で環境情報を更新する
* `cargo ac doctor` でローカル環境との差分を表示する

## セキュリティ方針

AtCoderの認証情報やセッション情報を扱うため、以下を守ります。

* パスワードをログ出力しない
* Cookieをログ出力しない
* CSRF tokenをログ出力しない
* debug HTMLに認証情報が含まれないよう注意する
* セッション情報の保存場所と削除方法をREADMEに明記する
* セッションファイルを利用する場合は、ファイル権限に注意する

## 初期段階でやらないこと

初期開発では、スコープを広げすぎないために以下は対象外とします。

| 項目       | 理由                          |
| -------- | --------------------------- |
| 複数OJ対応   | まずはAtCoderとRustに集中するため      |
| GUI      | CLIとしての体験を優先するため            |
| AI解説生成   | AtCoderのコンテスト中ルールとの関係が難しいため |
| VSCode拡張 | CLIが安定してから検討するため            |
| ブラウザ自動操作 | HTTP/HTMLベースの実装を優先するため      |
| 高度な統計機能  | 初期MVPの範囲外とするため              |

## 将来的に検討すること

基本機能が安定した後、以下を検討します。

| 項目                | 内容                     |
| ----------------- | ---------------------- |
| `ac-html` crate分離 | HTML parserを独立させる      |
| `ac-env` crate分離  | AtCoder Rust環境情報を独立させる |
| shell completion  | CLI補完を提供する             |
| VSCode task生成     | VSCodeからテスト・提出しやすくする   |
| 提出履歴表示            | 過去提出の確認をしやすくする         |
| 問題文markdown保存     | 問題文をローカルで参照しやすくする      |
| GitHub release自動化 | リリース作業を自動化する           |
| Windows向け動作確認強化   | Windows利用者向けに安定性を高める   |

## ライセンス

このプロジェクトは `MIT OR Apache-2.0` のデュアルライセンスです。

利用者は、MIT License または Apache License, Version 2.0 のいずれか一方の条件を選択して利用できます。
