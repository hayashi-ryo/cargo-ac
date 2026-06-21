# 開発ロードマップ

## 目的

このドキュメントは、`cargo-ac` の開発ロードマップを整理するためのものです。

`cargo-ac` は、RustでAtCoderに参加する人のためのCLIツールです。
まずはローカル開発体験を整え、その後AtCoder連携、提出、環境診断、継続運用機能を段階的に追加します。

## 現在の進捗

2026-06-21にPhase 0〜4の計画と成果物を監査した。

| Phase | 状態 | Notes |
| --- | --- | --- |
| Phase 0 | 完了 | コンセプト、初期MVP、Non-goals、READMEを確認済み |
| Phase 1 | 完了 | repository基盤とGitHub Topicsを確認済み |
| Phase 2 | 完了 | Issue Formsを修正し、Project boardをWeb UIで確認済み |
| Phase 3 | 完了 | CLI skeletonとDocker開発環境を確認済み |
| Phase 4 | 完了 | `cargo ac new`とworkspace生成integration testを確認済み |
| Phase 5 | 次の実装対象 | ローカルtest runnerをIssue単位で実装する |

詳細な監査結果と未完了項目は `docs/tasks.md` を参照してください。

## Phase 0: コンセプト・仕様整理

プロジェクトの目的、対象範囲、初期MVP、初期段階でやらないことを整理します。

## Phase 1: GitHubリポジトリ初期作成

GitHubリポジトリを作成し、README、dual license、.gitignore、docs配下の初期ドキュメントを整備します。

## Phase 2: Codex前提のGitHub運用整備

Codexに作業を依頼しやすいように、`AGENTS.md`、Issue、PR、branch、commit、CI、Project boardの運用ルールを整備します。

## Phase 3: CLI骨格作成

`cargo ac` として起動できるCLIの骨格を作成します。

## Phase 4: ローカルプロジェクト生成

`cargo ac new <contest>` により、AtCoder用Rustプロジェクトを生成できるようにします。

## Phase 5: ローカルテスト実行

`cargo ac test <task>` により、ローカルでサンプルテストを実行できるようにします。

## Phase 6: 問題ダウンロード

AtCoderの問題ページからサンプル入力・出力を取得できるようにします。

## Phase 7: ログイン・セッション管理

AtCoderへのログイン状態を管理できるようにします。

## Phase 8: 提出・結果監視

RustコードをAtCoderへ提出し、提出結果を監視できるようにします。

## Phase 9: doctor / selfcheck / env update

ローカル環境診断、AtCoder側の構造変更検知、AtCoder Rust環境情報の管理を実装します。

## Phase 10: crates.io公開・継続運用

基本機能が安定したら、crates.ioへ公開し、継続的に保守できる状態にします。
