# Clean-room protocol — LG-AI-EXAONE **roles** → Cursor **금척** (not plagiarism)

## What this is
Independent **Cursor-native** contracts for all **11** public [LG-AI-EXAONE](https://github.com/orgs/LG-AI-EXAONE/repositories) repos,
from **public GitHub metadata + public README docs-read only**.

Ties to language **이도 (Ido)** / lab **금척 (Golden Ruler)** — see [`../NOTICE.md`](../NOTICE.md).

## What this is NOT (anti-plagiarism)
- **Not** a fork/clone of `github.com/LG-AI-EXAONE/*`
- **Not** EXAONE weights, training code, or proprietary scaffolds
- **Not** claiming LG / EXAONE identity or endorsement
- **Not** the abandoned name “이도원” (too close in sound to EXAONE)

## Method
1. List public org repos via API (no `git clone`).
2. Fetch README only into `docs-read/raw/<slug>/`.
3. Author CONTRACT + GROK + SKILL stubs **here**.
4. VERIFY = `./smoke-all.sh` → `ok=11`.

Upstream README copyright remains upstream. Our stubs: Apache-2.0 (see repo `LICENSE`).
