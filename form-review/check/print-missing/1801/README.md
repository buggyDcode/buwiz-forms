# Form 1801 — Independent Check (`92b3e201` vs `5cab6647`)

## What these files show
- `data-excerpt.txt`: Saved draft JSON from SQLite `form_drafts` (`check-pm-p1.db`) for all three cases (`normal`, `hard-long`, `hard-empty`), proving the claimed values are held in the saved return.
- `before/` (`92b3e201`) and `after/` (`5cab6647`): Full saved PDF (`saved-*-1801-normal.pdf`), PDF page renders (`pdf/p1.png`, `pdf/p2.png`, 1364px wide), and `@media screen` print-preview renders (`preview/p1.png`, `preview/p2.png`) for the full return.
- `hard-long/{before,after}/` and `hard-empty/{before,after}/`: Same PDF and preview renders for the 12-digit amount / 60-char specify hard case and the empty-value case.
- `crops/`: Close-up before/after crops of each claimed box (`item5_tin`, `item16_taxable_estate`, `item24_total_payable`, `item37c_specify`, `sched5_specify`) across `normal`, `hard-long`, and `hard-empty`.
- `1801-check.mp4`: 7s recording opening the saved return, stepping through form pages, print preview, and saved PDF at `5cab6647`.
- `results.tsv`: Line-by-line verdict per box and case.

## What these files do not show
- A live non-`00000` branch code saved via `profile.save` (the agent profile API requires an existing branch record for non-head-office TINs); the unjoined status of `frm1801v2018:branchCode_1` / `p1c17` is verified directly from `html-frozen/1801-2018/writer-cells.json` lines 1205 and 1967.
- Behavior after merging with PR `#355` (the 12-digit comb overflow blank in `p1c56` comes from `main`'s `frozen_html.rs:2707` and is resolved by `#355`'s RULES 2 overflow engine).
