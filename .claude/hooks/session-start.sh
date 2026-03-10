#!/bin/bash
set -euo pipefail

# Only run in remote (web) environments
if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

# Redirect all stdout to stderr so the hook runner doesn't
# misinterpret command output as a hook response.
exec 1>&2

# Install gh CLI if not already installed
if ! command -v gh &>/dev/null; then
  GH_VERSION=$(curl -sL "https://api.github.com/repos/cli/cli/releases/latest" \
    -H "Authorization: token ${GITHUB_TOKEN:-}" | grep -oP '"tag_name":\s*"v\K[^"]+')
  if [ -z "$GH_VERSION" ]; then
    echo "Warning: Failed to fetch gh version from GitHub API, using fallback"
    GH_VERSION="2.87.3"
  fi
  curl -sL "https://github.com/cli/cli/releases/download/v${GH_VERSION}/gh_${GH_VERSION}_linux_amd64.tar.gz" \
    | tar xz -C /tmp
  cp "/tmp/gh_${GH_VERSION}_linux_amd64/bin/gh" /usr/local/bin/gh
  rm -rf "/tmp/gh_${GH_VERSION}_linux_amd64"
fi

# Authenticate gh using GITHUB_TOKEN if available
if [ -n "${GITHUB_TOKEN:-}" ] && ! gh auth status &>/dev/null; then
  echo "$GITHUB_TOKEN" | gh auth login --with-token
fi

# Install cargo-make from pre-built binary if not already installed
if ! command -v cargo-make &>/dev/null; then
  CM_VERSION=$(curl -sL "https://api.github.com/repos/sagiegurari/cargo-make/releases/latest" \
    -H "Authorization: token ${GITHUB_TOKEN:-}" | grep -oP '"tag_name":\s*"\K[^"]+')
  if [ -z "$CM_VERSION" ]; then
    echo "Warning: Failed to fetch cargo-make version from GitHub API, using fallback"
    CM_VERSION="0.37.24"
  fi
  curl -sL "https://github.com/sagiegurari/cargo-make/releases/download/${CM_VERSION}/cargo-make-v${CM_VERSION}-x86_64-unknown-linux-musl.zip" \
    -o /tmp/cargo-make.zip
  unzip -qo /tmp/cargo-make.zip -d /tmp/cargo-make
  cp /tmp/cargo-make/cargo-make-v${CM_VERSION}-x86_64-unknown-linux-musl/cargo-make /root/.cargo/bin/cargo-make
  rm -rf /tmp/cargo-make.zip /tmp/cargo-make
fi
