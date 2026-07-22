#!/usr/bin/env python3
"""이도 (세종) CLI — motif / translate / run / lex."""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[1]
if str(_ROOT) not in sys.path:
    sys.path.insert(0, str(_ROOT))

from geumcheok import __version__  # noqa: E402
from geumcheok.lex import keyword_set, tokenize  # noqa: E402
from geumcheok.translate import motif_table, transpile, transpile_file  # noqa: E402


def _cmd_motif(_: argparse.Namespace) -> int:
    print(f"이도(Ido) · 금척(Golden Ruler) {__version__} · 2 degrees")
    print(f"{'개념':8} {'Rust':12} {'창제':12} {'이도'}")
    print("-" * 48)
    for concept, rust, cj, ido in motif_table():
        print(f"{concept:8} {rust:12} {cj:12} {ido}")
    print()
    print("Golden Ruler · 倉頡×Prometheus · 복희여와 규+자 · 이도원/EXAONE 사칭 아님 · host=cangjie")
    return 0


def _cmd_translate(ns: argparse.Namespace) -> int:
    src = Path(ns.file)
    if not src.exists():
        print(f"파일 없음: {src}", file=sys.stderr)
        return 1
    dest = transpile_file(src, host=ns.host)
    print(f"변환 완료 ({ns.host}): {src.name} → {dest}")
    return 0


def _cmd_lex(ns: argparse.Namespace) -> int:
    src = Path(ns.file)
    text = src.read_text(encoding="utf-8")
    toks = tokenize(text)
    kws = keyword_set(text)
    print(f"tokens={len(toks)} keywords={kws}")
    if ns.verbose:
        for t in toks[:80]:
            print(f"  {t.kind:8} {t.value!r} @{t.line}:{t.col}")
    return 0


def _run_cangjie(src: Path) -> int:
    cj_text = transpile(src.read_text(encoding="utf-8"), host="cangjie")
    # Strip leading auto-header comments for package file — keep body
    smoke = _ROOT / "smoke_cjpm"
    main_cj = smoke / "src" / "main.cj"
    # Ensure package + body
    body = cj_text
    if "package " not in body:
        body = "package smoke_cjpm\n\n" + body
    main_cj.write_text(body, encoding="utf-8")

    # geumcheok → claude-science → vendor → cangjie-mini-ide
    env_setup = (
        _ROOT.parents[3] / "system" / "untitled-bridge" / "envsetup-mac105.sh"
    )
    if not env_setup.is_file():
        env_setup = Path.home() / "Projects/cangjie-mini-ide/system/untitled-bridge/envsetup-mac105.sh"

    if not shutil.which("cjpm") and not env_setup.is_file():
        out = src.with_suffix(".cj")
        out.write_text(cj_text, encoding="utf-8")
        print(f"cjpm/env 없음 — 변환만 저장: {out}", file=sys.stderr)
        return 2

    script = f"""
set -euo pipefail
if [ -f "{env_setup}" ]; then
  # shellcheck disable=SC1090
  source "{env_setup}"
fi
cd "{smoke}"
cjpm build
cjpm run
"""
    print("Cangjie 호스트: cjpm build/run (smoke_cjpm)")
    return int(subprocess.call(["bash", "-lc", script]))


def _run_han(src: Path) -> int:
    dest = transpile_file(src, host="han")
    han_dir = _ROOT.parent / "zodiac-lab" / "han"
    if not han_dir.is_dir():
        print(f"Han 호스트 없음: {han_dir}", file=sys.stderr)
        print(f"변환본만 생성됨: {dest}", file=sys.stderr)
        return 2
    cmd = ["cargo", "run", "--quiet", "--", "interpret", str(dest)]
    print(f"Han 실행: {' '.join(cmd)}")
    return int(subprocess.call(cmd, cwd=str(han_dir)))


def _cmd_run(ns: argparse.Namespace) -> int:
    src = Path(ns.file)
    if not src.exists():
        print(f"파일 없음: {src}", file=sys.stderr)
        return 1
    if ns.host == "han":
        return _run_han(src)
    return _run_cangjie(src)


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(
        prog="geumcheok",
        description="이도(Ido) · 금척(Golden Ruler/geumcheok) · 에이전트 2 degrees",
    )
    p.add_argument("--version", action="version", version=f"%(prog)s {__version__}")
    sub = p.add_subparsers(dest="cmd", required=True)

    m = sub.add_parser("motif", help="Rust/창제/이도 대응표")
    m.set_defaults(func=_cmd_motif)

    t = sub.add_parser("translate", help=".ido → .cj 또는 .hgl")
    t.add_argument("file")
    t.add_argument("--host", choices=("cangjie", "han"), default="cangjie")
    t.set_defaults(func=_cmd_translate)

    r = sub.add_parser("run", help="변환 후 호스트 실행")
    r.add_argument("file")
    r.add_argument("--host", choices=("cangjie", "han"), default="cangjie")
    r.set_defaults(func=_cmd_run)

    lx = sub.add_parser("lex", help="최소 렉서 (키워드 토큰)")
    lx.add_argument("file")
    lx.add_argument("-v", "--verbose", action="store_true")
    lx.set_defaults(func=_cmd_lex)

    ns = p.parse_args(argv)
    return int(ns.func(ns))


if __name__ == "__main__":
    raise SystemExit(main())
