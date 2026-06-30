# Save debug HTML on parse failure

## Summary

download workflowでparser failureが発生したときに、調査用debug HTMLを保存し、error出力へ保存先pathを含められるdiagnosticsを追加する。

## Planning metadata

* Labels: `type: feature`, `area: parser`, `area: diagnostics`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.3: problem download`
* Branch: `feature/<issue-number>-debug-html-on-parse-failure`
* Commit: `feat(parser): save debug HTML on parse failure`

## Background

AtCoderのHTML構造は変更される可能性がある。parser failure時に取得HTMLを残せると調査しやすい一方、通常成功時に不要なdebug fileを作るべきではない。保存責務はparser本体ではなくdownload workflowのdiagnosticsとして分離する。

## Scope

* Target crate: `ac-core`
* Target files: parser/download diagnostics module、error type、必要なunit test
* parse failure時のみdebug HTMLを保存する処理を追加する
* 保存先pathをerrorに含め、CLIが表示できるようにする
* operation、contest id、task id、URL、debug HTML pathを可能な範囲でerror contextに含める
* debug HTMLに認証情報が含まれないよう注意する
* parser実装、HTTP client実装、CLI handler実装は主目的にしない

## Requirements

* Depends on: `Implement AtCoder contest task list parser`, `Implement AtCoder sample parser for Japanese and English statements`
* Related component: `Add HTTP client foundation for AtCoder pages`
* debug HTML保存はparse failure時に限定する
* 保存先pathはerrorとして呼び出し元に伝える
* 公開ページのHTMLを対象にし、Cookie、CSRF token、passwordなどの秘密情報を保存しない
* file I/O失敗時もpanicせず、元のparse failureと保存失敗を調査できるerrorにする

## Acceptance criteria

* [ ] parse failure時にdebug HTMLが保存される
* [ ] parse成功時にはdebug HTMLを保存しない
* [ ] errorにdebug HTML pathが含まれる
* [ ] errorにoperation、contest id、task idまたはURLが含まれる
* [ ] debug HTML保存失敗時もpanicしない
* [ ] 保存処理がparserのHTML抽出責務と分離されている
* [ ] login、Cookie、CSRF token、提出処理が含まれていない

## Non-goals

* parser仕様の拡張
* HTTP clientの新規設計
* `cargo ac download` のCLI接続
* 認証済みページのHTML保存
* CookieやCSRF tokenのsanitize処理を前提にした認証機能

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core
cargo test --all
```

## Notes for implementation

Phase 6では公開ページのみを扱う。将来の認証ページ対応ではsanitize方針を別Issueで見直す。保存先は既存のlocal debug output方針と衝突しない、利用者が見つけやすい場所にする。
