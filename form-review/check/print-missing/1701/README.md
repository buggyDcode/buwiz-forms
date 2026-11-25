# Form 1701 — Independent Check (`92b3e201` vs `5cab6647`)

## What these files show
- `data-excerpt.txt`: Saved draft JSON from SQLite `form_drafts` (`check-pm-p2.db`) and reopened `form.fields` for `normal`, `hard-long` (72-char employer names + 14-digit TINs with non-zero branch codes), and `hard-empty`.
- `before/` (`92b3e201`) and `after/` (`5cab6647`): Saved PDFs, 4-page PDF renders (1364px wide), and `@media screen` preview renders.
- `hard-long/{before,after}/` and `hard-empty/{before,after}/`: Same renders for the 72-char employer name / non-zero branch TIN case and the empty Schedule 1 case.
- `crops/`: Close-up before/after crops of Schedule 1 (`p2c55`, `p2c56`, `p2c58`, `p2c60`, `p2c61`, `p2c63`).
- `1701-check.mp4`: 13s recording opening the saved return, print preview, and saved PDF at `5cab6647`.
- `results.tsv`: Per-box and per-case verdicts.

## What these files do not show
- More than 2 employers (Schedule 1 has 2 rows on the official sheet; additional employers belong on an additional sheet).
