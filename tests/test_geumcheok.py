"""이도 (세종) 단위 테스트."""

from __future__ import annotations

import unittest
from pathlib import Path

from geumcheok.lex import keyword_set, tokenize
from geumcheok.translate import transpile, transpile_to_cangjie, transpile_to_han

_ROOT = Path(__file__).resolve().parents[1]
_EXAMPLE = _ROOT / "examples" / "안녕.ido"


class TestTranslate(unittest.TestCase):
    def test_cangjie_keywords(self) -> None:
        src = "기능 f(값: 정수): 정수 { 반환 값 }\n메인() { 불변 x = 1\n출력(\"a\") }"
        out = transpile_to_cangjie(src)
        self.assertIn("func f", out)
        self.assertIn("Int64", out)
        self.assertIn("return", out)
        self.assertIn("main(", out)
        self.assertNotIn("func main", out)
        self.assertIn("let x", out)
        self.assertIn("println", out)

    def test_han_aliases(self) -> None:
        src = "기능 f() { 불변 x = 1 }"
        out = transpile_to_han(src)
        self.assertIn("함수", out)
        self.assertIn("상수", out)

    def test_example_file(self) -> None:
        text = _EXAMPLE.read_text(encoding="utf-8")
        cj = transpile(text, host="cangjie")
        self.assertIn("func double", cj)
        self.assertIn("println", cj)


class TestLex(unittest.TestCase):
    def test_keywords_detected(self) -> None:
        src = _EXAMPLE.read_text(encoding="utf-8")
        kws = keyword_set(src)
        self.assertIn("기능", kws)
        self.assertIn("불변", kws)
        self.assertIn("가변", kws)
        self.assertIn("출력", kws)
        self.assertIn("메인", kws)

    def test_tokenize_count(self) -> None:
        toks = tokenize("기능 a() { 반환 1 }")
        kinds = [t.kind for t in toks]
        self.assertIn("keyword", kinds)
        self.assertTrue(any(t.value == "기능" for t in toks))


if __name__ == "__main__":
    unittest.main()
