# Form 1707A — Independent Check (`92b3e201` vs `d1777a31`)

- **Verdict:** `tweak`
- **Build provenance:** `before` (`92b3e201`) built in `/home/ubuntu/BuwizNsh/target-92b3e201`; `after` (`d1777a31`) built in `/home/ubuntu/BuwizNsh/target-d1777a31` (separate `CARGO_TARGET_DIR` per commit). Binary identity verified directly by `p1c207` blank on `92b3e201` and filled on `d1777a31`.

## Files

- `data-excerpt.txt`: SQLite `form_drafts` JSON payloads proving `tax_relief` holds the 10-, 25-, 26-, and 60-character strings, empty string `""`, and `null` (No).
- `<case>/{before,after}/saved-*.pdf`: Saved PDFs printed from the running app at `92b3e201` (`before`) and `d1777a31` (`after`) for `len10`, `len25`, `len26`, `len60`, `empty`, and `relief-no`.
- `<case>/{before,after}/pdf/p{1,2}.png` and `<case>/{before,after}/preview/p{1,2}.png`: Full-page saved-PDF and print-preview renders (~1400 px wide) for both pages.
- `crops/item12a_<case>_{before,after}.png`: Close crops of official Items 12 and 12A (`p1c42`, `p1c43`, `p1c207`) at both commits.
- `1707A-check.mp4`: Recording (< 2 min) showing the saved return in the app, preview, and saved PDF.
- `results.tsv`: Box-by-box and case-by-case table.

## What this does and does not show

- **Shows:** `frm1707Av2021:txtI11ASpecify` (`p1c207`, 25-slot comb) is blank on `92b3e201` and prints cleanly in comb slots for 10 and 25 chars at `d1777a31`; empty and tax-relief-No cases stay blank; 26- and 60-char values print whole at `9.00pt` left-aligned inside `p1c207` (no truncation, no additional sheet) but retain the 25 SVG comb divider ticks behind the text (RULES 1, pre-existing `main` engine behavior fixed on `#355`); 0 unintended boxes changed across pages 1–2; preview matches saved PDF.
- **Does not show:** Filing validation (validation caps `tax_relief` at 20 chars; 25/26/60-char drafts were saved and printed directly to test print overflow behavior) or `#355` merged onto `d1777a31`.
