# Validate `.in` / `.out` pairs

## Summary

discovery結果を検証し、実行可能なtestcase pairまたは原因を特定できるerrorへ変換する。

## Planning metadata

* Labels: `type: feature`, `area: testcase`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-validate-testcase-pairs`
* Commit: `feat(testcase): validate testcase pairs`

## Background

inputだけ、outputだけ、重複したlogical caseをrunnerへ渡すと誤った結果になるため、実行前に一括検証する。

## Scope

* Target crate: `ac-core`
* Target files: `crates/ac-core/src/testcase.rs`
* discoveryが返す候補をlogical case nameごとに対応付ける
* validなpairの内容を読み込み、runnerへ渡すmodelを定義する
* 欠落、重複、読込失敗を区別する

## Requirements

* Depends on: `Define testcase file layout and discovery`
* inputとexpected outputはbytesとして読み込み、UTF-8をrunner実行の前提にしない
* 外部入力とfile I/Oの失敗をtyped errorとして返し、panicしない
* 複数の不正pairを検出できる場合は、少なくともcase nameと不足・重複した側を報告する

## Acceptance criteria

* [ ] 1つの `.in` と1つの `.out` を同じstemのpairとして返せる
* [ ] `.in` 欠落と `.out` 欠落がそれぞれcase nameを含むerrorになる
* [ ] inputまたはoutputの重複候補を黙って選択せずerrorにする
* [ ] inputまたはoutputの読込失敗がfile pathを含むerrorになる
* [ ] validな複数pairがdiscovery順を維持する
* [ ] 欠落、重複、読込失敗のunit testがある

## Non-goals

* filesystem discovery規則の変更
* binary実行、timeout、output比較
* testcase fileの自動修復または生成
* CLI表示、AtCoderアクセス、認証、提出

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core testcase
cargo test --all
```

## Notes for implementation

実filesystemでは同名fileを2つ作れないため、重複は候補列を受け取るvalidation unit testでも保証する。最初の候補を暗黙採用しない。
