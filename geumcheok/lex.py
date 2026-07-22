"""이도 최소 렉서 (2단계 프론트엔드 stub)."""

from __future__ import annotations

import json
import re
from dataclasses import dataclass
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[1]
_KEYWORDS = json.loads((_ROOT / "keywords.json").read_text(encoding="utf-8"))
_IDO_KW = set(_KEYWORDS.get("to_cangjie", {}).keys())

_TOKEN = re.compile(
    r"""
    (?P<comment>//[^\n]*)
  | (?P<string>"(?:\\.|[^"\\])*")
  | (?P<number>\d+(?:\.\d+)?)
  | (?P<ident>[A-Za-z_가-힣][A-Za-z0-9_가-힣]*)
  | (?P<op>->|=>|<:|==|!=|<=|>=|[+\-*/%=<>(){}\[\],.:|])
  | (?P<ws>\s+)
  | (?P<other>.)
    """,
    re.VERBOSE,
)


@dataclass(frozen=True)
class Token:
    kind: str
    value: str
    line: int
    col: int


def tokenize(source: str) -> list[Token]:
    tokens: list[Token] = []
    line, col = 1, 1
    pos = 0
    while pos < len(source):
        m = _TOKEN.match(source, pos)
        if not m:
            break
        kind = m.lastgroup or "other"
        val = m.group(0)
        if kind == "ident" and val in _IDO_KW:
            kind = "keyword"
        if kind != "ws" and kind != "comment":
            tokens.append(Token(kind, val, line, col))
        # advance position
        for ch in val:
            if ch == "\n":
                line += 1
                col = 1
            else:
                col += 1
        pos = m.end()
    return tokens


def keyword_set(source: str) -> list[str]:
    return sorted({t.value for t in tokenize(source) if t.kind == "keyword"})
