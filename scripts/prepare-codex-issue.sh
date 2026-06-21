#!/usr/bin/env bash
set -euo pipefail

issue_number="${1:-}"
branch_name="${2:-}"

if [ -z "$issue_number" ] || [ -z "$branch_name" ]; then
  echo "Usage: scripts/prepare-codex-issue.sh <issue-number> <branch-name>"
  echo
  echo "Example:"
  echo "  scripts/prepare-codex-issue.sh 5 feature/5-define-clap-command-structure"
  exit 1
fi

mkdir -p .codex/issues
mkdir -p .codex/prompts

echo "Fetching issue #${issue_number}..."

issue_title="$(gh issue view "$issue_number" --json title --jq '.title')"
issue_url="$(gh issue view "$issue_number" --json url --jq '.url')"
issue_labels="$(gh issue view "$issue_number" --json labels --jq '[.labels[].name] | join(", ")')"
issue_milestone="$(gh issue view "$issue_number" --json milestone --jq '.milestone.title // "-"')"

{
  echo "# Issue #${issue_number}: ${issue_title}"
  echo
  echo "URL: ${issue_url}"
  echo
  echo "Labels: ${issue_labels}"
  echo
  echo "Milestone: ${issue_milestone}"
  echo
  echo "---"
  echo
  gh issue view "$issue_number" --json body --jq '.body'
} > ".codex/issues/${issue_number}.md"

cat > ".codex/prompts/${issue_number}.md" <<EOF
\`.codex/issues/${issue_number}.md\` を読んで、Issue #${issue_number}「${issue_title}」を実装してください。

作業前に \`AGENTS.md\` を読んでください。
Issue本文のScope、Requirements、Acceptance criteria、Non-goalsを守ってください。
Issue範囲外の変更や不要なリファクタリングは行わないでください。

この作業では、Issue本文に書かれていない追加機能は実装しないでください。
判断に迷う場合は、最も単純な実装を選び、PR本文に前提を書いてください。

作業後は可能な範囲で以下を実行してください。

- \`cargo fmt --all -- --check\`
- \`cargo clippy --all-targets --all-features -- -D warnings\`
- \`cargo test --all\`

実行できない場合は、理由をPR本文に書いてください。

作業完了後の報告では、ローカル絶対パスではなくリポジトリ相対パスでファイル名を書いてください。
EOF

echo "Switching to main..."
git switch main

echo "Pulling latest main..."
git pull origin main

echo "Creating branch: ${branch_name}"
git switch -c "$branch_name"

echo
echo "Prepared Codex prompt:"
echo "----------------------------------------"
cat ".codex/prompts/${issue_number}.md"
echo "----------------------------------------"

if command -v pbcopy >/dev/null 2>&1; then
  pbcopy < ".codex/prompts/${issue_number}.md"
  echo
  echo "Prompt copied to clipboard."
fi

echo
echo "Next step:"
echo "  codex"
echo
echo "Then paste the prompt above."