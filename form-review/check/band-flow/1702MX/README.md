# Independent Band-Flow Check — Form 1702MX

- **Branch under check:** `hexuria/buwiz-forms` `try/forms-band-flow` at `4321b79b` (`after`) vs `b9552e27` (`before`).
- **Build folders:** isolated `/home/ubuntu/BuwizNsh/target-bf-b9552e27` and `/home/ubuntu/BuwizNsh/target-bf-4321b79b`; `~/BuwizNsh/target-nsh` untouched (`4.0K`).
- **Verdict:** **`fail`** (after: 4 pages, blank_pages=[], old_fallback_pages=[3, 4]).

## Results (`results.tsv`)
- **Schedule 6 (3 base + 7 modal):** before=0/10 (missing 10) → after=0/10 (missing 10, in_order=False, tot=123,000.00 in_pdf=False).

## Files
- `data-excerpt.txt`, `results.tsv`, `summary.json`, `{before,after}-overflow.pdf`, `{before,after}-overflow-p*.png`, `recording.mp4`.
- What these files do NOT show: live BIR submission or Queue (never run).
