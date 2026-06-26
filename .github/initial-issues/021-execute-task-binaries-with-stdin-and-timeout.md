# Execute task binaries with stdin and timeout

## Summary

task binaryへtestcase inputを渡し、stdout、stderr、exit status、timeoutを構造化して返すlocal runnerを実装する。

## Planning metadata

* Labels: `type: feature`, `area: runner`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-local-task-runner`
* Commit: `feat(runner): execute local task binaries`

## Background

比較やCLI表示とprocess制御を分離し、子processの失敗をpanicではなくrunner結果として扱う必要がある。

## Scope

* Target crate: `ac-core`
* Target files: `crates/ac-core/src/runner.rs`, `crates/ac-core/src/lib.rs`
* contest workspaceでtaskのCargo binaryをdebugまたはrelease profileで実行する
* testcase inputをstdinへ書き、stdout、stderr、exit statusを取得する
* 設定された時間で子processを停止し、timeout結果を返す

## Requirements

* Depends on: `Define testcase file layout and discovery`, `Validate .in / .out pairs`
* task名をshell commandへ連結せず、process argumentとして渡す
* stdin書込失敗、spawn失敗、wait失敗、timeout後の停止失敗をerrorとして返す
* non-zero exitはinfrastructure errorではなくRE判定に利用できるexecution resultとする
* timeout時に子processを残さない

## Acceptance criteria

* [ ] 指定task binaryに `.in` のbytesをstdinとして渡せる
* [ ] stdoutとstderrを別々のbytesとして保持する
* [ ] successとnon-zeroのexit statusを保持する
* [ ] non-zero exitをpanicせずexecution resultとして返す
* [ ] timeoutをTLE判定可能なresultとして返し、子processを停止・回収する
* [ ] debugとreleaseを選択できるが、CLI option parsingは含まない
* [ ] fixture binaryを使うunit testがsuccess、RE、timeoutをネットワークなしで検証する

## Non-goals

* expected/actual比較、AC/WA判定、WA diff
* 結果のCLI表示またはsummary
* `cargo ac test` の引数解析と接続
* AtCoderアクセス、認証、提出

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core runner
cargo test --all
```

## Notes for implementation

timeout実装はplatform固有の大規模なprocess管理へ広げず、現在のsupported environmentで子processを確実に回収できる最小構成を選ぶ。
