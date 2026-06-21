# Rust向けAtCoder CLI 開発タスク

## 1. プロジェクト概要

本プロジェクトでは、RustでAtCoderに参加するユーザー向けに、快適な開発・テスト・提出体験を提供するCLIツールを作成する。

コンセプトは以下とする。

> Rustを書く人にとって気持ちいい、現代的なAtCoder CLI

本ツールでは、以下を重視する。

| 観点 | 内容 |
| ---------------------- | ---------------------------- |
| Cargo-native | Cargo標準の開発体験に近い操作感を実現する |
| AtCoder-aware | AtCoderの仕様変更に追従しやすい設計にする |
| Rust-environment-aware | AtCoderのRust実行環境に追従しやすい設計にする |
| Maintenance-first | 壊れにくく、壊れたときに直しやすい構成にする |
| Codex-friendly | Codexに開発を依頼しやすいIssue・PR運用にする |

## 2. 設計方針

本プロジェクトでは、以下の方針を採用する。

| 方針 | 説明 |
| -------------------------- | -------------------------------------------- |
| Cargo-native | `cargo ac ...` のようにCargoサブコマンドとして使えるCLIを目指す |
| AtCoder-aware | AtCoderのHTML構造、ログイン、提出、言語ID変更を意識する |
| Rust-environment-aware | AtCoderのRustバージョン、edition、利用可能crateを扱えるようにする |
| Maintenance-first | HTML変更や環境変更を検知・修正しやすくする |
| Codex-friendly development | Codexが小さなIssue単位で安全に実装できる運用にする |

## 3. 想定コマンド

### 基本コマンド

| コマンド | 目的 |
| --------------------------- | ----------------------- |
| `cargo ac login` | AtCoderへログインする |
| `cargo ac new abc400` | AtCoder用Rustプロジェクトを生成する |
| `cargo ac download abc400` | コンテストの問題・サンプルを取得する |
| `cargo ac open a` | 指定問題をブラウザで開く |
| `cargo ac test a` | ローカルでサンプルテストを実行する |
| `cargo ac addcase a` | 自作テストケースを追加する |
| `cargo ac submit a` | コードを提出する |
| `cargo ac submit a --watch` | 提出後に結果を監視する |
| `cargo ac watch` | 最新提出の結果を監視する |
| `cargo ac doctor` | ローカル環境・プロジェクト構成を診断する |
| `cargo ac selfcheck` | AtCoder側の構造変更を検知する |
| `cargo ac env show` | AtCoder Rust環境情報を表示する |
| `cargo ac env update` | AtCoder Rust環境情報を更新する |
| `cargo ac lang refresh` | Rustのlanguage_idを再解決する |

### 将来的な短縮エイリアス

| 短縮コマンド | 対応する基本コマンド |
| -------------- | ------------------- |
| `cargo ac t a` | `cargo ac test a` |
| `cargo ac s a` | `cargo ac submit a` |

## 4. 全体ロードマップ

| Phase | 内容 |
| -------- | ------------------------------- |
| Phase 0 | コンセプト・仕様整理 |
| Phase 1 | GitHubリポジトリ初期作成 |
| Phase 2 | Codex前提のGitHub運用整備 |
| Phase 3 | CLI骨格作成 |
| Phase 4 | ローカルプロジェクト生成 |
| Phase 5 | ローカルテスト実行 |
| Phase 6 | 問題ダウンロード |
| Phase 7 | ログイン・セッション管理 |
| Phase 8 | 提出・結果監視 |
| Phase 9 | doctor / selfcheck / env update |
| Phase 10 | crates.io公開・継続運用 |

## Phase 0: コンセプト・仕様整理

### 目的

実装前に、ツールの目的・スコープ・非対応範囲を明確にする。

### 方針

| 項目 | 方針 |
| ----------- | ---------------------------------- |
| 対象OJ | 最初はAtCoderのみ |
| 対象言語 | 最初はRustのみ |
| CLI形式 | Cargoサブコマンド形式 |
| 公開形態 | CLI + library crate |
| 初期対応OS | macOS / Linux / Windows |
| 認証情報 | keyring優先、cookie fallback |
| AtCoderアクセス | キャッシュ・適切なポーリング間隔・過剰アクセス防止を重視 |
| HTML変更対応 | parser分離、fixture test、selfcheckで検知 |
| Rust環境情報 | ツール本体とは別に環境定義を管理する |

### 初期MVP

初期MVPでは、以下の機能を対象とする。

| コマンド | 目的 |
| ----------------------- | ----------------------- |
| `cargo ac new` | Rust用AtCoderプロジェクトを生成する |
| `cargo ac download` | 問題ページからサンプルを取得する |
| `cargo ac test` | ローカルでサンプルテストを実行する |
| `cargo ac login` | AtCoderにログインする |
| `cargo ac submit` | コードを提出する |
| `cargo ac watch` | 提出結果を監視する |
| `cargo ac lang refresh` | Rustのlanguage_idを更新する |
| `cargo ac doctor` | ローカル環境を簡易診断する |

### 初期段階ではやらないこと

| 項目 | 理由 |
| -------- | --------------------------- |
| 複数OJ対応 | スコープが広がりすぎるため |
| GUI | CLI体験の確立を優先するため |
| AI解説生成 | AtCoderのコンテスト中ルールとの関係が難しいため |
| VSCode拡張 | CLI安定後に検討するため |
| ブラウザ自動操作 | HTML/HTTPベースの実装を優先するため |
| 高度な統計機能 | 初期MVPの価値から外れるため |

### タスク

* [x] 対象OJをAtCoderのみに決める
* [x] 対象言語をRustのみに決める
* [x] CLI形式をCargoサブコマンド形式に決める
* [x] 公開形態をCLI + library crateに決める
* [x] 初期対応OSを決める
* [x] 認証情報の保存方針を決める
* [x] AtCoderへのアクセス方針を決める
* [x] AtCoderのHTML変更に対する設計方針を決める
* [x] Rust実行環境情報の管理方針を決める
* [x] 初期MVPの範囲を決める
* [x] 初期段階でやらないことを決める

### Phase 0 完了条件

* [x] プロジェクトの目的が説明できる
* [x] 初期MVPが決まっている
* [x] 初期段階でやらないことが決まっている
* [x] Codexに実装を依頼する前提の開発方針が決まっている

## Phase 1: GitHubリポジトリ初期作成

### 目的

Codexに作業を依頼する前に、人間がGitHubリポジトリの器を作成し、プロジェクトの目的・方向性・初期ドキュメントを配置する。

このPhaseでは、Codex向けの詳細な運用ルールまでは作り込まない。
目的は、リポジトリとして存在し、プロジェクトの方向性が読める状態にすることである。

### リポジトリ名候補

| 候補 | 備考 |
| ------------------ | ------------------------ |
| `cargo-ac` | 第一候補。`cargo ac` として使いやすい |
| `cargo-atcoder-rs` | 意味は明確だが少し長い |
| `atcoder-rs-cli` | CLIであることが分かりやすい |
| `ac-rs` | 短いが意味がやや曖昧 |
| `rust-atcoder-cli` | 分かりやすいが長い |

### リポジトリ名決定

リポジトリ名は `cargo-ac`に決定する。

### READMEに含める内容

| 項目 | 内容 |
| ------------------ | ---------------------- |
| プロジェクト名 | `cargo-ac` など |
| 概要 | Rust向けAtCoder CLIであること |
| コンセプト | Rustを書く人にとって気持ちいいCLI |
| Planned Features | 実装予定機能 |
| Example Usage | 想定される利用例 |
| Development Status | 開発中であること |
| Design Principles | 設計方針 |
| Initial MVP | 初期MVP |
| Non-goals | 初期段階でやらないこと |
| License | ライセンス |

### README冒頭案

```markdown
# cargo-ac

`cargo-ac` は、RustでAtCoderに参加する人のためのCLIツールです。

Rustの開発体験に自然になじむ形で、AtCoderの問題取得、ローカルテスト、提出、提出結果の確認までを一気通貫で扱えることを目指します。

## コンセプト

Rustを書く人にとって気持ちいい、現代的なAtCoder CLI。

既存のAtCoder向けツールには便利なものがありますが、長期間更新されていないものもあり、AtCoder側のHTML構造変更、言語ID変更、Rust実行環境の更新に追従しづらい場合があります。

本プロジェクトでは、単にAtCoderへ提出できるだけではなく、以下を重視します。

* Cargo標準の開発体験に近い操作感
* AtCoderの仕様変更に追従しやすい設計
* AtCoderのRust実行環境に追従しやすい設計
* ローカルテストから提出、結果確認までの一貫した操作
* 壊れにくく、壊れたときに直しやすい実装
* Codexなどの開発支援ツールに作業を依頼しやすいIssue・PR運用
```

### ライセンス候補

| ライセンス | 備考 |
| ----------------- | ---------------------- |
| MIT | シンプルで広く使われる |
| Apache-2.0 | 特許条項を含む |
| MIT OR Apache-2.0 | Rust crateでよく使われる組み合わせ |

### ライセンス決定

ライセンスは`MIT or Apache-2.0`を採用します。

### .gitignore候補

```gitignore
/target/
**/*.rs.bk
.DS_Store
.vscode/
.idea/
*.log

# local credentials / sessions
*.cookie
session.json
session.cookie

# local debug output
.cache/
debug-html/
```

### docsディレクトリ

| ファイル | 役割 |
| ---------------------- | ------------ |
| `docs/tasks.md` | 全体タスク一覧 |
| `docs/ROADMAP.md` | 開発ロードマップ |
| `docs/ARCHITECTURE.md` | crate構成・設計方針 |

### タスク

* [x] GitHub上に新規リポジトリを作成する
* [x] リポジトリ名を決定する
* [x] 公開範囲を決定する
* [x] 初期ブランチを `main` にする
* [x] リポジトリ説明文を設定する
* [x] Topicsを設定する
* [x] README.mdを作成する
* [x] LICENSEを追加する
* [x] .gitignoreを追加する
* [x] docsディレクトリを作成する
* [x] docs/tasks.mdを追加する
* [x] docs/ROADMAP.mdを追加する
* [x] docs/ARCHITECTURE.mdを追加する
* [x] プロジェクトの目的をREADMEに書く
* [x] 想定コマンドをREADMEに書く
* [x] 初期MVPをREADMEに書く
* [x] 初期段階でやらないことをREADMEに書く
* [x] `LICENSE-MIT` を追加する
* [x] `LICENSE-APACHE` を追加する
* [x] READMEに `MIT OR Apache-2.0` と明記する
* [ ] `Cargo.toml` の `license` に `MIT OR Apache-2.0` を設定する

### Phase 1 完了条件

* [x] GitHub上に開発用リポジトリが存在する
* [x] READMEにプロジェクトの目的と方針が書かれている
* [x] LICENSEが存在する
* [x] .gitignoreが存在する
* [x] docs/tasks.mdが存在する
* [x] docs/ROADMAP.mdが存在する
* [x] docs/ARCHITECTURE.mdが存在する
* [x] Phase 2でCodex運用整備に進める状態になっている

## Phase 2: Codex前提のGitHub運用整備

### 目的

Codexに実装を依頼する前に、Issue、PR、branch、commit、CI、レビュー、Project boardの運用ルールを整備する。

このPhaseでは、人間がCodex作業のためのガードレールを作る。
ここで作成する `CODEX.md`、Issue template、PR template、labels、milestones、Project board、branch protectionが、以降のCodex作業の前提となる。

### 基本運用

| 項目 | 方針 |
| ------------ | ---------------------------------------------------- |
| Issue | 1〜3時間で終わる程度に分割する |
| Branch | Issueごとに作成する |
| Pull Request | 1 Issue = 1 PRを基本にする |
| Merge | 人間がSquash mergeする |
| main | 常にCIが通る安定ブランチにする |
| Codex対象 | `agent: codex-ready` が付いたIssueのみ |
| 要注意Issue | 認証・Cookie・提出周りは `agent: needs-review-carefully` を付ける |

### Branch命名規則

| 種別 | 例 |
| ------- | ------------------------- |
| feature | `feature/12-cli-skeleton` |
| feature | `feature/18-new-command` |
| feature | `feature/24-test-runner` |
| fix | `fix/31-sample-parser-ja` |
| docs | `docs/9-roadmap` |
| chore | `chore/5-github-actions` |

形式は以下とする。

```text
<type>/<issue-number>-<short-description>
```

### Commit message規則

Conventional Commits形式を採用する。

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

### Commit type候補

| type | 用途 |
| ---------- | ----------- |
| `feat` | 機能追加 |
| `fix` | バグ修正 |
| `docs` | ドキュメント |
| `chore` | 雑務・設定 |
| `refactor` | 振る舞いを変えない整理 |
| `test` | テスト追加・修正 |
| `ci` | CI変更 |

### Commit scope候補

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

### GitHub Project status

| Status | 意味 |
| ----------- | ------------------- |
| Backlog | いつかやる |
| Ready | Codexに依頼可能な粒度まで整理済み |
| In Progress | 作業中 |
| In Review | PRレビュー中 |
| Done | mainにマージ済み |

### Ready条件

| 条件 | 内容 |
| ------------------- | -------------------------------- |
| 要件 | 箇条書きで明確である |
| Scope | 対象ファイル・モジュールがある程度分かる |
| Acceptance criteria | 完了条件がある |
| Non-goals | やらないことが明記されている |
| 依存Issue | 依存関係が明記されている |
| label | `agent: codex-ready` を付けてよい状態である |

### GitHub labels

| 分類 | ラベル |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| type | `type: feature`, `type: bug`, `type: refactor`, `type: docs`, `type: test`, `type: chore` |
| area | `area: cli`, `area: core`, `area: config`, `area: runner`, `area: testcase`, `area: atcoder`, `area: parser`, `area: auth`, `area: submit`, `area: env`, `area: ci`, `area: docs` |
| priority | `priority: high`, `priority: medium`, `priority: low` |
| agent | `agent: codex-ready`, `agent: needs-human-design`, `agent: needs-review-carefully` |

### Milestone案

| Milestone | 内容 |
| ------------------------------------------------- | -------------- |
| `v0.1.0-alpha.1: repository and CLI skeleton` | リポジトリ整備とCLI骨格 |
| `v0.1.0-alpha.2: local workspace and test runner` | ローカル生成・ローカルテスト |
| `v0.1.0-alpha.3: sample downloader` | サンプル取得 |
| `v0.1.0-alpha.4: login and submit` | ログイン・提出 |
| `v0.1.0: initial usable release` | 初回利用可能版 |

### CODEX.md雛形

```markdown
# Instructions for Codex

This repository implements `cargo-ac`, a modern AtCoder CLI for Rust users.

## Project rules

- Keep changes small and focused.
- Implement one GitHub issue per PR.
- Do not implement adjacent features unless explicitly requested.
- Keep `ac-core` independent from CLI-only dependencies.
- Do not hard-code AtCoder language IDs.
- Do not log credentials, cookies, CSRF tokens, or source code unnecessarily.
- Avoid aggressive polling against AtCoder.
- Prefer fixture-based tests for HTML parsers.
- Run `cargo fmt` and `cargo test --all` before completing a task.

## Crate boundaries

- `cargo-ac`: CLI only.
- `ac-core`: core logic, config, runner, AtCoder client abstractions.
- Future: `ac-html`, `ac-env`.

## Error handling

- Library code should return typed errors where practical.
- CLI code may use `anyhow`.
- Do not panic in library code for user input or external data.
```

### タスク

* [x] リポジトリルートに `AGENTS.md` を作成する
* [x] `AGENTS.md` にCodex向けの恒常的な作業指示を書く
* [x] `AGENTS.md` にプロジェクトの目的を書く
* [x] `AGENTS.md` にcrate境界を書く
* [x] `AGENTS.md` にエラーハンドリング方針を書く
* [x] `AGENTS.md` にAtCoder連携時の注意点を書く
* [x] `AGENTS.md` にセキュリティ上の禁止事項を書く
* [x] `AGENTS.md` にテスト実行方針を書く
* [x] `AGENTS.md` にBranch命名規則を書く
* [x] `AGENTS.md` にCommit message規則を書く
* [x] `AGENTS.md` にPR作成ルールを書く
* [x] `main` ブランチを保護する
* [x] Pull Request経由のmergeを必須にする
* [x] status checks成功を必須にする
* [x] force pushを禁止する
* [x] `.github/ISSUE_TEMPLATE/` ディレクトリを作成する
* [x] `.github/ISSUE_TEMPLATE/feature.yml` を作成する
* [x] bug用Issue templateを作成する
* [x] `.github/pull_request_template.md` を作成する
* [x] `.github/workflows/ci.yml` を作成する
* [x] Rust toolchainを設定する
* [x] CIで `cargo fmt --all -- --check` を実行する
* [x] CIで `cargo clippy --all-targets --all-features -- -D warnings` を実行する
* [x] CIで `cargo test --all` を実行する
* [x] GitHub labelsを作成する
* [x] GitHub milestonesを作成する
* [x] GitHub Projectsを作成する
* [x] Project statusを作成する
* [x] Ready条件を定義する
* [x] Phase 3以降の初期Issueを作成する
* [x] Codexに依頼できるIssueに `agent: codex-ready` を付ける

### Phase 2 完了条件

* [x] `AGENTS.md` にCodex向け作業指示が書かれている
* [x] `AGENTS.md` にBranch命名規則が書かれている
* [x] `AGENTS.md` にCommit message規則が書かれている
* [x] `AGENTS.md` にPR作成ルールが書かれている
* [x] Issue templateが作成されている
* [x] PR templateが作成されている
* [x] CIが作成されている
* [x] labelsが整理されている
* [x] milestonesが作成されている
* [x] Project boardが作成されている
* [x] Branch protectionが設定されている
* [x] Phase 3以降の初期Issueが登録されている
* [x] Codexに依頼可能なIssueが1つ以上Readyになっている

## Phase 3: CLI骨格作成

### 目的

実機能なしでCLIの形を作成する。
ここからCodexに実装を依頼する対象とする。

### 実装するコマンド骨格

| コマンド | 内容 |
| --------------------------- | ----------------- |
| `cargo ac --help` | CLI全体のヘルプ |
| `cargo ac login --help` | ログインコマンドのヘルプ |
| `cargo ac new --help` | プロジェクト生成コマンドのヘルプ |
| `cargo ac download --help` | 問題取得コマンドのヘルプ |
| `cargo ac test --help` | テストコマンドのヘルプ |
| `cargo ac submit --help` | 提出コマンドのヘルプ |
| `cargo ac watch --help` | 結果監視コマンドのヘルプ |
| `cargo ac doctor --help` | 診断コマンドのヘルプ |
| `cargo ac selfcheck --help` | selfcheckコマンドのヘルプ |
| `cargo ac env --help` | 環境情報コマンドのヘルプ |
| `cargo ac lang --help` | 言語IDコマンドのヘルプ |

### 推奨ディレクトリ構成

```text
cargo-ac/
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
├── CODEX.md
├── docs/
│   ├── ROADMAP.md
│   ├── CONTRIBUTING.md
│   ├── BRANCHING.md
│   ├── ARCHITECTURE.md
│   └── tasks.md
├── crates/
│   ├── cargo-ac/
│   └── ac-core/
└── .github/
    ├── workflows/
    ├── ISSUE_TEMPLATE/
    └── pull_request_template.md
```

### タスク

* [x] Rust workspaceを作成する
* [x] `cargo-ac` CLI crateを作成する
* [x] `ac-core` library crateを作成する
* [x] `clap` を導入する
* [x] CLIコマンド構造を定義する
* [x] placeholder command handlersを追加する
* [x] CLI境界での基本的なエラーハンドリングを追加する
* [x] `cargo ac --help` を表示できるようにする
* [x] 未実装コマンドで適切なメッセージを返す
* [x] `cargo fmt` を通す
* [x] `cargo test --all` を通す

### Phase 3 完了条件

* [x] `cargo ac --help` が実行できる
* [x] 主要コマンドのhelpが表示できる
* [x] 未実装コマンドが適切なメッセージを返す
* [x] `cargo fmt --all -- --check` が通る
* [x] `cargo clippy --all-targets --all-features -- -D warnings` が通る
* [x] `cargo test --all` が通る

## Phase 4: ローカルプロジェクト生成

### 目的

AtCoderにアクセスしないローカル機能として `new` / `init` を実装する。

### 生成される構成

```text
abc400/
├── Cargo.toml
├── ac.toml
├── src/
│   └── bin/
│       ├── a.rs
│       ├── b.rs
│       ├── c.rs
│       ├── d.rs
│       ├── e.rs
│       └── f.rs
└── testcases/
    ├── a/
    ├── b/
    ├── c/
    ├── d/
    ├── e/
    └── f/
```

### ac.tomlで管理する情報

| 項目 | 内容 |
| ------------------ | -------------- |
| contest ID | `abc400` など |
| source directory | `src/bin` など |
| testcase directory | `testcases` など |
| language name | `rust` |
| Rust edition | `2021` など |
| task ID | `abc400_a` など |
| bin名 | `a` など |

### タスク

* [ ] `cargo ac new <contest>` を実装する
* [ ] コンテスト用ディレクトリを生成する
* [ ] `Cargo.toml` を生成する
* [ ] `ac.toml` を生成する
* [ ] `src/bin/*.rs` を生成する
* [ ] `testcases/*` ディレクトリを生成する
* [ ] デフォルトテンプレートを用意する
* [ ] 既存ディレクトリがある場合の挙動を決める
* [ ] `--force` オプションの必要性を検討する
* [ ] workspace generationのテストを追加する

### Phase 4 完了条件

* [ ] `cargo ac new abc400` が実行できる
* [ ] コンテスト用ディレクトリが生成される
* [ ] `Cargo.toml` が生成される
* [ ] `ac.toml` が生成される
* [ ] `src/bin/*.rs` が生成される
* [ ] `testcases/*` が生成される
* [ ] 生成処理のテストがある
* [ ] CIが通る

## Phase 5: ローカルテスト実行

### 目的

AtCoder提出前にローカルでサンプル確認する。

### 対象コマンド

| コマンド | 目的 |
| -------------------------------- | ----------------------- |
| `cargo ac test <task>` | 指定taskのサンプルを実行する |
| `cargo ac test all` | 全taskのサンプルを実行する |
| `cargo ac test <task> --release` | release buildでサンプルを実行する |
| `cargo ac addcase <task>` | 自作ケースを追加する |

### 表示する結果

| 結果 | 内容 |
| --- | -------- |
| AC | 期待出力と一致 |
| WA | 期待出力と不一致 |
| RE | 実行時エラー |
| TLE | タイムアウト |

### タスク

* [ ] `cargo ac test <task>` を実装する
* [ ] `cargo ac test all` を実装する
* [ ] `cargo ac test <task> --release` を実装する
* [ ] testcase discoveryを実装する
* [ ] `.in` と `.out` のペア検証を実装する
* [ ] `cargo run --bin <bin>` の実行処理を実装する
* [ ] 標準入力に `.in` の内容を渡す
* [ ] stdoutを取得する
* [ ] stderrを取得する
* [ ] exit statusを取得する
* [ ] output normalizationを実装する
* [ ] expected / actual の比較を実装する
* [ ] AC / WA / RE / TLE の表示を実装する
* [ ] テスト結果のsummary表示を実装する
* [ ] WA時のdiff表示を実装する
* [ ] timeout設定を実装する
* [ ] test runnerのunit testを追加する
* [ ] `cargo ac addcase <task>` を実装する
* [ ] 対話的にinputを受け取る
* [ ] 対話的にexpected outputを受け取る
* [ ] `custom-N.in` を保存する
* [ ] `custom-N.out` を保存する
* [ ] 既存番号と衝突しないようにする

### Phase 5 完了条件

* [ ] `cargo ac test a` が実行できる
* [ ] sample inputを標準入力として渡せる
* [ ] expected outputとactual outputを比較できる
* [ ] AC / WAが表示できる
* [ ] WA時に差分が表示できる
* [ ] `cargo ac addcase a` で自作ケースを追加できる
* [ ] test runnerのテストがある
* [ ] CIが通る

## Phase 6: 問題ダウンロード

### 目的

AtCoderのコンテストページから問題一覧を取得し、問題ページからサンプル入力・出力を取得する。

### 対象コマンド

| コマンド | 目的 |
| ----------------------------------- | --------------------- |
| `cargo ac download <contest>` | コンテストの問題・サンプルを取得する |
| `cargo ac new <contest> --download` | プロジェクト生成と同時にサンプルを取得する |

### parser対応対象

| 表記 | 用途 |
| --------------- | --------- |
| `入力例` | 日本語UIの入力例 |
| `出力例` | 日本語UIの出力例 |
| `Sample Input` | 英語UIの入力例 |
| `Sample Output` | 英語UIの出力例 |
| `pre` | サンプル本文 |

### タスク

* [ ] HTTP client foundationを追加する
* [ ] AtCoder task page fetchを実装する
* [ ] contest task list parserを実装する
* [ ] sample parserを実装する
* [ ] 日本語ラベルのsample parserを実装する
* [ ] 英語ラベルのsample parserを実装する
* [ ] `pre` 要素の抽出処理を実装する
* [ ] 入力例・出力例の対応付けを実装する
* [ ] 取得したサンプルを `testcases/{task}/sample-N.in/out` に保存する
* [ ] `ac.toml` にtask情報を書き込む
* [ ] `cargo ac download <contest>` を実装する
* [ ] `cargo ac new <contest> --download` を実装する
* [ ] parse failure時にdebug HTMLを保存する
* [ ] parser fixture testsを追加する
* [ ] エラー時にissue報告しやすい情報を出す

### Phase 6 完了条件

* [ ] `cargo ac download abc400` が実行できる
* [ ] コンテスト内の問題一覧を取得できる
* [ ] 問題ページからサンプルを取得できる
* [ ] サンプルを `testcases/{task}` に保存できる
* [ ] 日本語UIと英語UIのサンプル抽出に対応している
* [ ] parser fixture testがある
* [ ] parse failure時にdebug HTMLを保存できる
* [ ] CIが通る

## Phase 7: ログイン・セッション管理

### 目的

AtCoderへのログイン状態を管理する。

### 対象コマンド

| コマンド | 目的 |
| ----------------- | -------------- |
| `cargo ac login` | AtCoderへログインする |
| `cargo ac whoami` | ログイン状態を確認する |

### 認証情報の保存方針

| 優先度 | 保存方式 |
| --- | --------------- |
| 1 | OS keyring |
| 2 | cookie jar file |
| 3 | 毎回ログイン |

### セキュリティ上の禁止事項

| 対象 | 方針 |
| ------------ | ---------------- |
| パスワード | ログ出力しない |
| Cookie | ログ出力しない |
| CSRF token | ログ出力しない |
| debug HTML | 認証情報が含まれないよう注意する |
| session file | 権限を考慮する |

### タスク

* [ ] `cargo ac login` を実装する
* [ ] `cargo ac whoami` を実装する
* [ ] login page parserを実装する
* [ ] CSRF token extractionを実装する
* [ ] username / passwordの入力処理を実装する
* [ ] パスワードをマスク入力できるようにする
* [ ] login POSTを実装する
* [ ] ログイン成功判定を実装する
* [ ] session cookie storage abstractionを実装する
* [ ] OS keyring保存を検討する
* [ ] cookie jar file保存を実装する
* [ ] ログイン切れ検出を実装する
* [ ] 認証情報削除コマンドを検討する
* [ ] READMEに保存情報・保存場所・削除方法を書く

### Phase 7 完了条件

* [ ] `cargo ac login` が実行できる
* [ ] ログイン成功を判定できる
* [ ] セッションを保存できる
* [ ] `cargo ac whoami` でログイン状態を確認できる
* [ ] 認証情報やCookieをログ出力しない
* [ ] CIが通る

## Phase 8: 提出・結果監視

### 目的

ローカルのRustコードをAtCoderへ提出し、提出結果を監視する。

### 対象コマンド

| コマンド | 目的 |
| ---------------------------------- | ---------------- |
| `cargo ac submit <task>` | 指定taskへ提出する |
| `cargo ac submit <task> --watch` | 提出後に結果を監視する |
| `cargo ac submit <task> --no-test` | ローカルテストを省略して提出する |
| `cargo ac submit <task> --yes` | 確認プロンプトを省略する |
| `cargo ac watch` | 最新提出の結果を監視する |

### 提出前チェック

| チェック | 内容 |
| ------------- | ------------------------- |
| source exists | `src/bin/<task>.rs` が存在する |
| cargo check | コンパイル確認 |
| local tests | サンプルテスト |
| language_id | Rustのlanguage_idを固定値なしで解決 |
| confirmation | 提出前に確認プロンプトを表示 |

### watch対象ステータス

| ステータス | 内容 |
| ------- | ------------------- |
| WJ | Waiting for Judging |
| Judging | ジャッジ中 |
| AC | Accepted |
| WA | Wrong Answer |
| TLE | Time Limit Exceeded |
| RE | Runtime Error |
| CE | Compilation Error |

### ポーリング方針

| 項目 | 方針 |
| ------ | ---------- |
| 初回 | 2秒後 |
| 以降 | 3〜5秒間隔 |
| タイムアウト | 2〜3分目安 |
| 注意点 | 過剰アクセスを避ける |

### タスク

* [ ] `cargo ac submit <task>` を実装する
* [ ] `cargo ac submit <task> --watch` を実装する
* [ ] `cargo ac submit <task> --no-test` を実装する
* [ ] `cargo ac submit <task> --yes` を実装する
* [ ] `cargo ac watch` を実装する
* [ ] submit form parserを実装する
* [ ] language selector parserを実装する
* [ ] Rust language_id resolverを実装する
* [ ] language_idをcacheに保存する
* [ ] language_id refreshを実装する
* [ ] submit preflight checksを実装する
* [ ] `src/bin/<task>.rs` の存在確認を実装する
* [ ] `cargo check` を実行する
* [ ] ローカルサンプルテストを実行する
* [ ] 提出確認プロンプトを実装する
* [ ] submit POSTを実装する
* [ ] 提出結果URLを表示する
* [ ] submission result watchを実装する
* [ ] WJ / Judging / AC / WA / TLE / RE / CE などの表示を実装する
* [ ] ポーリング間隔を制御する
* [ ] ポーリングタイムアウトを実装する
* [ ] 提出連打防止を検討する

### Phase 8 完了条件

* [ ] `cargo ac submit a` が実行できる
* [ ] 提出前チェックが実行される
* [ ] Rust language_idを固定値なしで解決できる
* [ ] 提出前に確認プロンプトが出る
* [ ] コードを提出できる
* [ ] `--watch` で提出結果を監視できる
* [ ] 過剰なポーリングをしない
* [ ] CIが通る

## Phase 9: doctor / selfcheck / env update

### 目的

ローカル環境診断、AtCoder側の構造変更検知、AtCoder Rust環境情報の管理を行う。

### 対象コマンド

| コマンド | 目的 |
| --------------------- | ---------------------- |
| `cargo ac doctor` | ローカル環境やプロジェクト構成を診断する |
| `cargo ac selfcheck` | AtCoder側のHTML構造変更を検知する |
| `cargo ac env show` | AtCoder Rust環境情報を表示する |
| `cargo ac env update` | AtCoder Rust環境情報を更新する |

### doctor診断項目

| 診断項目 | 内容 |
| ------------------- | --------------------------- |
| `ac.toml` | 存在確認 |
| `Cargo.toml` | 存在確認 |
| `src/bin/*.rs` | ソース存在確認 |
| `testcases/*` | テストケース存在確認 |
| login session | セッション有効性確認 |
| language_id | Rust language_id解決済みか確認 |
| rustc version | ローカルRustバージョン確認 |
| crate compatibility | AtCoderで利用できないcrateの検出 |
| release build | `cargo build --release` の確認 |

### selfcheck対象

| 対象 | 内容 |
| ------------------------ | ------------ |
| Top page | 到達確認 |
| Login form parser | ログインフォーム構造確認 |
| Task page parser | 問題ページ構造確認 |
| Submit page parser | 提出ページ構造確認 |
| Language selector parser | 言語選択構造確認 |
| Sample parser | サンプル抽出構造確認 |

### envで管理する情報

| 項目 | 内容 |
| ------------ | ----------------- |
| Rust version | AtCoderのRustバージョン |
| Rust edition | AtCoderで使うedition |
| crates | 利用可能crate一覧 |
| updated_at | 環境定義の更新日 |

### タスク

* [ ] `cargo ac doctor` を実装する
* [ ] `ac.toml` の存在確認を実装する
* [ ] `Cargo.toml` の存在確認を実装する
* [ ] `src/bin/*.rs` の存在確認を実装する
* [ ] `testcases/*` の存在確認を実装する
* [ ] login sessionの有効性確認を実装する
* [ ] Rust language_id解決済みか確認する
* [ ] rustc version確認を実装する
* [ ] AtCoder環境との差分確認を実装する
* [ ] AtCoderで利用できないcrateの検出を検討する
* [ ] `cargo build --release` が通るか確認する
* [ ] doctor結果をOK / WARN / ERRORで表示する
* [ ] `cargo ac selfcheck` を実装する
* [ ] AtCoderトップページ到達確認を実装する
* [ ] Login form parserの確認を実装する
* [ ] Task page parserの確認を実装する
* [ ] Submit page parserの確認を実装する
* [ ] Language selector parserの確認を実装する
* [ ] Sample parserの確認を実装する
* [ ] selfcheck結果をOK / WARN / ERRORで表示する
* [ ] 週次selfcheck用GitHub Actionsを追加する
* [ ] env data modelを実装する
* [ ] `cargo ac env show` を実装する
* [ ] `cargo ac env update` を実装する
* [ ] static JSONから環境情報を読み込む
* [ ] 将来的にGitHub上の環境定義ファイルから取得できるようにする
* [ ] Rust versionを管理する
* [ ] Rust editionを管理する
* [ ] 利用可能crate一覧を管理する
* [ ] `updated_at` を管理する
* [ ] ツール本体と環境定義を分離する

### Phase 9 完了条件

* [ ] `cargo ac doctor` が実行できる
* [ ] ローカルプロジェクト構成を診断できる
* [ ] ログイン状態を診断できる
* [ ] Rust環境差分を表示できる
* [ ] OK / WARN / ERRORで表示できる
* [ ] `cargo ac selfcheck` が実行できる
* [ ] AtCoderページ構造の変更を検知できる
* [ ] 週次実行CIがある
* [ ] `cargo ac env show` が実行できる
* [ ] `cargo ac env update` が実行できる
* [ ] ツール本体と環境定義を分離できている

## Phase 10: crates.io公開・継続運用

### 目的

基本機能が安定したら、crates.ioへ公開し、継続的に保守できる状態にする。

### 公開対象

| crate | 役割 |
| ---------- | ------------------------- |
| `cargo-ac` | CLI本体 |
| `ac-core` | AtCoder操作・設定・テスト実行などの共通処理 |

### 将来的なcrate分離候補

| crate | 役割 |
| --------- | ------------------- |
| `ac-html` | AtCoder HTML parser |
| `ac-env` | AtCoder Rust環境情報管理 |

### バージョン方針

| Version | 内容 |
| ------- | ----------------------------------------------------------------------------------- |
| v0.1.0 | `new`, `download`, `test`, `login`, `submit`, `watch`, `lang refresh`, `doctor` 簡易版 |
| v0.2.0 | `env show`, `env update`, `selfcheck`, debug HTML保存, crate互換性チェック |
| v0.3.0 | template管理強化, `addcase`, 複数task一括test, 提出履歴表示, 設定ファイル改善 |
| v1.0.0 | 基本機能安定、HTML変更検知方針確立、README/CI整備、破壊的変更が少ないAPI |

### タスク

* [ ] READMEを整備する
* [ ] LICENSEを確認する
* [ ] CHANGELOGを作成する
* [ ] GitHub Actions CIを整備する
* [ ] unit testを整備する
* [ ] integration testを整備する
* [ ] issue templateを整備する
* [ ] PR templateを整備する
* [ ] CODEX.mdを整備する
* [ ] セキュリティ方針を書く
* [ ] AtCoderに過剰アクセスしない方針を書く
* [ ] crates.io用metadataを整備する
* [ ] crate名の空きを確認する
* [ ] `cargo publish --dry-run` を実行する
* [ ] crates.ioへ公開する

### Phase 10 完了条件

* [ ] crates.io公開準備が完了している
* [ ] `cargo publish --dry-run` が通る
* [ ] READMEとCHANGELOGが整備されている
* [ ] セキュリティ方針が明文化されている
* [ ] AtCoderに優しい利用方針が明文化されている
* [ ] crates.ioへ公開できる

## 優先順位つきバックログ

### Must

| 項目 | 内容 |
| ---------------------- | ------------------------- |
| GitHubリポジトリ作成 | 開発用リポジトリを作成する |
| README作成 | プロジェクトの目的を書く |
| LICENSE作成 | ライセンスを明記する |
| .gitignore作成 | 不要ファイル・認証情報を除外する |
| docs/tasks.md作成 | 全体タスクを管理する |
| CODEX.md作成 | Codex向け指示を書く |
| Issue / PR template作成 | Codex作業とレビューを安定させる |
| GitHub Actions CI作成 | fmt / clippy / testを自動化する |
| Rust workspace作成 | crate構成の土台を作る |
| CLI skeleton | コマンド体系を作る |
| new/init | プロジェクト生成 |
| ac.toml | 設定ファイル管理 |
| sample testcase format | テストケース構造 |
| test runner | ローカルテスト実行 |
| sample downloader | サンプル取得 |
| login | AtCoderログイン |
| language resolver | Rust language_id解決 |
| submit | 提出 |
| watch | 結果監視 |

### Should

| 項目 | 内容 |
| ----------------------- | ------------- |
| doctor | ローカル環境診断 |
| debug HTML保存 | parser失敗時の調査用 |
| addcase | 自作ケース追加 |
| template管理 | ユーザー定義テンプレート |
| env show | Rust環境表示 |
| env update | Rust環境更新 |
| selfcheck | AtCoder構造変更検知 |
| weekly compatibility CI | 定期互換性チェック |

### Could

| 項目 | 内容 |
| ----------------- | --------------- |
| 提出履歴表示 | 過去提出の表示 |
| 問題文markdown保存 | 問題文のローカル保存 |
| VSCode task生成 | VSCode連携 |
| shell completion | 補完対応 |
| GitHub release自動化 | リリース効率化 |
| Windows向け動作確認強化 | Windows利用者向け安定化 |

### Won’t initially

| 項目 | 理由 |
| ------------- | -------------------- |
| 複数OJ対応 | 初期スコープを超えるため |
| GUI | CLIを優先するため |
| AI解説生成 | AtCoderルールとの関係が難しいため |
| ブラウザ自動操作 | HTTP/HTMLベースで進めるため |
| コンテスト中の外部AI連携 | 安全・規約面で慎重に扱う必要があるため |

## 推奨Issue分割

### Phase 3: CLI skeleton

| Issue | 内容 |
| ----- | ---------------------------------------- |
| #1 | Create Rust workspace |
| #2 | Add cargo-ac CLI crate |
| #3 | Add ac-core library crate |
| #4 | Define clap command structure |
| #5 | Add placeholder command handlers |
| #6 | Add basic error handling at CLI boundary |

### Phase 4: Local workspace

| Issue | 内容 |
| ----- | ------------------------------------------- |
| #7 | Implement ac.toml data model |
| #8 | Implement ac.toml read/write |
| #9 | Implement cargo ac new |
| #10 | Generate Cargo.toml for contest workspace |
| #11 | Generate src/bin/*.rs from default template |
| #12 | Generate testcases directories |
| #13 | Add tests for workspace generation |

### Phase 5: Test runner

| Issue | 内容 |
| ----- | ---------------------------------- |
| #14 | Implement testcase discovery |
| #15 | Implement testcase pair validation |
| #16 | Implement cargo run execution |
| #17 | Implement output normalization |
| #18 | Implement AC/WA result display |
| #19 | Implement diff display for WA |
| #20 | Implement cargo ac addcase |
| #21 | Add tests for test runner |

### Phase 6: AtCoder download

| Issue | 内容 |
| ----- | ------------------------------------------- |
| #22 | Add HTTP client foundation |
| #23 | Implement task page fetch |
| #24 | Implement sample parser for Japanese labels |
| #25 | Implement sample parser for English labels |
| #26 | Save debug HTML on parse failure |
| #27 | Implement contest task list parser |
| #28 | Implement cargo ac download |
| #29 | Add parser fixture tests |

### Phase 7: Login and submit

| Issue | 内容 |
| ----- | -------------------------------------------- |
| #30 | Implement login page parser |
| #31 | Implement CSRF token extraction |
| #32 | Implement session cookie storage abstraction |
| #33 | Implement cargo ac login |
| #34 | Implement cargo ac whoami |
| #35 | Implement language selector parser |
| #36 | Implement language_id resolver for Rust |
| #37 | Implement submit form parser |
| #38 | Implement cargo ac submit preflight checks |
| #39 | Implement submit POST |
| #40 | Implement submission result watch |

### Phase 9: Maintenance features

| Issue | 内容 |
| ----- | ---------------------------------------------- |
| #41 | Implement cargo ac doctor basic checks |
| #42 | Implement cargo ac selfcheck |
| #43 | Implement env data model |
| #44 | Implement cargo ac env show |
| #45 | Implement cargo ac env update from static JSON |
| #46 | Add weekly compatibility CI workflow |

## 初期マイルストーン

### Milestone 1: GitHubリポジトリ初期作成

* [ ] GitHub repository exists
* [ ] README exists
* [ ] LICENSE exists
* [ ] .gitignore exists
* [ ] docs/tasks.md exists
* [ ] docs/ROADMAP.md exists
* [ ] docs/ARCHITECTURE.md exists

### Milestone 2: Codex前提のGitHub運用整備

* [ ] CODEX.md exists
* [ ] PR template exists
* [ ] Issue template exists
* [ ] GitHub Actions CI runs
* [ ] Labels are created
* [ ] Milestones are created
* [ ] Project board is prepared
* [ ] Branch protection is enabled
* [ ] Codex-ready issue exists

### Milestone 3: CLIとして起動できる

* [ ] `cargo ac --help` が実行できる
* [ ] clapでコマンド一覧が表示される
* [ ] 未実装コマンドが適切にメッセージを返す

### Milestone 4: ローカルCLIとして使える

* [ ] `cargo ac new abc400` が実行できる
* [ ] `cd abc400` 後に作業できる
* [ ] `cargo ac test a` が実行できる
* [ ] `cargo ac addcase a` が実行できる
* [ ] AtCoder連携なしでRust競プロ用プロジェクトを生成できる
* [ ] 手動で置いたサンプルをテストできる
* [ ] 自作ケースを追加できる

### Milestone 5: サンプル取得できる

* [ ] `cargo ac download abc400` が実行できる
* [ ] AtCoder問題ページからサンプルを取得できる
* [ ] 取得したサンプルをローカルで実行できる

### Milestone 6: 提出できる

* [ ] `cargo ac login` が実行できる
* [ ] `cargo ac submit a --watch` が実行できる
* [ ] AtCoderにログインできる
* [ ] Rustのlanguage_idを自動解決できる
* [ ] コードを提出できる
* [ ] 提出結果を監視できる

### Milestone 7: 壊れにくさを確認できる

* [ ] `cargo ac doctor` が実行できる
* [ ] `cargo ac selfcheck` が実行できる
* [ ] `cargo ac env update` が実行できる
* [ ] ローカル環境を診断できる
* [ ] AtCoder HTML構造変更を検知できる
* [ ] AtCoder Rust環境情報を更新できる

## 開発順序

* [ ] コンセプト・仕様整理
* [ ] GitHubリポジトリ作成
* [ ] README / LICENSE / .gitignore 作成
* [ ] docs/tasks.md 作成
* [ ] docs/ROADMAP.md 作成
* [ ] docs/ARCHITECTURE.md 作成
* [ ] CODEX.md 作成
* [ ] Issue / PR template 作成
* [ ] GitHub Actions CI 作成
* [ ] GitHub Project / labels / milestones 作成
* [ ] Branch protection設定
* [ ] Phase 3以降の初期Issue作成
* [ ] Rust workspace作成
* [ ] clapでCLI定義
* [ ] new/init 実装
* [ ] ac.toml 読み書き
* [ ] testcases構造定義
* [ ] cargo run実行処理
* [ ] expected/actual比較
* [ ] addcase
* [ ] AtCoder task page取得
* [ ] sample parser
* [ ] contest downloader
* [ ] debug HTML保存
* [ ] login parser
* [ ] session保存
* [ ] language resolver
* [ ] submit
* [ ] watch
* [ ] doctor
* [ ] selfcheck
* [ ] env update
* [ ] README整備
* [ ] crates.io公開

## 開発時に守る設計ルール

### 固定値に頼らない

| 対象 | 方針 |
| -------------- | ------------ |
| language_id | 固定しない |
| AtCoder HTML構造 | 1パターンだけ想定しない |
| Rustバージョン | バイナリに直書きしない |
| testcasesの場所 | 変更不可にしない |

### 壊れたときに原因が見えるようにする

| 表示する情報 | 内容 |
| ---------- | ----------- |
| URL | どのURLで失敗したか |
| parser | 何のパースに失敗したか |
| debug HTML | どこに保存したか |
| login状態 | login切れの可能性 |
| issue報告情報 | 再現・報告に必要な情報 |

### CLIは短く気持ちよくする

| 基本形 | 将来的な短縮形 |
| ------------------- | -------------- |
| `cargo ac test a` | `cargo ac t a` |
| `cargo ac submit a` | `cargo ac s a` |

### AtCoderに優しくする

| 方針 | 内容 |
| ----- | -------------- |
| キャッシュ | 不要な再取得を避ける |
| ポーリング | 間隔を短くしすぎない |
| 提出 | 連打を防ぐ |
| 問題取得 | 不必要に全問題を再取得しない |
| アクセス | 過剰アクセスを避ける |

### AI連携は初期段階では入れない

| 対象 | 方針 |
| ------------- | ------- |
| AI解説生成 | 初期実装しない |
| 問題文の外部AI送信 | 初期実装しない |
| コンテスト中の外部AI連携 | 初期実装しない |

## 最終的な開発方針

本プロジェクトでは、以下の順番で価値を積み上げる。

| 順番 | 方針 |
| -- | -------------------------- |
| 1 | GitHub上で継続開発できる状態を作る |
| 2 | Codexに小さなIssueを依頼できる運用を整える |
| 3 | Rustのローカル開発体験を気持ちよくする |
| 4 | AtCoderからサンプルを取れるようにする |
| 5 | 提出まで一気通貫にする |
| 6 | AtCoder/Rust環境の変更に追従しやすくする |
| 7 | 継続的に壊れにくいツールとして公開する |

### 最初の実装ゴール

| コマンド | 目標 |
| --------------------- | ---------------- |
| `cargo ac new abc400` | 気持ちよくプロジェクト生成できる |
| `cargo ac test a` | 気持ちよくローカルテストできる |

### その後に段階的に追加する機能

| 機能 | 内容 |
| ------------ | ------------- |
| `download` | 問題・サンプル取得 |
| `login` | AtCoderログイン |
| `submit` | コード提出 |
| `watch` | 結果監視 |
| `doctor` | 環境診断 |
| `selfcheck` | AtCoder構造変更検知 |
| `env update` | Rust環境情報更新 |
