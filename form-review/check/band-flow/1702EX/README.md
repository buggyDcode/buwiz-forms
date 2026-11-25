# Independent Band-Flow Check — Form 1702EX

- **Branch under check:** `hexuria/buwiz-forms` `try/forms-band-flow` at `4321b79b` (`after`) vs `b9552e27` (`before`).
- **Build folders:** isolated `/home/ubuntu/BuwizNsh/target-bf-b9552e27` and `/home/ubuntu/BuwizNsh/target-bf-4321b79b`; `~/BuwizNsh/target-nsh` untouched (`4.0K`).
- **Verdict:** **`fail`** (after: 3 pages, blank_pages=[], old_fallback_pages=[3]).

## Results (`results.tsv`)
- **Sched 1 Item 17 (Other Ded):** before=0/8 (missing 8) → after=0/8 (missing 8, in_order=False, tot=11,996.00 in_pdf=False).
- **Sched 2 Item 4 (Special):** before=0/8 (missing 8) → after=0/8 (missing 8, in_order=False, tot=19,996.00 in_pdf=False).
- **Sched 3 Item 3 (Non-Ded):** before=0/8 (missing 8) → after=0/8 (missing 8, in_order=False, tot=27,996.00 in_pdf=False).
- **Sched 3 Item 6 (Non-Tax Inc):** before=0/8 (missing 8) → after=0/8 (missing 8, in_order=False, tot=35,996.00 in_pdf=False).
- **Sched 3 Item 8 (Special Ded):** before=0/8 (missing 8) → after=0/8 (missing 8, in_order=False, tot=43,996.00 in_pdf=False).

## Files
- `data-excerpt.txt`, `results.tsv`, `summary.json`, `{before,after}-overflow.pdf`, `{before,after}-overflow-p*.png`, `recording.mp4`.
- What these files do NOT show: live BIR submission or Queue (never run).
