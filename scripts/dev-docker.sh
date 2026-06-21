#!/usr/bin/env bash

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
image_name="cargo-ac-dev"

docker build --tag "${image_name}" --file "${repository_root}/.devcontainer/Dockerfile" "${repository_root}"
docker run --rm --interactive --tty \
  --volume "${repository_root}:/workspace" \
  --workdir /workspace \
  "${image_name}"
