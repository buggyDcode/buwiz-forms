# Form 1800 — Independent Check (`92b3e201` vs `5cab6647`)

## What these files show
- `data-excerpt.txt`: Saved draft JSON from SQLite `form_drafts` (`check-pm-p1.db`) for all three cases (`normal`, `hard-long`, `hard-empty`), proving `TotalNetGifts` and Schedule A/B gifts are held in the saved return.
- `before/` (`92b3e201`) and `after/` (`5cab6647`): Full saved PDF (`saved-*-1800-normal.pdf`), PDF page renders (`pdf/p1.png`, `pdf/p2.png`, 1364px wide), and `@media screen` print-preview renders (`preview/p1.png`, `preview/p2.png`) for the full return.
- `hard-long/{before,after}/` and `hard-empty/{before,after}/`: Same PDF and preview renders for the 12-digit amount (`123,456,789,012.34` gift -> `123,456,539,012.34` net gift) and empty-gift return.
- `crops/`: Close-up before/after crops of `item14_total_net_gifts` across `normal`, `hard-long`, and `hard-empty`.
- `1800-check.mp4`: 7s recording opening the saved return, stepping through form pages, print preview, and saved PDF at `5cab6647`.
- `results.tsv`: Line-by-line verdict per box and case.

## What these files do not show
- Behavior after merging with PR `#355` (the 12-digit comb overflow blank in `p1c62` comes from `main`'s `frozen_html.rs:2707` and is resolved by `#355`'s RULES 2 overflow engine).
