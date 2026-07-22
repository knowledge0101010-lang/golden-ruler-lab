#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/../.." && pwd)"
out="$(python3 "$ROOT/engines/dispatch.py" "$DIR/CONTRACT.json")"
echo "$out" | grep -q '"ok": true'
echo OK "$(basename "$DIR")"
