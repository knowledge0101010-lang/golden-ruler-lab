"""이도 → 창제 / Han 키워드 치환."""

from __future__ import annotations

import json
import re
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[1]
_KEYWORDS = json.loads((_ROOT / "keywords.json").read_text(encoding="utf-8"))


def motif_table() -> list[tuple[str, str, str, str]]:
    return [
        ("함수", "fn", "func", "기능"),
        ("불변", "let", "let", "불변"),
        ("가변", "let mut", "var", "가변"),
        ("구조", "struct", "struct", "구조"),
        ("열거", "enum", "enum", "열거"),
        ("매칭", "match", "match", "맞춤"),
        ("Option", "Some/None", "Some/None", "있음/없음값"),
        ("출력", "println!", "println", "출력"),
        ("엔트리", "main", "main()", "메인"),
    ]


def _word_sub(source: str, mapping: dict[str, str]) -> str:
    out = source
    for ko, en in sorted(mapping.items(), key=lambda kv: -len(kv[0])):
        if not en:
            # drop surface particles like 이면
            out = re.sub(
                rf"(?<![A-Za-z0-9_가-힣]){re.escape(ko)}(?![A-Za-z0-9_가-힣])",
                "",
                out,
            )
            continue
        out = re.sub(
            rf"(?<![A-Za-z0-9_가-힣]){re.escape(ko)}(?![A-Za-z0-9_가-힣])",
            en,
            out,
        )
    return out


def transpile_to_cangjie(source: str) -> str:
    mapping = dict(_KEYWORDS.get("to_cangjie", {}))
    body = _word_sub(source, mapping)
    # Cangjie entry is `main()` not `func main()`
    body = re.sub(r"\bfunc\s+main\s*\(", "main(", body)
    header = "// 이도(세종)→Cangjie 자동변환 (geumcheok translate --host cangjie)\n"
    return header + body


def transpile_to_han(source: str) -> str:
    mapping = dict(_KEYWORDS.get("to_han", {}))
    body = _word_sub(source, mapping)
    header = "// 이도(세종)→Han 자동변환 (geumcheok translate --host han)\n"
    return header + body


def transpile(source: str, host: str = "cangjie") -> str:
    if host == "han":
        return transpile_to_han(source)
    if host == "cangjie":
        return transpile_to_cangjie(source)
    raise ValueError(f"unknown host: {host}")


def transpile_file(path: Path, host: str = "cangjie") -> Path:
    text = path.read_text(encoding="utf-8")
    out = transpile(text, host=host)
    suffix = ".hgl" if host == "han" else ".cj"
    dest = path.with_suffix(suffix)
    dest.write_text(out, encoding="utf-8")
    return dest
