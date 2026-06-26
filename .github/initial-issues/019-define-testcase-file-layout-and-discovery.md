# Define testcase file layout and discovery

## Summary

Phase 4で生成されるtestcase directoryから、taskごとのtestcase fileを決定的な順序で検出する。

## Planning metadata

* Labels: `type: feature`, `area: testcase`, `priority: high`, `agent: codex-ready`
* Milestone: `v0.1.0-alpha.2: local workspace and test runner`
* Branch: `feature/<issue-number>-testcase-discovery`
* Commit: `feat(testcase): discover testcase files`

## Background

Phase 5のpair validation、runner、`addcase`が同じlayoutを共有するため、filesystem探索を先に `ac-core` へ分離する。

## Scope

* Target crate: `ac-core`
* Target files: `crates/ac-core/src/testcase.rs`, `crates/ac-core/src/lib.rs`
* `ac.toml` の `testcase_directory` とtaskの `bin_name` から `testcases/<task>/` を解決する
* flatなtask directory直下の `.in` と `.out` を検出し、logical case nameとpathを保持する
* filesystemの列挙順に依存しない順序を定義する

## Requirements

* 前提component: Phase 4の `ProjectConfig`、`TaskConfig`、`testcases/<task>/` layout
* filesystem accessとpath解決はCLIに依存させない
* directory不在、列挙失敗、file metadata取得失敗をtyped errorとして返し、panicしない
* subdirectoryと対象外extensionはtestcaseとして扱わない

## Acceptance criteria

* [ ] `src/bin/<task>.rs` に対応する `testcases/<task>/` から `.in` / `.out` fileを検出できる
* [ ] 検出結果がlogical case nameの決定的な順序で返る
* [ ] 対象外fileとsubdirectoryがtestcaseに混入しない
* [ ] testcase directory不在と読込失敗がpathを含むerrorになる
* [ ] pair欠落と重複候補は捨てず、後続validationが判定できる検出結果として保持される
* [ ] 同じlogical case nameの候補を黙って上書きしない
* [ ] discoveryのunit testが一時directoryを使い、ネットワークに依存しない

## Non-goals

* `.in` / `.out` pairの妥当性判定
* testcase内容の読込、binary実行、output比較
* recursive discoveryまたは新しいlayoutの追加
* CLI変更、AtCoderアクセス、認証、提出

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p ac-core testcase
cargo test --all
```

## Notes for implementation

後続Issueがfile名を再解析しないよう、logical case nameとinput/output種別を表す最小限の型を返す。pair判定はこのIssueへ含めない。
