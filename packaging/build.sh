#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "==> Building OpenWire CLI (Release binary)..."
cd "$ROOT_DIR"
cargo build --release

echo "==> Packaging Arch Linux package..."
cd "$SCRIPT_DIR"
makepkg -f

echo "==> Package build completed successfully!"
