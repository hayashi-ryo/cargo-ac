# Create Phase 5 implementation issue drafts

## Summary

Phase 5のlocal test runnerを実装可能な単位へ分割し、Codex-readyなIssue本文を `.github/initial-issues/` 配下にMarkdownファイルとして作成する。

## Planning metadata

* Labels: `type: docs`, `area: docs`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `docs/<issue-number>-phase-5-issue-drafts`
* Commit: `docs: draft Phase 5 implementation issues`

## Background

Phase 4までに、`ac.toml`、contest workspace生成、task sourceとtestcase directoryの生成、`cargo ac new`、およびworkspace生成のintegration testが整備されている。Phase 5では、このworkspaceを前提として `cargo ac test <task>` によるlocal sample testと `cargo ac addcase <task>` による自作case追加を実装する。

`.github/initial-issues/016-phase-5-and-later-implementation-backlog.md` には、testcase discovery、runner、結果比較・表示、`addcase`、integration testの候補がある。一方、`docs/tasks.md` のPhase 5実装順序にある `cargo ac test <task>`、`all`、`--release` のCLI接続は、backlogで独立したIssue候補になっていない。

runnerの中核処理とCLI接続の責務を混在させず、各Issueの依存関係と完了条件を明確にしてから実装へ進む必要がある。

## Scope

* `.github/initial-issues/016-phase-5-and-later-implementation-backlog.md` と `docs/tasks.md` のPhase 5要件を照合する
* Phase 5の各候補を、1つのIssueで実装・検証しやすい粒度のIssue本文へ展開する
* 次の「個別Issue候補」に対応するMarkdownファイルを `.github/initial-issues/` 配下へ作成する
* 各IssueにPlanning metadata、依存するIssue、対象crateまたは対象file、実装範囲、検証方法を記載する
* 個別Issue間の依存順を整理する
* `cargo ac test` のCLI接続Issueをbacklogに不足している個別Issue候補として追加する
* Phase 5の計画と個別Issue候補にずれが残る場合は、`.github/initial-issues/016-phase-5-and-later-implementation-backlog.md` のPhase 5部分を更新する

### Individual issue candidates

| Order | Issue candidate | Labels | Branch suffix | Commit |
| --- | --- | --- | --- | --- |
| 1 | Define testcase file layout and discovery | `type: feature`, `area: testcase`, `agent: codex-ready` | `testcase-discovery` | `feat(testcase): discover testcase files` |
| 2 | Validate `.in` / `.out` pairs | `type: feature`, `area: testcase`, `agent: codex-ready` | `validate-testcase-pairs` | `feat(testcase): validate testcase pairs` |
| 3 | Execute task binaries with stdin and timeout | `type: feature`, `area: runner`, `agent: codex-ready` | `local-task-runner` | `feat(runner): execute local task binaries` |
| 4 | Normalize and compare output | `type: feature`, `area: runner`, `agent: codex-ready` | `compare-test-output` | `feat(runner): compare testcase output` |
| 5 | Display AC, WA, RE, and TLE results | `type: feature`, `area: cli`, `agent: codex-ready` | `test-result-output` | `feat(test): display testcase results` |
| 6 | Display WA diff | `type: feature`, `area: runner`, `agent: codex-ready` | `wa-diff-output` | `feat(runner): display wrong answer diff` |
| 7 | Connect `cargo ac test` to the local test runner | `type: feature`, `area: cli`, `agent: codex-ready` | `test-command` | `feat(cli): connect test command` |
| 8 | Implement `cargo ac addcase` | `type: feature`, `area: testcase`, `agent: codex-ready` | `addcase-command` | `feat(testcase): add custom testcase` |
| 9 | Add test runner integration tests | `type: test`, `area: runner`, `agent: codex-ready` | `test-runner-integration` | `test(runner): cover local test runner` |

`Connect cargo ac test to the local test runner` では、少なくとも `cargo ac test <task>`、`cargo ac test all`、`cargo ac test <task> --release` をCLIから既存のcore処理へ接続する。runner実装、結果表示、CLI引数解析を同じIssueで新規設計しない。

## Requirements

* 各Issue本文に `Summary`、`Planning metadata`、`Background`、`Scope`、`Requirements`、`Acceptance criteria`、`Non-goals`、`Verification`、`Notes for implementation` を含める
* 各IssueのPlanning metadataに想定labels、milestone、branch、commitを記載する
* Milestoneは `v0.1.0-alpha.2: local workspace and test runner` とする
* branchは `<type>/<issue-number>-<short-description>`、commitはConventional Commits形式とする
* 各Issueに先行Issueまたは前提となるcomponentを明記し、実際のIssue番号が未確定の場合は候補名で参照する
* `cargo-ac` はCLI層、`ac-core` はtestcase管理とrunnerの中核処理を担当し、`ac-core` から `clap` へ依存させない
* 外部入力、file I/O、子processの失敗、timeoutをpanicではなくerrorまたはtest resultとして扱う
* testcase layoutはPhase 4で生成する `src/bin/<task>.rs` と `testcases/<task>/` を前提とする
* `.in` / `.out` pairの欠落、重複、読込失敗に対する期待動作をAcceptance criteriaへ記載する
* runner Issueではstdin、stdout、stderr、exit status、timeoutの扱いを明記する
* 比較Issueでは末尾改行と空白のnormalization範囲を明記し、過剰な正規化で誤答をACにしない
* 表示IssueではAC、WA、RE、TLEとsummaryの責務を明記する
* WA diff Issueではexpectedとactualを識別できる出力をAcceptance criteriaへ含める
* CLI接続Issueでは`<task>`、`all`、`--release`の挙動、無効なtask、testcase不在、終了statusをAcceptance criteriaへ含める
* `addcase` Issueではinputとexpected outputの受付、`custom-N.in` / `custom-N.out` の保存、既存番号との衝突回避を扱う
* integration test IssueではAC、WA、RE、TLE、および主要なCLI経路のうち、先行Issueのunit testだけでは保証できない範囲を対象にする
* test fixture用binaryや一時workspaceを使い、testがAtCoderアクセスや利用者環境の既存contest workspaceに依存しないようにする
* すべてのIssueをネットワーク非依存とし、認証、Cookie、提出、AtCoderアクセスを含めない
* 実装Issueの作成に限定し、このIssue内ではRust codeやtest codeを変更しない

## Dependency order

Issue本文では、少なくとも次の依存関係を維持する。

1. testcase discoveryを定義した後にpair validationを実装する
2. discoveryとvalidationを前提にtask binary runnerを実装する
3. runnerが取得したoutputを使ってnormalizationと比較を実装する
4. runnerと比較結果を前提にAC、WA、RE、TLE表示とWA diffを実装する
5. core側のrunnerと結果model、CLI表示が利用可能になった後に `cargo ac test` を接続する
6. `addcase` は確定したtestcase layoutを前提とするが、runnerのCLI接続とは独立して進められるようにする
7. integration testは `cargo ac test` と `cargo ac addcase` の対象範囲が揃った後に追加する

独立して実装可能なIssueまで不要に直列化しない。依存先が同じ場合は、その旨を記載したうえで並行実装可能な構成にする。

## Acceptance criteria

* [ ] Phase 5の9件の個別Issue候補が、それぞれ独立したMarkdownファイルとして作成されている
* [ ] `cargo ac test` のCLI接続Issueが個別Issue候補に追加されている
* [ ] CLI接続Issueに `<task>`、`all`、`--release` が含まれている
* [ ] 各Issueに必須セクションとPlanning metadataがある
* [ ] 各Issueに対象crateまたは対象fileと依存関係が明記されている
* [ ] testcase、runner、比較、表示、CLI接続の責務がcrate境界に沿って分離されている
* [ ] 各IssueのAcceptance criteriaが実装完了を客観的に判定できる内容になっている
* [ ] 各IssueのVerificationに、変更範囲に応じたunit testまたはintegration testとRust標準checkが記載されている
* [ ] `.github/initial-issues/016-phase-5-and-later-implementation-backlog.md` と作成したIssue候補の不一致が解消されている
* [ ] Phase 5 IssueにAtCoderアクセス、認証、Cookie、提出処理が含まれていない
* [ ] Rust codeとtest codeを変更していない

## Non-goals

* Phase 5機能の実装
* unit testまたはintegration test codeの追加
* `cargo ac test` または `cargo ac addcase` のCLI挙動変更
* Phase 6以降のIssue本文作成
* AtCoderへのアクセス
* 認証、Cookie、CSRF token、提出処理の実装
* GitHub Issueの実登録
* milestone、label、GitHub Projectの変更
* Pull Requestの作成

## Verification

以下で文書差分と追加したIssue draftを確認する。

```bash
git diff --check
git diff --stat
ls .github/initial-issues
```

加えて、作成した全Issue本文を `.github/initial-issues/016-phase-5-and-later-implementation-backlog.md`、`docs/tasks.md` のPhase 5、`docs/ROADMAP.md` と読み合わせ、次を確認する。

* 9件の候補が欠けていないこと
* `cargo ac test` のCLI接続が独立したIssueであること
* 各IssueのScopeとNon-goalsが重複または矛盾していないこと
* 依存順に循環がないこと
* `cargo-ac` と `ac-core` のcrate境界が守られていること
* すべてのVerificationがネットワーク非依存であること

文書のみの変更であるため、Rust codeへ差分がないことを確認した場合は `cargo fmt`、`cargo clippy`、`cargo test` を省略できる。その場合はPR本文のVerificationに理由を記載する。

## Notes for implementation

Issue draftのfile名先頭に使う連番は、既存の `.github/initial-issues/` と衝突しない次の番号から割り当てる。file名の連番をGitHub Issue番号として扱わず、GitHub登録後に確定するIssue番号はbranch名や依存関係へ反映する。

既存のCLI skeletonに `test` や `addcase` のplaceholderがある場合も、Issue本文作成時に現在の引数定義とhandler境界を確認する。既存構造で表現できる範囲は維持し、Issue draft内で大規模なCLI再設計やcrate再編を要求しない。

`cargo ac test` のCLI接続Issueは、core機能をCLIから利用可能にする境界の作業である。結果表示Issueとの重複を避けるため、どちらが表示modelを定義し、どちらがcommand引数と終了statusを接続するかをそれぞれのScopeとNon-goalsで明確にする。
