# Implement AtCoder sample parser for Japanese and English statements

## Summary

AtCoder task statementのHTML文字列から、日本語UIと英語UIのsample input/outputを対応付けて抽出するparserを `ac-core` に追加する。

## Planning metadata

* Labels: `type: feature`, `area: parser`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `feature/<issue-number>-sample-parser`
* Commit: `feat(parser): parse task samples`

## Background

Phase 6の中心は、task pageからlocal test runnerで使えるsample testcaseを取得することである。sample parserはHTTP取得やfile outputから分離し、fixture HTMLで日本語UIと英語UIの両方を検証できるようにする。

## Scope

* Target crate: `ac-core`
* Target files: `crates/ac-core/src/parser.rs` または sample parser module、parser fixture test filesまたはinline fixtures
* task page HTML文字列を入力にしてsample input/output pairの一覧を返す
* `入力例`、`出力例`、`Sample Input`、`Sample Output`、`pre` 要素を扱う
* 入力例と出力例を番号で正しく対応付ける
* `pre` 要素内の改行と空白をlocal testに必要な範囲で保持する
* HTTP取得、debug HTML保存、file output、CLI接続は実装しない

## Requirements

* Depends on: no implementation dependency; can run in parallel with `Implement AtCoder contest task list parser`
* Parser inputはHTML文字列、contest id、task idに限定し、HTTP clientへ依存しない
* 日本語UIの `入力例 N` / `出力例 N` と英語UIの `Sample Input N` / `Sample Output N` を扱う
* sample番号の欠落、input/output不一致、`pre` 欠落でpanicせずparse errorを返す
* errorにはoperation、contest id、task id、不一致の概要を含める

## Acceptance criteria

* [ ] 日本語UI fixtureからsample input/output pairを抽出できる
* [ ] 英語UI fixtureからsample input/output pairを抽出できる
* [ ] `入力例`、`出力例`、`Sample Input`、`Sample Output`、`pre` の扱いがtestで検証されている
* [ ] 複数sampleのinput/outputが正しい番号で対応付く
* [ ] `pre` 内の改行と空白がlocal testに必要な範囲で保持される
* [ ] input/output不一致や欠落でpanicせずparse errorを返す
* [ ] parser testがHTTP client、file system、ネットワークに依存しない

## Non-goals

* contest task list parser
* task page HTML取得
* sample file保存、`ac.toml`更新
* debug HTML保存
* `cargo ac download` の実装
* login、Cookie、CSRF token、提出、watch

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core parser
cargo test --all
```

## Notes for implementation

AtCoder HTML構造変更を検知しやすいようにfixture testを重視する。debug HTML保存は別Issueで扱うため、このIssueではparse errorに必要なcontextを返すところまでに留める。
