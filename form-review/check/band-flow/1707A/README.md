# Independent Band-Flow Check — Form 1707A (Official Overflow Sample `1707A-12312023` + Fault 6)

- **Branch under check:** `hexuria/buwiz-forms` `try/forms-band-flow` at `4321b79b` (`after`) vs `b9552e27` (`before`).
- **Source sample:** `crates/bir-core/tests/sample-payloads/1707A-12312023.plain.xml` (Schedule 1: 14 rows = 3 base + 11 modal; Schedule 2: 12 rows = 3 base + 9 modal; paper capacity is 4 rows with Row 4 = `OTHERS` subtotal).
- **Build folders:** isolated `/home/ubuntu/BuwizNsh/target-bf-b9552e27` (`before`) and `/home/ubuntu/BuwizNsh/target-bf-4321b79b` (`after`), `RUSTC_WRAPPER=""`, `/home/ubuntu/.cargo/bin/cargo`; `~/BuwizNsh/target-nsh` untouched (`4.0K`).
- **Verdict:** **`fail`** (Fault 6 NOT fixed on `1707A` — **two blank pages (Pages 4 and 6 of 6)** and old-style `ADDITIONAL SHEET` fallback remain at `4321b79b`, identical to `b9552e27`).

## Rule (b) & Fault 6 Summary (`results.tsv`)
- **Schedule 1 (Gains):** 14/14 rows printed once, in order (rows 1–3 on Page 2, Row 4 `OTHERS` subtotal on Page 2, 11 modal rows on Page 3 `ADDITIONAL SHEET`); total Capital Gains `716,619.65` matches arithmetic.
- **Schedule 2 (Losses):** 12/12 rows printed once, in order (rows 1–3 on Page 2, Row 4 `OTHERS` subtotal on Page 2, 9 modal rows on Page 5 `ADDITIONAL SHEET`); total Capital Loss `49,269.80` matches arithmetic.
- **Remaining Faults at `4321b79b` (unchanged from `b9552e27`):**
  1. **Rank 5 — Two completely blank pages (`after-12312023-p4.png` and `after-12312023-p6.png`):** 6 pages total, Pages 4 and 6 are pure white blank pages after each `ADDITIONAL SHEET`.
  2. **Rank 5 — Old-style `ADDITIONAL SHEET` fallback still used** instead of band-flow continuation: no page number on Pages 3 and 5, `Return Period: —`.
  3. **Rank 6 (cosmetic) — Row numbering skips #4:** Page 2 prints rows 1–3 + `OTHERS`, and Page 3/5 `ADDITIONAL SHEET` numbers the 11/9 modal rows `#5..#15` and `#5..#13` (`continuing after 4 printed rows`) instead of `#4..#14` and `#4..#12`.

## Files
- `data-excerpt.txt`, `results.tsv`, `summary.json`, `{before,after}-12312023.pdf`, `{before,after}-12312023-p{1..6}.png`, `crop-p2-scheds-{before,after}.png`, `crop-p3-cont-{before,after}.png`, `recording.mp4`.
- What these files do NOT show: live BIR submission or Queue (never run).
