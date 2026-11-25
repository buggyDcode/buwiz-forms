# Form 1604E — Independent Check (`92b3e201` vs `5cab6647`)

## What these files show
- `data-excerpt.txt`: Saved draft JSON from SQLite `form_drafts` (`check-pm-p2.db`) and reopened `form.fields` for `normal` (2025), `year-2030`, `year-2009`, `year-1999`, and `hard-empty` (UI refuses clearing `txtYear`).
- `before/` (`92b3e201`) and `after/` (`5cab6647`): Saved PDFs, 2-page PDF renders (1364px wide), and `@media screen` preview renders for `2025`.
- `year-2030/`, `year-2009/`, `year-1999/`, `hard-empty/`: Same before/after renders.
- `crops/`: Close-up before/after crops of Item 1 (`p1c4`).
- `1604E-check.mp4`: 7s recording opening the saved return, print preview, and saved PDF at `5cab6647`.
- `results.tsv`: Per-box and per-case verdicts.

## What these files do not show
- Fiscal year (1604E is calendar-year only and has no fiscal-year control).
- Filing/validation error UI (only draft save and print preview/PDF were exercised; Queue was never pressed).
