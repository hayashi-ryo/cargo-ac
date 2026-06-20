#!/usr/bin/env bash
set -euo pipefail

issue_number="${1:?Usage: scripts/fetch-issue.sh <issue-number>}"

mkdir -p .codex/issues

gh issue view "$issue_number" \
  --json number,title,body,labels,milestone,url \
  --jq '"# Issue #\(.number): \(.title)\n\nURL: \(.url)\n\nLabels: \([.labels[].name] | join(", "))\n\nMilestone: \(.milestone.title // "-")\n\n---\n\n\(.body)"' \
  > ".codex/issues/${issue_number}.md"

echo "Saved to .codex/issues/${issue_number}.md"
