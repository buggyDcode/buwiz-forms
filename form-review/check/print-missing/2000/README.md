# Form 2000 — Independent Check of Item 15A (`92b3e201` vs `d1777a31`)

- **Verdict:** `clean` (author is right: Item 15A already prints on `main` `92b3e201`).
- **Build provenance:** `before` (`92b3e201`) built in `/home/ubuntu/BuwizNsh/target-92b3e201`; `after` (`d1777a31`) built in `/home/ubuntu/BuwizNsh/target-d1777a31`.

## Files

- `data-excerpt.txt`: SQLite `form_drafts` JSON payload proving `is_amended: true` and `balance_carried_over: 1119.17`.
- `{before,after}/saved-*-2000-normal.pdf`: Saved PDFs at `92b3e201` and `d1777a31`.
- `{before,after}/pdf/p{1,2}.png` and `{before,after}/preview/p{1,2}.png`: Full-page saved-PDF and print-preview renders (~1400 px wide).
- `crops/item15a_normal_{before,after}.png`: Close crops of Items 14–16 (`p1c50` Item 15A, `p1c56` Item 15D) at both commits.
- `2000-check.mp4`: Recording (< 2 min) showing the saved return, preview, and saved PDF.
- `results.tsv`: Cell-level verification row.

## What this does and does not show

- **Shows:** On `main` `92b3e201`, amended Form 2000 holding `balance_carried_over = 1119.17` prints `1 1 1 9 . 1 7` (peso and centavo digits) in official cell `p1c50` (Item 15A) and `p1c56` (Item 15D), byte-identical at `d1777a31`; preview matches saved PDF.
- **Does not show:** Overflow (>12 peso digits) or unamended Item 15A behavior.
