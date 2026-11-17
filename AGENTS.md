# Assignment 4 standing context

Before planning, editing, or evaluating A4 work, read these repository files:

1. `docs/assignment/a4-rule-book.md` — the complete text of Section 12, rules R1–R34, from the November 7 A4 handout.
2. `docs/assignment/a4-hardware-handout.md` — searchable text of all five pages of the November 7 hardware and auctions handout.
3. `rules.json` — the keyed numbers for the rules. Its current values are provisional until the game opens; check for a newer published version when making a time-sensitive decision.
4. `docs/assignment/programmable-agreements-agent-brief.md` — the Round 3 agreement-file workflow and signing checks from the November 14 agreements pack. For agreement-file behavior, use this brief and the updated SDK/source in preference to the older settlement-program description in Section 4 of `docs/a4-api-guide.md`.
5. `docs/assignment/Website-read-API.md` — the official read-only website API guide. Read this before using the API. Never use it to act in the game; a person makes every game-site click. Read the API key only from the `NA4_KEY` environment variable, never from chat or repository files, and never print it.

The optional pit-wall and per-world coach workspace is in `harness/`. When working there, read `harness/AGENTS.md` and the relevant `harness/pitwall/AGENTS.md` or `harness/WN/AGENTS.md`. The harness's `tools/na4.py` is for read-only API requests; it takes its key only from `NA4_KEY`.

The original PDFs are archived at `docs/assignment/a4-handout-2026-11-07.pdf` and `docs/assignment/a4-hardware-handout-2026-11-07.pdf`. Consult them when a formula, table, figure, or auction example depends on layout. If the extracted text disagrees with a PDF, use the PDF. The A4 program interfaces are in `docs/a4-api-guide.md`.

`docs/LIMITS.md` includes older A1 limits. For A4 resource limits, use `rules.json` and `docs/a4-api-guide.md`.
