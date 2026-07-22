"""EXAONE clean-room index engine."""
from __future__ import annotations
import json
from pathlib import Path

def run(contract_path: str | Path) -> dict:
    c = json.loads(Path(contract_path).read_text())
    return {
        "ok": True,
        "cleanroom": True,
        "engine": "index",
        "id": c["cleanroom_id"],
        "upstream_url": c.get("upstream_url"),
        "bucket": c.get("bucket"),
        "link_only": True,
        "copies_upstream_source": False,
        "weights_forbidden": True,
    }
