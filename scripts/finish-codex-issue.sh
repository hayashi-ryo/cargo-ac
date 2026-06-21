#!/usr/bin/env bash
set -euo pipefail

issue_number="${1:-}"
commit_message="${2:-}"

if [ -z "$issue_number" ] || [ -z "$commit_message" ]; then
echo "Usage: scripts/finish-codex-issue.sh <issue-number> "<commit-message>""
echo
echo "Example:"
echo '  scripts/finish-codex-issue.sh 2 "feat(cli): add cargo-ac CLI crate"'
exit 1
fi

current_branch="$(git branch --show-current)"

if [ -z "$current_branch" ]; then
echo "Error: current branch could not be detected."
exit 1
fi

if [ "$current_branch" = "main" ]; then
echo "Error: You are on main branch. Create or switch to a feature branch first."
exit 1
fi

if [ -z "$(git status --porcelain)" ]; then
echo "Error: There are no local changes to commit."
exit 1
fi

mkdir -p .codex/pr-bodies

echo "Current branch: ${current_branch}"
echo "Issue number: #${issue_number}"
echo "Commit message: ${commit_message}"
echo

issue_title="$(gh issue view "$issue_number" --json title --jq '.title')"
issue_url="$(gh issue view "$issue_number" --json url --jq '.url')"

echo "Issue title: ${issue_title}"
echo "Issue URL: ${issue_url}"
echo

echo "Changed files:"
git status --short
echo

read -r -p "Continue with verification, commit, push, and PR creation? [y/N] " answer
case "$answer" in
[yY][eE][sS]|[yY]) ;;
*)
echo "Canceled."
exit 0
;;
esac

run_check() {
local name="$1"
shift

echo
echo "Running: $name"

if "$@"; then
echo "PASS: $name"
return 0
else
echo "FAIL: $name"
return 1
fi
}

git_diff_check_status="[ ]"
cargo_fmt_status="[ ]"
cargo_clippy_status="[ ]"
cargo_test_status="[ ]"

git_diff_check_note="OK"
cargo_fmt_note="OK"
cargo_clippy_note="OK"
cargo_test_note="OK"

if run_check "git diff --check" git diff --check; then
git_diff_check_status="[x]"
else
git_diff_check_note="Failed. Check whitespace or conflict markers."
fi

has_cargo="false"
if command -v cargo >/dev/null 2>&1; then
has_cargo="true"
fi

has_crates="false"
if compgen -G "crates/*/Cargo.toml" > /dev/null; then
has_crates="true"
fi

if [ "$has_cargo" = "true" ] && [ "$has_crates" = "true" ]; then
if run_check "cargo fmt --all -- --check" cargo fmt --all -- --check; then
cargo_fmt_status="[x]"
else
cargo_fmt_note="Failed. Run cargo fmt and check the diff."
fi

if run_check "cargo clippy --all-targets --all-features -- -D warnings" cargo clippy --all-targets --all-features -- -D warnings; then
cargo_clippy_status="[x]"
else
cargo_clippy_note="Failed. Check clippy diagnostics."
fi

if run_check "cargo test --all" cargo test --all; then
cargo_test_status="[x]"
else
cargo_test_note="Failed. Check test output."
fi
elif [ "$has_cargo" != "true" ]; then
cargo_fmt_note="Not run because cargo is not installed or not found in PATH."
cargo_clippy_note="Not run because cargo is not installed or not found in PATH."
cargo_test_note="Not run because cargo is not installed or not found in PATH."
elif [ "$has_crates" != "true" ]; then
cargo_fmt_note="Not run because no Rust crate exists under crates/*/Cargo.toml."
cargo_clippy_note="Not run because no Rust crate exists under crates/*/Cargo.toml."
cargo_test_note="Not run because no Rust crate exists under crates/*/Cargo.toml."
fi

cat > ".codex/pr-bodies/${issue_number}.md" <<EOF
## Summary

Issue #${issue_number}「${issue_title}」に対応しました。

## Related issue

Closes #${issue_number}

## Changes

- Codex作業結果を反映
- Issue本文のScope / Requirements / Acceptance criteriaに沿って変更

## Verification

- ${git_diff_check_status} git diff --check
- ${cargo_fmt_status} cargo fmt --all -- --check
- ${cargo_clippy_status} cargo clippy --all-targets --all-features -- -D warnings
- ${cargo_test_status} cargo test --all

## Verification notes

- git diff --check: ${git_diff_check_note}
- cargo fmt: ${cargo_fmt_note}
- cargo clippy: ${cargo_clippy_note}
- cargo test: ${cargo_test_note}

## Scope control

- [x] Issue範囲外の変更を含めていない
- [x] 不要なリファクタリングを含めていない
- [x] secrets / credentials を含めていない
- [x] AtCoderのlanguage_idを固定値として追加していない
- [x] AtCoderへの過剰アクセスにつながる変更を含めていない

## Notes

レビュー時は、Issue #${issue_number} のAcceptance criteriaとNon-goalsに沿って差分を確認してください。
EOF

echo
echo "Generated PR body:"
echo "----------------------------------------"
cat ".codex/pr-bodies/${issue_number}.md"
echo "----------------------------------------"
echo

read -r -p "Commit and create PR with this body? [y/N] " answer
case "$answer" in
[yY][eE][sS]|[yY]) ;;
*)
echo "Canceled before commit."
exit 0
;;
esac

git add .
git restore --staged .codex 2>/dev/null || true

git commit -m "$commit_message"

git push -u origin "$current_branch"

existing_pr_url="$(
  gh pr view "$current_branch" --json url --jq '.url' 2>/dev/null || true
)"

if [ -n "$existing_pr_url" ]; then
  echo
  echo "Pull request already exists:"
  echo "$existing_pr_url"
else
  gh pr create --base main --head "$current_branch" --title "$commit_message" --body-file ".codex/pr-bodies/${issue_number}.md"
fi

echo
echo "Switching to main..."
git switch main
