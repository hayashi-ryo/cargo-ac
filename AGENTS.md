# AGENTS.md

このドキュメントは、AIエージェントがこのリポジトリで作業する際に守るべきルールをまとめたものです。

このリポジトリでは、RustでAtCoderに参加する人のためのCLIツール `cargo-ac` を開発します。

## プロジェクトの目的

`cargo-ac` は、Rustを書く人にとって気持ちいいAtCoder CLIを目指すプロジェクトです。

主に以下を重視します。

* Cargoサブコマンドとして自然に使えること
* AtCoder用Rustプロジェクトを簡単に生成できること
* ローカルでサンプルテストを実行しやすいこと
* AtCoderから問題・サンプルを取得できること
* AtCoderへ安全に提出できること
* 提出結果をCLI上で確認できること
* Rustの提出用language_idを固定値にしないこと
* AtCoderのHTML構造変更に気づきやすく、修正しやすいこと
* AtCoderのRust実行環境に追従しやすいこと

## 基本ルール

* 1つのIssueにつき、1つのPull Requestで対応してください。
* Issueで指定された範囲外の機能は実装しないでください。
* 関係のないリファクタリングは行わないでください。
* public APIは必要最小限にしてください。
* 判断に迷う場合は、最も単純な実装を選び、Pull Request本文に前提を書いてください。
* 実装方針がIssueから読み取れない場合は、大きな設計変更を勝手に行わないでください。
* 認証情報、Cookie、CSRF token、パスワードをログ出力しないでください。
* AtCoderに対して過剰なアクセスを行わないでください。
* Rustのlanguage_idを固定値として実装しないでください。

## 初期開発の対象範囲

初期開発では、以下の機能を優先します。

| 機能 | 内容 |
| ----------------------- | ----------------------- |
| `cargo ac new` | AtCoder用Rustプロジェクトを生成する |
| `cargo ac download` | AtCoderから問題・サンプルを取得する |
| `cargo ac test` | ローカルでサンプルテストを実行する |
| `cargo ac addcase` | 自作テストケースを追加する |
| `cargo ac login` | AtCoderにログインする |
| `cargo ac submit` | AtCoderへコードを提出する |
| `cargo ac watch` | 提出結果を監視する |
| `cargo ac doctor` | ローカル環境や設定を診断する |
| `cargo ac selfcheck` | AtCoder側の構造変更を検知する |
| `cargo ac env` | AtCoder Rust環境情報を扱う |
| `cargo ac lang refresh` | Rustのlanguage_idを更新する |

## 初期開発でやらないこと

初期開発では、以下は実装しないでください。

| 項目 | 理由 |
| ------------- | --------------------------- |
| 複数OJ対応 | まずはAtCoderとRustに集中するため |
| GUI | CLI体験の確立を優先するため |
| AI解説生成 | AtCoderのコンテスト中ルールとの関係が難しいため |
| VSCode拡張 | CLIが安定してから検討するため |
| ブラウザ自動操作 | HTTP/HTMLベースの実装を優先するため |
| 高度な統計機能 | 初期MVPの範囲外とするため |
| コンテスト中の外部AI連携 | ルール・安全面で慎重に扱う必要があるため |

## 想定crate構成

初期段階では、以下の2crate構成を想定します。

| crate | 役割 |
| ---------- | ------------------------- |
| `cargo-ac` | CLI本体 |
| `ac-core` | 設定、テスト実行、AtCoder連携などの共通処理 |

将来的に実装が大きくなった場合は、以下のcrate分離を検討します。

| crate | 役割 |
| --------- | ------------------- |
| `ac-html` | AtCoder HTML parser |
| `ac-env` | AtCoder Rust環境情報管理 |

## crate境界

### `cargo-ac`

`cargo-ac` はCLI層を担当します。

主な責務は以下です。

* コマンドライン引数の解析
* ユーザー向け出力
* 対話プロンプト
* コマンドごとの処理呼び出し
* CLI境界でのエラーハンドリング

CLI専用の依存関係は、基本的に `cargo-ac` に閉じ込めてください。

例:

* `clap`
* `dialoguer`
* `console`
* `indicatif`

### `ac-core`

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

`ac-core` はCLIに依存しないようにしてください。

特に、`ac-core` から `clap` に依存しないでください。

## 依存方向

依存方向は以下に限定します。

```text
cargo-ac
   ↓
ac-core
```

`ac-core` から `cargo-ac` へ依存してはいけません。

## エラーハンドリング方針

### 基本方針

* ライブラリ側では、可能な範囲で型付きエラーを返してください。
* CLI境界では、ユーザーに分かりやすいメッセージへ変換してください。
* 外部入力、HTML構造、ファイルI/O、ネットワーク失敗ではpanicしないでください。
* AtCoder側の構造変更が疑われる場合は、原因調査しやすい情報を出してください。
* parser失敗時は、必要に応じてdebug HTMLの保存場所を表示してください。
* ログイン切れが疑われる場合は、再ログインを促してください。

### `ac-core` 側

`ac-core` では、ユーザー入力や外部サービスに起因する失敗を通常のエラーとして扱ってください。

例:

* 設定ファイルが存在しない
* `ac.toml` の形式が不正
* テストケースの `.in` / `.out` の対応が崩れている
* AtCoderへのHTTPリクエストに失敗した
* AtCoderのHTML parserに失敗した
* ログインセッションが切れている
* Rustのlanguage_idを解決できない

これらはpanicではなく、呼び出し元へエラーとして返してください。

### `cargo-ac` 側

`cargo-ac` では、`ac-core` から返されたエラーをCLI向けに表示してください。

表示時には以下を意識してください。

* 何が失敗したか分かること
* ユーザーが次に何をすればよいか分かること
* parser失敗時は、debug HTMLの保存場所が分かること
* ログイン切れが疑われる場合は、再ログインを促すこと
* 認証情報やCookieなどの秘密情報を表示しないこと

## AtCoder連携時のルール

AtCoder連携では、以下を守ってください。

* AtCoderのlanguage_idを固定値にしないでください。
* 提出ページからRustのlanguage_idを解決してください。
* AtCoderへの過剰アクセスを避けてください。
* 提出結果の監視では短すぎる間隔でポーリングしないでください。
* HTML parserはfixture testを重視してください。
* parser失敗時のみdebug HTMLを保存する方針にしてください。
* debug HTMLに認証情報が含まれないよう注意してください。
* AtCoderの利用規約やコンテストルールに反する使い方を推奨しないでください。

## HTML parser方針

AtCoderのHTML構造は変更される可能性があるため、parserは壊れる前提で設計してください。

以下を意識してください。

* parser処理を他の処理から分離する
* 日本語UIと英語UIの両方を考慮する
* `入力例` / `出力例` に対応する
* `Sample Input` / `Sample Output` に対応する
* `pre` 要素からサンプル本文を取得する
* parser fixture testを追加する
* parser失敗時にdebug HTMLを保存できるようにする
* debug HTMLには認証情報が含まれないよう注意する

## ローカルテスト実行方針

ローカルテストでは、AtCoderのサンプルを快適に確認できることを重視します。

以下を意識してください。

* `cargo ac test a` のように短く実行できる
* `src/bin/a.rs` と `testcases/a/` を対応させる
* `.in` / `.out` のペアをテストケースとして扱う
* 標準入力に `.in` の内容を渡す
* 標準出力と `.out` の内容を比較する
* 末尾改行や空白差分の扱いを整理する
* WA時には差分を表示する
* RE時にはstderrやexit statusを表示する
* TLE時にはtimeoutとして表示する

## 提出機能の方針

提出機能では、誤提出や過剰アクセスを避けることを重視します。

以下を意識してください。

* 提出前に対象ファイルの存在を確認する
* 提出前に `cargo check` を実行する
* 提出前にローカルサンプルテストを実行する
* Rustのlanguage_idを固定値なしで解決する
* 提出前に確認プロンプトを表示する
* `--yes` 指定時のみ確認を省略する
* `--no-test` 指定時のみローカルテストを省略する
* 提出後の結果監視では適切な間隔でポーリングする
* 提出連打を防ぐ仕組みを検討する

## セキュリティ方針

AtCoderの認証情報やセッション情報を扱うため、以下を守ってください。

* パスワードをログ出力しない
* Cookieをログ出力しない
* CSRF tokenをログ出力しない
* debug HTMLに認証情報が含まれないよう注意する
* セッション情報の保存場所と削除方法をREADMEに明記する
* セッションファイルを利用する場合は、ファイル権限に注意する
* ログやエラーメッセージに秘密情報を含めない

## Branch命名規則

作業ブランチは、Issue単位で作成してください。

形式は以下とします。

```text
<type>/<issue-number>-<short-description>
```

例:

```text
feature/12-cli-skeleton
feature/18-new-command
feature/24-test-runner
fix/31-sample-parser-ja
docs/9-roadmap
chore/5-github-actions
```

### type候補

| type | 用途 |
| ---------- | ----------- |
| `feature` | 機能追加 |
| `fix` | バグ修正 |
| `docs` | ドキュメント |
| `chore` | 設定・雑務 |
| `refactor` | 振る舞いを変えない整理 |
| `test` | テスト追加・修正 |
| `ci` | CI変更 |

## Commit message規則

コミットメッセージは、Conventional Commits形式にしてください。

形式は以下とします。

```text
<type>(<scope>): <summary>
```

例:

```text
feat(cli): add command skeleton
feat(new): generate contest workspace
feat(test): run sample testcases
fix(parser): handle Japanese sample labels
docs: add development roadmap
chore(ci): add rust checks
refactor(core): split testcase loader
test(runner): add WA case test
```

### commit type候補

| type | 用途 |
| ---------- | ----------- |
| `feat` | 機能追加 |
| `fix` | バグ修正 |
| `docs` | ドキュメント |
| `chore` | 雑務・設定 |
| `refactor` | 振る舞いを変えない整理 |
| `test` | テスト追加・修正 |
| `ci` | CI変更 |

### scope候補

| scope | 対象 |
| ---------- | ----------- |
| `cli` | CLI本体 |
| `core` | core crate |
| `config` | 設定ファイル |
| `runner` | ローカル実行 |
| `testcase` | テストケース管理 |
| `parser` | HTML parser |
| `auth` | 認証 |
| `submit` | 提出 |
| `env` | Rust環境情報 |
| `doctor` | 診断機能 |
| `docs` | ドキュメント |
| `ci` | CI |

## Pull Requestルール

Pull Requestでは、以下を守ってください。

* 1つのPull Requestは1つのIssueに対応させてください。
* Issue範囲外の変更を含めないでください。
* 不要なリファクタリングを含めないでください。
* 変更内容を簡潔に説明してください。
* 実行した確認内容を書いてください。
* 未対応範囲や前提があれば書いてください。
* 認証情報、Cookie、CSRF token、パスワードを含めないでください。

Pull Request本文には、少なくとも以下を含めてください。

```markdown
## Summary

変更内容を簡潔に書く。

## Related issue

Closes #

## Changes

- 
- 

## Verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all`

## Scope control

- [ ] Issue範囲外の変更を含めていない
- [ ] 不要なリファクタリングを含めていない
- [ ] secrets / credentials を含めていない

## Notes

前提、判断理由、未対応範囲があれば書く。
```

## テスト・検証

作業完了前に、可能な範囲で以下を実行してください。

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

まだRust workspaceが存在しない段階では、該当するチェックのみ実行してください。

実行できない場合は、Pull Request本文に理由を書いてください。

## Issue対応時の進め方

Issue対応時は、以下の順番で進めてください。

1. Issue本文を読む
2. ScopeとNon-goalsを確認する
3. 関連する既存コードを確認する
4. 最小限の実装方針を決める
5. 必要なテストを追加または更新する
6. 実装する
7. `cargo fmt` を実行する
8. `cargo clippy` を実行する
9. `cargo test` を実行する
10. Pull Request本文に変更内容と確認結果を書く

## Issue作業完了時の報告

Issue実装完了後、最終報告の末尾に以下の形式で完了スクリプトの実行コマンドを記載してください。

```bash
./scripts/finish-codex-issue.sh \
  <issue-number> \
  "<conventional-commit-message>"
```

Codexはこのスクリプトを自動実行せず、ユーザーが確認して実行できるコマンドとして出力してください。

コミットメッセージは、IssueのPlanning metadataとこのドキュメントのConventional Commits規則に従ってください。

## 実装時に避けること

以下は避けてください。

* Issue範囲外の機能追加
* 大規模な一括リファクタリング
* unrelated fileの変更
* language_idのハードコード
* 認証情報のログ出力
* 過剰なAtCoderアクセス
* ライブラリコードでの安易なpanic
* parser処理とHTTP処理の密結合
* CLI依存を `ac-core` に入れること
* 作業完了後の報告では、ローカル絶対パスで記載すること

## Codex-ready Issueの条件

Codexに依頼するIssueは、以下の条件を満たしている必要があります。

* Summaryが明確である
* Scopeが明確である
* Requirementsが箇条書きで整理されている
* Acceptance criteriaがチェックボックス形式で書かれている
* Non-goalsが明記されている
* 対象crateまたは対象ファイルの見当がついている
* 依存するIssueがある場合は明記されている
* `agent: codex-ready` ラベルが付いている
* 認証、Cookie、提出、AtCoderアクセスを含む場合は `agent: needs-review-carefully` も付ける

Codex-readyではないIssueには、`agent: needs-human-design` を付けます。

## ライセンス

このプロジェクトは `MIT OR Apache-2.0` のデュアルライセンスです。

新規に追加するファイルやコードも、このライセンス方針に従ってください。
