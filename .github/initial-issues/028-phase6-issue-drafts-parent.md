# Create Phase 6 implementation issue drafts

## Summary

Phase 6のproblem downloadを実装可能な単位へ分割し、Codex-readyなIssue本文を `.github/initial-issues/` 配下にMarkdownファイルとして作成する。

## Planning metadata

* Labels: `type: docs`, `area: docs`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `docs/<issue-number>-phase-6-issue-drafts`
* Commit: `docs: draft Phase 6 implementation issues`

## Background

Phase 5までに、contest workspace生成、task sourceとtestcase directoryの生成、`cargo ac new`、`cargo ac test <task>` によるlocal sample test、`cargo ac addcase <task>` による自作case追加、およびlocal test runnerの基礎が整備されている。

Phase 6では、AtCoderのコンテストページから問題一覧を取得し、各問題ページからサンプル入力・出力を取得して、既存のworkspace layoutに保存できるようにする。

対象コマンドは次のとおり。

| Command | Purpose |
| --- | --- |
| `cargo ac download <contest>` | コンテストの問題・サンプルを取得する |
| `cargo ac new <contest> --download` | プロジェクト生成と同時にサンプルを取得する |

Phase 6ではHTTP access、HTML parsing、sample extraction、file output、`ac.toml`更新、CLI接続が関係する。これらを1つのIssueで実装すると責務が混在し、fixture testやparse failure時の調査が難しくなる。

そのため、HTTP client、contest task list parser、sample parser、file writer、debug HTML保存、CLI接続を分離し、各Issueの依存関係と完了条件を明確にしてから実装へ進む必要がある。

## Scope

* `docs/tasks.md` のPhase 6要件を確認する
* 必要に応じて `.github/initial-issues/016-phase-5-and-later-implementation-backlog.md` または後続Phaseのbacklog記述を確認する
* Phase 6の各候補を、1つのIssueで実装・検証しやすい粒度のIssue本文へ展開する
* 次の「個別Issue候補」に対応するMarkdownファイルを `.github/initial-issues/` 配下へ作成する
* 各IssueにPlanning metadata、依存するIssue、対象crateまたは対象file、実装範囲、検証方法を記載する
* 個別Issue間の依存順を整理する
* Phase 6の計画と個別Issue候補にずれが残る場合は、backlogまたは関連docsのPhase 6部分を更新する

### Individual issue candidates

| Order | Issue candidate | Labels | Branch suffix | Commit |
| --- | --- | --- | --- | --- |
| 1 | Add HTTP client foundation for AtCoder pages | `type: feature`, `area: atcoder`, `area: network`, `agent: codex-ready` | `http-client-foundation` | `feat(atcoder): add HTTP client foundation` |
| 2 | Implement AtCoder contest task list parser | `type: feature`, `area: parser`, `agent: codex-ready` | `contest-task-list-parser` | `feat(parser): parse contest task list` |
| 3 | Implement AtCoder sample parser for Japanese and English statements | `type: feature`, `area: parser`, `agent: codex-ready` | `sample-parser` | `feat(parser): parse task samples` |
| 4 | Save downloaded samples and task metadata | `type: feature`, `area: testcase`, `area: config`, `agent: codex-ready` | `save-downloaded-samples` | `feat(download): save samples and task metadata` |
| 5 | Save debug HTML on parse failure | `type: feature`, `area: parser`, `area: diagnostics`, `agent: codex-ready` | `debug-html-on-parse-failure` | `feat(parser): save debug HTML on parse failure` |
| 6 | Implement `cargo ac download <contest>` | `type: feature`, `area: cli`, `area: download`, `agent: codex-ready` | `download-command` | `feat(cli): implement download command` |
| 7 | Implement `cargo ac new <contest> --download` | `type: feature`, `area: cli`, `area: download`, `agent: codex-ready` | `new-download-option` | `feat(cli): add new --download option` |
| 8 | Add download workflow integration tests | `type: test`, `area: download`, `area: parser`, `agent: codex-ready` | `download-integration-tests` | `test(download): cover download workflow` |

`Implement cargo ac download <contest>` では、HTTP client、contest task list parser、sample parser、file writer、debug HTML保存を既存のcore処理として利用し、CLI引数解析や終了status、ユーザー向け出力に責務を絞る。HTTP client、parser、file writerを同じIssueで新規設計しない。

`Implement cargo ac new <contest> --download` では、既存の `new` 処理と `download` workflowを接続する。download処理を複製せず、内部APIを再利用する。

`Add download workflow integration tests` では、可能な限りfixture HTMLやlocal mockを使い、testがAtCoderの実サイト、ネットワーク状態、認証、Cookie、利用者環境に依存しないようにする。

## Requirements

* 各Issue本文に `Summary`、`Planning metadata`、`Background`、`Scope`、`Requirements`、`Acceptance criteria`、`Non-goals`、`Verification`、`Notes for implementation` を含める
* 各IssueのPlanning metadataに想定labels、milestone、branch、commitを記載する
* Milestoneは `v0.1.0-alpha.3: problem download` とする
* branchは `<type>/<issue-number>-<short-description>`、commitはConventional Commits形式とする
* 各Issueに先行Issueまたは前提となるcomponentを明記し、実際のIssue番号が未確定の場合は候補名で参照する
* `cargo-ac` はCLI層、`ac-core` はdownload workflow、parser、testcase保存、config更新などの中核処理を担当し、`ac-core` から `clap` へ依存させない
* HTTP client、HTML parser、file writer、CLI handlerの責務を分離する
* parserはHTML文字列を入力として受け取り、HTTP clientに依存しない
* file writerはparserやHTTP clientに過度に依存せず、構造化されたtask/sample情報を保存する責務に絞る
* 外部入力、HTTP失敗、HTML parse failure、file I/O失敗、config更新失敗をpanicではなくerrorとして扱う
* errorには可能な範囲で、operation、contest id、task id、URL、HTTP status、debug HTML pathなど、issue報告に役立つ情報を含める
* AtCoder task list parserでは、contest tasks pageからtask id、task title、task URLまたはURL pathを取得する
* sample parserでは、日本語UIと英語UIの両方に対応する
* sample parserでは、少なくとも次の表記を扱う
  * `入力例`
  * `出力例`
  * `Sample Input`
  * `Sample Output`
  * `pre`
* sample parserでは、入力例と出力例を正しい番号で対応付ける
* sample parserでは、`pre`要素内の改行と空白をlocal testに必要な範囲で保持する
* sample保存では、既存workspace layoutの `testcases/<task>/` を前提とする
* sample保存では、`sample-N.in` と `sample-N.out` の命名を使い、番号は1始まりとする
* `ac.toml` へのtask情報書き込み範囲を各Issueに明記する
* 既存の `cargo ac new`、`cargo ac test`、`cargo ac addcase` の挙動を壊さない
* `cargo ac download <contest>` では、少なくとも `abc400` を対象に実行できることを完了条件に含める
* `cargo ac new <contest> --download` では、project生成後にdownload workflowを実行する
* parse failure時にはdebug HTMLを保存し、保存先pathをerror出力に含める
* parser fixture testsを追加し、日本語UIと英語UIのsample抽出を検証する
* integration testはAtCoder実サイトへのアクセスに依存しない構成を優先する
* 実装Issueの作成に限定し、このIssue内ではRust codeやtest codeを変更しない

## Dependency order

Issue本文では、少なくとも次の依存関係を維持する。

1. HTTP client foundationを定義した後に、実サイト取得を伴うdownload workflowへ接続する
2. contest task list parserとsample parserは、HTTP clientに依存しないfixtureベースのparserとして実装する
3. contest task list parserとsample parserは並行実装可能にする
4. sample保存と`ac.toml`更新は、parserが返す構造化データを前提にする
5. debug HTML保存は、parserとdownload workflowのerror handling方針を前提にする
6. `cargo ac download <contest>` は、HTTP client、contest task list parser、sample parser、file writer、debug HTML保存を統合する
7. `cargo ac new <contest> --download` は、`cargo ac download <contest>` と同じ内部download workflowを再利用する
8. integration testは、download commandとnew --downloadの対象範囲が揃った後に追加する

独立して実装可能なIssueまで不要に直列化しない。特に、contest task list parserとsample parserは、fixture HTMLを使って並行実装可能な構成にする。

## Acceptance criteria

* [ ] Phase 6の8件の個別Issue候補が、それぞれ独立したMarkdownファイルとして作成されている
* [ ] 各Issueに必須セクションとPlanning metadataがある
* [ ] 各Issueに対象crateまたは対象fileと依存関係が明記されている
* [ ] HTTP client、parser、file writer、CLI接続の責務がcrate境界に沿って分離されている
* [ ] parser IssueがHTTP clientに依存しない内容になっている
* [ ] sample parser Issueに日本語UIと英語UIの両対応が含まれている
* [ ] sample parser Issueに `入力例`、`出力例`、`Sample Input`、`Sample Output`、`pre` の扱いが含まれている
* [ ] sample parser Issueに入力例・出力例の対応付けが含まれている
* [ ] sample保存Issueに `testcases/<task>/sample-N.in/out` の保存仕様が含まれている
* [ ] sample保存Issueに `ac.toml` へのtask情報書き込みが含まれている
* [ ] parse failure Issueにdebug HTML保存とerror出力へのpath表示が含まれている
* [ ] `cargo ac download <contest>` のCLI接続Issueに `cargo ac download abc400` の完了条件が含まれている
* [ ] `cargo ac new <contest> --download` のCLI接続Issueが個別Issue候補に含まれている
* [ ] integration test IssueがAtCoder実サイト、認証、Cookie、利用者環境に依存しない方針になっている
* [ ] 各IssueのAcceptance criteriaが実装完了を客観的に判定できる内容になっている
* [ ] 各IssueのVerificationに、変更範囲に応じたunit test、fixture test、integration test、Rust標準checkが記載されている
* [ ] Phase 6 Issueに提出処理、login、Cookie永続化、CSRF token、watch、judge結果取得が含まれていない
* [ ] Rust codeとtest codeを変更していない

## Non-goals

* Phase 6機能の実装
* HTTP client、parser、file writer、CLI handlerのRust code変更
* unit testまたはintegration test codeの追加
* `cargo ac download` または `cargo ac new --download` のCLI挙動変更
* Phase 7以降のIssue本文作成
* AtCoderへの実アクセス
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

加えて、作成した全Issue本文を `docs/tasks.md` のPhase 6、`docs/ROADMAP.md`、および既存の `.github/initial-issues/` 内のbacklog文書と読み合わせ、次を確認する。

* 8件の候補が欠けていないこと
* `cargo ac download <contest>` のCLI接続が独立したIssueであること
* `cargo ac new <contest> --download` のCLI接続が独立したIssueであること
* HTTP client、parser、file writer、CLI接続のScopeとNon-goalsが重複または矛盾していないこと
* parser IssueがHTTP clientに依存していないこと
* sample parser Issueが日本語UIと英語UIを対象にしていること
* debug HTML保存の責務がerror handlingとdiagnosticsに限定されていること
* 依存順に循環がないこと
* `cargo-ac` と `ac-core` のcrate境界が守られていること
* integration testがAtCoder実サイト、認証、Cookie、利用者環境に依存しないこと
* Phase 6 Issueに提出処理、login、Cookie永続化、CSRF token、watch、judge結果取得が混入していないこと

文書のみの変更であるため、Rust codeへ差分がないことを確認した場合は `cargo fmt`、`cargo clippy`、`cargo test` を省略できる。その場合はPR本文のVerificationに理由を記載する。

## Notes for implementation

Issue draftのfile名先頭に使う連番は、既存の `.github/initial-issues/` と衝突しない次の番号から割り当てる。file名の連番をGitHub Issue番号として扱わず、GitHub登録後に確定するIssue番号はbranch名や依存関係へ反映する。

既存のCLI skeletonに `download`、`new`、`test`、`addcase` のplaceholderや実装済みhandlerがある場合は、Issue本文作成時に現在の引数定義とhandler境界を確認する。既存構造で表現できる範囲は維持し、Issue draft内で大規模なCLI再設計やcrate再編を要求しない。

AtCoderのHTML構造は将来変更される可能性があるため、parser Issueではfixture testとparse failure時のdiagnosticsを重視する。ただし、debug HTML保存そのものは独立Issueとして扱い、sample parserやcontest task list parserのIssueにfile output責務を混ぜない。

HTTP client Issueでは、将来のlogin、Cookie、CSRF token、submit処理に拡張できる余地を残してよいが、このPhase 6では認証や提出を実装しない。公開contest pageとtask pageのHTML取得に必要な最小限のclient foundationに留める。

`cargo ac download <contest>` のIssueでは、download workflowの統合に集中する。parserやfile writerの仕様変更が必要になった場合は、既存Issueの成果物を尊重し、必要最小限の調整に留める。

`cargo ac new <contest> --download` のIssueでは、download workflowを複製しない。project生成後に同じ内部処理を呼び出す構成にし、エラー時に「project生成は完了したがdownloadに失敗した」のか、「project生成前に失敗した」のかが分かるようにする。

integration test Issueでは、fixture HTML、temporary workspace、mock HTTP layerなどを使い、CIがネットワークやAtCoderのページ変更で不安定にならないようにする。実サイトを使った手動確認が必要な場合は、unit/integration testではなくManual verificationとしてIssue本文に分離して記載する。
