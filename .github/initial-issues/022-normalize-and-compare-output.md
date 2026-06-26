# Normalize and compare output

## Summary

expected outputとtask stdoutを、AtCoderの判定を過度に緩めない範囲でnormalizeして比較する。

## Planning metadata

* Labels: `type: feature`, `area: runner`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-compare-test-output`
* Commit: `feat(runner): compare testcase output`

## Background

末尾改行の有無だけでWAにせず、内部の改行、空白、token境界を壊す正規化で誤答をACにしない比較規則が必要である。

## Scope

* Target crate: `ac-core`
* Target files: `crates/ac-core/src/runner.rs` または `crates/ac-core/src/output.rs`, `crates/ac-core/src/lib.rs`
* expected/actualのline endingを `LF` として扱う
* file全体末尾の改行と各行末尾のspace/tabを比較時に無視する
* normalized expected/actualと一致結果を保持するmodelを定義する

## Requirements

* Depends on: `Execute task binaries with stdin and timeout`
* bytesがUTF-8でない場合は比較errorとして返し、panicしない
* 行内のspace/tab、空行、token順序は保持する
* stdout以外のstderrとexit statusを比較対象に混ぜない

## Acceptance criteria

* [ ] `LF` と `CRLF` の違いでWAにならない
* [ ] file末尾改行の有無でWAにならない
* [ ] 各行末尾のspace/tabだけの違いでWAにならない
* [ ] 行内空白、余分な空行、異なるtokenを同一と判定しない
* [ ] invalid UTF-8がpanicせずerrorになる
* [ ] 境界条件を含むunit testがある

## Non-goals

* 浮動小数点誤差やspecial judge対応
* whitespace token単位ですべてを比較する緩い判定
* process実行、CLI表示、WA diff formatting
* AtCoderアクセス、認証、提出

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core output
cargo test --all
```

## Notes for implementation

正規化規則は1つの関数に閉じ、expectedとactualへ対称に適用する。将来のjudge mode追加はこのIssueに含めない。
