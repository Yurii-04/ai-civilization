#!/usr/bin/env bash
# Installs the pinned flatc into .tools/bin. Keep FLATC_VERSION in sync with the
# `flatbuffers` crate (crates/protocol) and npm package (packages/protocol).
set -euo pipefail

FLATC_VERSION="25.9.23"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST="$ROOT/.tools/bin"

if [[ -x "$DEST/flatc" ]] && "$DEST/flatc" --version | grep -q "$FLATC_VERSION"; then
  echo "flatc $FLATC_VERSION already installed"
  exit 0
fi

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) ASSET="Linux.flatc.binary.g++-13.zip" ;;
  Darwin-*) ASSET="Mac.flatc.binary.zip" ;;
  *) echo "No prebuilt flatc for $(uname -s)-$(uname -m); install flatc $FLATC_VERSION and set FLATC" >&2; exit 1 ;;
esac

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
curl -fsSL -o "$TMP/flatc.zip" "https://github.com/google/flatbuffers/releases/download/v$FLATC_VERSION/$ASSET"
mkdir -p "$DEST"
unzip -o -q "$TMP/flatc.zip" -d "$TMP"
install -m 0755 "$TMP/flatc" "$DEST/flatc"
"$DEST/flatc" --version
