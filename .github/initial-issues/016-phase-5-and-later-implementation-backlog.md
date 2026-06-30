# Phase 5 and later implementation issue backlog

## Summary

Phase 5からPhase 10までを、今後個別のCodex-ready Issueへ展開するための候補一覧として整理する。

## Background

Phase 4完了後はlocal test runnerから着手し、その後にAtCoderアクセス、認証、提出を段階的に導入する。外部アクセスや秘密情報を扱う作業をローカル機能と混在させないため、Issue境界を先に定義する。

## Scope

* Phase 5〜10のIssue候補と依存順を整理する
* 想定label、milestone、branch、commitの命名方針を示す
* AtCoderアクセス、認証、Cookie、提出を要注意Issueとして識別する
* 個別Issue化するときに必要なセクションを定義する

## Requirements

個別Issueへ展開する際は、各Issueに `Summary`、`Background`、`Scope`、`Requirements`、`Acceptance criteria`、`Non-goals`、`Verification`、`Notes for implementation` を含める。

すべてのbranchは `<type>/<issue-number>-<short-description>`、commitはConventional Commits形式とする。

### Phase 5: Local test runner

* Milestone: `v0.1.0-alpha.2: local workspace and test runner`

| Issue candidate | Labels | Branch suffix | Commit |
| --- | --- | --- | --- |
| Define testcase file layout and discovery | `type: feature`, `area: testcase`, `agent: codex-ready` | `testcase-discovery` | `feat(testcase): discover testcase files` |
| Validate `.in` / `.out` pairs | `type: feature`, `area: testcase`, `agent: codex-ready` | `validate-testcase-pairs` | `feat(testcase): validate testcase pairs` |
| Execute task binaries with stdin and timeout | `type: feature`, `area: runner`, `agent: codex-ready` | `local-task-runner` | `feat(runner): execute local task binaries` |
| Normalize and compare output | `type: feature`, `area: runner`, `agent: codex-ready` | `compare-test-output` | `feat(runner): compare testcase output` |
| Display AC, WA, RE, and TLE results | `type: feature`, `area: cli`, `agent: codex-ready` | `test-result-output` | `feat(test): display testcase results` |
| Display WA diff | `type: feature`, `area: runner`, `agent: codex-ready` | `wa-diff-output` | `feat(runner): display wrong answer diff` |
| Connect `cargo ac test` to the local test runner | `type: feature`, `area: cli`, `agent: codex-ready` | `test-command` | `feat(cli): connect test command` |
| Implement `cargo ac addcase` | `type: feature`, `area: testcase`, `agent: codex-ready` | `addcase-command` | `feat(testcase): add custom testcase` |
| Add test runner integration tests | `type: test`, `area: runner`, `agent: codex-ready` | `test-runner-integration` | `test(runner): cover local test runner` |

### Phase 6: AtCoder download

公開contest pageとtask pageからproblem samplesを取得する。認証、Cookie、CSRF token、提出、watchはPhase 7以降で扱う。

* Milestone: `v0.1.0-alpha.3: problem download`

| Issue candidate | Labels | Branch suffix | Commit |
| --- | --- | --- | --- |
| Add HTTP client foundation for AtCoder pages | `type: feature`, `area: atcoder`, `area: network`, `agent: codex-ready` | `http-client-foundation` | `feat(atcoder): add HTTP client foundation` |
| Implement AtCoder contest task list parser | `type: feature`, `area: parser`, `agent: codex-ready` | `contest-task-list-parser` | `feat(parser): parse contest task list` |
| Implement AtCoder sample parser for Japanese and English statements | `type: feature`, `area: parser`, `agent: codex-ready` | `sample-parser` | `feat(parser): parse task samples` |
| Save downloaded samples and task metadata | `type: feature`, `area: testcase`, `area: config`, `agent: codex-ready` | `save-downloaded-samples` | `feat(download): save samples and task metadata` |
| Save debug HTML on parse failure | `type: feature`, `area: parser`, `area: diagnostics`, `agent: codex-ready` | `debug-html-on-parse-failure` | `feat(parser): save debug HTML on parse failure` |
| Implement `cargo ac download <contest>` | `type: feature`, `area: cli`, `area: download`, `agent: codex-ready` | `download-command` | `feat(cli): implement download command` |
| Implement `cargo ac new <contest> --download` | `type: feature`, `area: cli`, `area: download`, `agent: codex-ready` | `new-download-option` | `feat(cli): add new --download option` |
| Add download workflow integration tests | `type: test`, `area: download`, `area: parser`, `agent: codex-ready` | `download-integration-tests` | `test(download): cover download workflow` |

ParserはHTML文字列を入力とするfixture-based componentとしてHTTP clientから分離する。download workflowはHTTP client、parser、file writer、debug HTML保存を統合し、integration testはfixture HTMLやmock HTTP layerを使ってAtCoder実サイト、認証、Cookie、利用者環境に依存しない構成にする。

### Phase 7: Login and session management

認証、Cookie、CSRF tokenを扱うため、すべて `agent: needs-review-carefully` を追加する。

* Milestone: `v0.1.0-alpha.4: login and submit`

| Issue candidate | Labels | Branch suffix | Commit |
| --- | --- | --- | --- |
| Parse login form and CSRF token from fixtures | `type: feature`, `area: auth`, `agent: needs-review-carefully` | `login-form-parser` | `feat(auth): parse AtCoder login form` |
| Add secure session storage abstraction | `type: feature`, `area: auth`, `agent: needs-review-carefully` | `session-storage` | `feat(auth): add session storage` |
| Implement login HTTP flow | `type: feature`, `area: auth`, `agent: needs-review-carefully` | `login-http-flow` | `feat(auth): implement AtCoder login` |
| Connect `cargo ac login` to auth flow | `type: feature`, `area: cli`, `agent: needs-review-carefully` | `login-command` | `feat(cli): connect login command` |

### Phase 8: Submit and result watch

提出とAtCoderアクセスを扱うため、すべて `agent: needs-review-carefully` を追加する。

* Milestone: `v0.1.0-alpha.4: login and submit`

| Issue candidate | Labels | Branch suffix | Commit |
| --- | --- | --- | --- |
| Parse submit form and language options from fixtures | `type: feature`, `area: submit`, `agent: needs-review-carefully` | `submit-form-parser` | `feat(submit): parse submit form` |
| Resolve Rust language_id without hardcoding | `type: feature`, `area: submit`, `agent: needs-review-carefully` | `rust-language-resolver` | `feat(submit): resolve Rust language id` |
| Implement submit preflight checks | `type: feature`, `area: submit`, `agent: needs-review-carefully` | `submit-preflight` | `feat(submit): add submission preflight checks` |
| Implement confirmed submit POST | `type: feature`, `area: submit`, `agent: needs-review-carefully` | `submit-post` | `feat(submit): post AtCoder submission` |
| Poll submission result at a safe interval | `type: feature`, `area: submit`, `agent: needs-review-carefully` | `submission-watch` | `feat(submit): watch submission result` |

### Phase 9: Maintenance features

* Milestone: Issue登録時にmaintenance features用milestoneを作成して割り当てる

| Issue candidate | Labels | Branch suffix | Commit |
| --- | --- | --- | --- |
| Implement local `doctor` checks | `type: feature`, `area: doctor`, `agent: codex-ready` | `doctor-local-checks` | `feat(doctor): add local environment checks` |
| Implement fixture-based `selfcheck` parser checks | `type: feature`, `area: parser`, `agent: codex-ready` | `selfcheck-fixtures` | `feat(parser): add fixture selfcheck` |
| Define AtCoder Rust environment data model | `type: feature`, `area: env`, `agent: codex-ready` | `env-data-model` | `feat(env): define Rust environment data` |
| Implement `cargo ac env show` | `type: feature`, `area: env`, `agent: codex-ready` | `env-show-command` | `feat(env): show Rust environment data` |
| Update environment data from reviewed source | `type: feature`, `area: env`, `agent: needs-review-carefully` | `env-update-command` | `feat(env): update Rust environment data` |
| Add low-frequency compatibility workflow | `type: ci`, `area: ci`, `agent: needs-review-carefully` | `compatibility-workflow` | `ci: add AtCoder compatibility check` |

### Phase 10: Publishing and maintenance

* Milestone: Issue登録時にrelease用milestoneを作成して割り当てる

| Issue candidate | Labels | Branch suffix | Commit |
| --- | --- | --- | --- |
| Complete package metadata and README installation guide | `type: docs`, `area: docs`, `agent: codex-ready` | `release-metadata` | `docs: prepare crate release metadata` |
| Add packaging verification | `type: ci`, `area: ci`, `agent: codex-ready` | `package-verification` | `ci: verify cargo package` |
| Prepare crates.io release checklist | `type: docs`, `area: docs`, `agent: codex-ready` | `crates-io-checklist` | `docs: add crates.io release checklist` |
| Define compatibility and release maintenance process | `type: docs`, `area: docs`, `agent: codex-ready` | `maintenance-process` | `docs: define release maintenance process` |

## Acceptance criteria

* [ ] Phase 5〜10の候補IssueがPhase別に整理されている
* [ ] 各候補にlabels、branch suffix、commit messageがある
* [ ] AtCoderアクセスを含む候補が明示されている
* [ ] 認証、Cookie、提出を含む候補に `agent: needs-review-carefully` がある
* [ ] parser fixture作業がHTTPアクセスから分離されている
* [ ] 個別Issue化に必要な必須セクションが明記されている

## Non-goals

* 候補Issue本文の完全な作成
* GitHub Issueの実登録
* 実装コードやテストコードの変更
* AtCoderへのアクセス
* 認証、Cookie、提出処理の実装

## Verification

Docker環境内で以下を実行する。

```bash
git diff --stat
ls .github/initial-issues
```

表を確認し、外部アクセス・認証・提出Issueがローカル機能から分離されていることを確認する。

## Notes for implementation

個別Issue登録時に、実際のIssue番号をbranch名へ設定し、対象releaseのmilestoneを確定する。外部アクセスIssueはアクセス頻度、fixture、秘密情報の非表示をAcceptance criteriaへ必ず含める。
