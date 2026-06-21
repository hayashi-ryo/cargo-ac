# Review Phase 0-4 completion and align project plans

## Summary

Phase 0〜4の計画、成果物、完了済みIssue、現在の実装を全体監査し、`docs/tasks.md` の進捗、抜け漏れ、今後のロードマップを実態に合わせて更新する。

## Planning metadata

* Labels: `type: docs`, `area: docs`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `docs/<issue-number>-phase-0-4-completion-review`
* Commit: `docs: review phase 0-4 completion`

## Background

Phase 0〜3ではプロジェクト方針、repository基盤、Codexを前提としたGitHub運用、CLI骨格を整備し、Phase 4ではネットワーク非依存のcontest workspace生成を実装した。

監査対象は以下とする。

| Phase | 主な対象 |
| --- | --- |
| Phase 0 | コンセプト、初期MVP、対象範囲、Non-goals |
| Phase 1 | repository構成、README、license、基本docs、Rust開発基盤 |
| Phase 2 | Issue・PR・branch・commit・CI・Codex運用ルール |
| Phase 3 | Rust workspace、`cargo-ac`、`ac-core`、CLI構造、placeholder、CLI error handling、Docker開発環境 |
| Phase 4 | `ac.toml`、workspace生成component、`cargo ac new`、integration test |

Phase 4の主な実装Issueは以下である。

| Issue | 内容 |
| --- | --- |
| #18 | Define `ac.toml` data model |
| #19 | Implement `ac.toml` read/write |
| #20 | Add contest workspace generator foundation |
| #21 | Generate contest `Cargo.toml` |
| #22 | Generate default task sources and testcase directories |
| #23 | Implement `cargo ac new` command |
| #24 | Add contest workspace generation integration tests |

一方、`docs/tasks.md` にはPhase 0〜4の成果物や完了条件と現在のrepository状態が一致していない可能性があり、Phase 4のcheckboxや推奨Issue分割には実装前の状態が残っている。また、Phase 4の目的に `new` / `init` と記載されているが、Phase 4の実装Issueは `cargo ac new` のみを対象としている。

Phase 5以降へ進む前に、Phase 0からの計画と実態の差分を通して監査し、完了済み項目と未対応項目を明確にする必要がある。

## Scope

* `docs/tasks.md` のPhase 0〜4について、目的、タスク、完了条件をrepositoryの成果物と照合する
* Phase 0〜3の完了済みIssue、merged PR、docs、設定、CI、CLI実装を確認する
* Issue #18〜#24のScope、Acceptance criteria、実装、テスト結果を照合する
* `docs/tasks.md` のPhase 0〜4のタスクと完了条件を、確認できた事実に基づいて更新する
* Phase 0〜4で予定していた内容に抜け漏れがないか確認し、結果を `docs/tasks.md` に記録する
* `docs/tasks.md` の推奨Issue分割を実際のIssue #18〜#24と整合させる、または旧計画であることを明記する
* Phase 4の `init`、`--force` など、実装IssueのNon-goalsと一致しない記載を整理する
* Phase 5以降の作業内容と順序を、現在の実装および `.github/initial-issues/016-phase-5-and-later-implementation-backlog.md` と照合する
* 計画にずれがある場合のみ `docs/ROADMAP.md` と `docs/tasks.md` を更新する
* 監査で見つかった文書のずれは、このIssue内で関連文書を更新する
* 今後のPhase完了時にもPhase 0から全体監査するルールを恒常的な指示へ追加する

## Requirements

* checkboxはコード、テスト、merged Issueなどの根拠を確認してから更新する
* 未実装項目を完了扱いにしない
* 実装漏れが見つかった場合、このIssue内では実装せず、未完了タスクまたは後続Issue候補として記録する
* 各Phaseの完了判定を区別し、Phase 4完了判定とPhase 5開始条件を混同しない
* `docs/tasks.md` と `docs/ROADMAP.md` のPhase名、目的、順序を一致させる
* 実際のGitHub Issue番号と旧計画上の仮番号を混同しない記述にする
* 将来機能を新たに追加せず、既存計画の整合性確認に限定する
* AtCoderへのネットワークアクセスを行わない

### Audit points

少なくとも以下を確認する。

* Phase 0のプロジェクト目的、初期MVP、Non-goalsが現在のdocsと `AGENTS.md` で矛盾していないこと
* Phase 1のrepository、license、README、基本docs、Rust関連ファイルが実在し、記載と一致していること
* Phase 2のIssue・PR template、CI、branch・commit・Codex運用ルールが実在し、現行運用と一致していること
* Phase 3のRust workspace、2 crate構成、CLI help、placeholder handler、CLI error boundary、Docker開発環境が現在も成立していること
* `cargo ac new abc400` がworkspace一式を生成できること
* `Cargo.toml`、`ac.toml`、`src/bin/a.rs`〜`f.rs`、`testcases/a/`〜`f/` が整合していること
* 生成workspaceで `cargo check --bins` が成功すること
* 既存pathと不正入力が保護されていること
* workspace生成のintegration testが存在すること
* Phase 4がAtCoderへのHTTPアクセスに依存していないこと
* `init` や `--force` がPhase 4完了条件として誤って残っていないこと
* Phase 5のtest runner作業が、現在のworkspace layoutと `ac.toml` modelを前提にできること

## Acceptance criteria

* [ ] Phase 0〜4の目的、タスク、完了条件が成果物と照合されている
* [ ] Phase 0〜3のdocs、repository設定、CI、CLI基盤について抜け漏れが確認されている
* [ ] Issue #18〜#24と実装の対応関係が確認されている
* [ ] `docs/tasks.md` のPhase 0〜4のタスクと完了条件が実態に合わせて更新されている
* [ ] Phase 0〜4の抜け漏れがある場合は未完了項目または後続Issue候補として明記されている
* [ ] 抜け漏れがないPhaseは、確認した根拠とともに完了が明記されている
* [ ] `docs/tasks.md` の古いIssue番号または旧Issue分割が現在のIssue構成と混同されない状態になっている
* [ ] `init`、`--force`などの記載がPhase 4の実装範囲と整合している
* [ ] Phase 5以降の目的と作業順序が現在の実装およびbacklogと整合している
* [ ] 必要な場合は `docs/ROADMAP.md` が更新され、不要な場合は変更しない理由がPR本文に記載されている
* [ ] 未実施タスクと、このIssueで解消した文書差分が区別して記録されている
* [ ] 今後のPhase完了時にPhase 0から全体監査するルールが文書化されている
* [ ] 実装コードやテストコードを変更していない

## Non-goals

* Phase 0〜4の追加機能実装
* 発見した抜け漏れの修正
* Phase 5のtest runner実装
* `cargo ac init`、`--force`、`--template`の追加
* Phase 5以降の全候補Issue本文の作成
* AtCoderへのアクセス
* GitHub IssueやPull Requestの一括変更

## Verification

以下で文書差分と既存実装の状態を確認する。

```bash
git diff --check
git diff -- docs/tasks.md docs/ROADMAP.md
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

加えて、`docs/tasks.md` のPhase 0〜4、推奨Issue分割、milestone、Phase 5以降の記載を相互に読み合わせる。

## Notes for implementation

各Phaseの完了を示すためだけに未対応項目を削除したり完了扱いにしたりしない。repository外のGitHub設定などローカルから確認できない項目は、未確認であることを明記し、推測でcheckboxを更新しない。`init`や`--force`が今後も必要と判断した場合は、Phase 4の完了条件から分離し、将来候補であることが分かる位置へ移す。

Phase 5以降については、既存のbacklogを基準に依存順とIssue粒度を確認する。このIssueで新しい機能要件を確定させる必要はない。
