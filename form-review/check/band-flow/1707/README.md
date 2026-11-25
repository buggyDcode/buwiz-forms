# Independent Band-Flow Check — Form 1707

- **Branch under check:** `hexuria/buwiz-forms` `try/forms-band-flow` at `4321b79b` (`after`) vs `b9552e27` (`before`).
- **Build folders:** isolated `/home/ubuntu/BuwizNsh/target-bf-b9552e27` and `/home/ubuntu/BuwizNsh/target-bf-4321b79b`; `~/BuwizNsh/target-nsh` untouched (`4.0K`).
- **Verdict:** **`fail`** (after: 5 pages, blank_pages=[5], old_fallback_pages=[1, 2]).

## Results (`results.tsv`)
- **Schedule 2 (Shares):** before=9/12 (missing 3) → after=9/12 (missing 3, in_order=False, tot=206,666.58 in_pdf=True).
- **Schedule 3 (Expenses):** before=0/12 (missing 12) → after=0/12 (missing 12, in_order=False, tot=15,629.10 in_pdf=False).

## Files
- `data-excerpt.txt`, `results.tsv`, `summary.json`, `{before,after}-overflow.pdf`, `{before,after}-overflow-p*.png`, `recording.mp4`.
- What these files do NOT show: live BIR submission or Queue (never run).
