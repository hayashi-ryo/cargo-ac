# Create implementation issue drafts for Phase 4 and later

## 概要

Phase 3でCLI骨格の整備が完了したため、Phase 4以降の実装作業をGitHub Issueとして整理する。

このIssueでは、実装そのものは行わず、Codexが後続Issueを実装しやすいように、`.github/initial-issues/` 配下へIssue本文Markdownを作成する。

## 背景

`cargo-ac` は、RustでAtCoderに参加する人のためのCLIツールである。

Phase 3までに以下が完了した。

* Rust workspace作成
* `cargo-ac` CLI crate追加
* `ac-core` library crate追加
* `clap` によるCLI構造定義
* placeholder command handlers追加
* CLI境界の基本的なエラーハンドリング追加
* Dockerベースの開発・検証環境追加

今後は、`cargo ac new` によるローカルプロジェクト生成を起点に、ローカルテスト、問題ダウンロード、ログイン、提出などを段階的に実装する。

## 対応範囲

Phase 4以降の作業を、Codexが実装しやすい粒度のIssueに分解する。

特にPhase 4は詳細にIssue化する。

対象ロードマップは以下とする。

* Phase 4: ローカルプロジェクト生成
* Phase 5: ローカルテスト実行
* Phase 6: 問題ダウンロード
* Phase 7: ログイン・セッション管理
* Phase 8: 提出・結果監視
* Phase 9: doctor / selfcheck / env update
* Phase 10: crates.io公開・継続運用

## 要件

* `.github/initial-issues/` 配下にIssue本文Markdownを追加する

* 既存のIssue本文スタイルに合わせる

* 各Issueには以下を含める

  * Summary
  * Background
  * Scope
  * Requirements
  * Acceptance criteria
  * Non-goals
  * Verification
  * Notes for implementation

* 各Issueは、Codexが1回の作業で対応しやすい粒度にする

* 大きすぎるIssueは分割する

* Phase 4ではAtCoderへのネットワークアクセスを含めない

* 認証、Cookie、提出、AtCoderアクセスが必要なIssueは明確に分離する

* 認証、Cookie、提出、AtCoderアクセスが絡むIssueには `agent: needs-review-carefully` を付ける想定にする

* Docker環境で検証できるようにVerificationを書く

* 各Issueについて、想定label、milestone、branch名、commit messageを整理する

## 受け入れ条件

* [ ] Phase 4の実装Issue案が複数のMarkdownファイルとして作成されている
* [ ] Phase 5以降のIssue案が必要な粒度で作成、または今後作成するためのTODOとして整理されている
* [ ] 各IssueにScope、Requirements、Acceptance criteria、Non-goals、Verificationが含まれている
* [ ] 各IssueがCodexに依頼しやすい粒度になっている
* [ ] 各Issueに想定label、milestone、branch名、commit messageが分かる情報がある
* [ ] Phase 4のIssueにはAtCoderネットワークアクセスが含まれていない
* [ ] 認証、Cookie、提出、AtCoderアクセスが必要なIssueは別Issueとして分離されている
* [ ] 実装コードは変更していない

## 対象外

* `cargo-ac` の実装コード変更
* CLI挙動の変更
* テストコード追加
* AtCoderへのアクセス
* ログイン処理
* 提出処理
* 問題ダウンロード処理
* GitHub Issueの実登録
* PR作成

## 検証方法

以下を確認する。

```bash
git diff --stat
ls .github/initial-issues
```

生成されたIssue Markdownを読み、以下を確認する。

* 1Issueが大きすぎないこと
* Acceptance criteriaが実装完了判定に使えること
* Non-goalsが明確であること
* Docker環境で検証できるVerificationになっていること
* Phase 4にAtCoderネットワークアクセスが混ざっていないこと

## 実装メモ

Phase 4では、まず `cargo ac new` のローカルプロジェクト生成を扱う。

想定される分割例は以下。

* 生成先ディレクトリ構造の設計
* AtCoder Rust環境に合わせた `Cargo.toml` テンプレート生成
* `src/bin/<task>.rs` の生成
* sample testcase用ディレクトリ生成
* 既存ディレクトリがある場合のエラー処理
* `--force` や `--template` などのオプションは必要に応じて後続Issueに分離

ただし、最終的な分割はCodexが既存コードとロードマップを確認して判断してよい。
