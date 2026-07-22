"""Dispatch CONTRACT.json for EXAONE clean-room."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
from engines import index_engine

def main(contract_path: str) -> dict:
    c = json.loads(Path(contract_path).read_text())
    kind = c.get("kind", "index-card")
    if kind in ("index-card", "archive-card", "model-card", "eval-card", "examples-card"):
        return index_engine.run(contract_path)
    return {"ok": False, "error": f"unknown kind {kind}"}

if __name__ == "__main__":
    cp = sys.argv[1] if len(sys.argv) > 1 else "CONTRACT.json"
    print(json.dumps(main(cp), ensure_ascii=False))
