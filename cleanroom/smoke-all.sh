#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
ok=0; fail=0
while IFS= read -r d; do
  if bash "$d/smoke.sh" >/dev/null 2>&1; then ok=$((ok+1)); else echo FAIL "$d"; fail=$((fail+1)); fi
done < <(find "$ROOT/modules" -mindepth 1 -maxdepth 1 -type d | sort)
echo "geumcheok-cleanroom smoke ok=$ok fail=$fail total=$((ok+fail))"
test "$fail" -eq 0
test "$ok" -eq 11
