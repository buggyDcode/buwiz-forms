# Independent Band-Flow Check — Form 2200M

- **Branch under check:** `hexuria/buwiz-forms` `try/forms-band-flow` at `4321b79b` (`after`) vs `b9552e27` (`before`).
- **Build folders:** isolated `/home/ubuntu/BuwizNsh/target-bf-b9552e27` and `/home/ubuntu/BuwizNsh/target-bf-4321b79b`; `~/BuwizNsh/target-nsh` untouched (`4.0K`).
- **Verdict:** **`fail`** (after: 4 pages, blank_pages=[4], old_fallback_pages=[]).

## Results (`results.tsv`)
- **Schedule 1 (Minerals 16 rows):** before=0/16 (missing 8) → after=0/16 (missing 8, in_order=False, tot=31,130.00 in_pdf=True).

## Files
- `data-excerpt.txt`, `results.tsv`, `summary.json`, `{before,after}-overflow.pdf`, `{before,after}-overflow-p*.png`, `recording.mp4`.
- What these files do NOT show: live BIR submission or Queue (never run).
