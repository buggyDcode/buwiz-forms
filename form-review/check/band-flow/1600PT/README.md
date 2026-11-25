# Independent Band-Flow Check — Form 1600PT (Fault 2)

- **Branch under check:** `hexuria/buwiz-forms` `try/forms-band-flow` at `4321b79b` (`after`) vs `b9552e27` (`before`).
- **Build folders:** isolated `/home/ubuntu/BuwizNsh/target-bf-b9552e27` (`before`) and `/home/ubuntu/BuwizNsh/target-bf-4321b79b` (`after`), `RUSTC_WRAPPER=""`, `/home/ubuntu/.cargo/bin/cargo`; `~/BuwizNsh/target-nsh` untouched (`4.0K`).
- **App environment:** running NSH Linux apps (`nsh-bf-before` / `nsh-bf-after`) with fresh scratch SQLite databases, demo taxpayer `SANTOS, JUANA DELA CRUZ` (`TIN 000-123-453-00000`, `RDO 039`), real official `FORM_1600PT_ATC_OPTIONS` with distinct `tax_base` per row, `BUWIZ_NO_KEYCHAIN=1 EBIR_TEST_ENV=1 BIR_AUTH_BYPASS=1 BIR_SFTP_DRY_RUN=1`, `BIR_SFTP_LIVE` unset, never Queue.
- **Verdict:** **`tweak`** — both reported parts of Fault 2 are fixed (rows print once in order with matching totals; 1-digit rates are right-aligned away from centavos), **but** `Page 1 (continued)` omits the taxpayer TIN/name on every continuation page, and 4-char item labels `17aa`–`17ai` touch the first ATC character at 40 rows.

## Cases & Rule (b) Summary (`results.tsv`)
- `cap5` (5 rows, exact paper capacity): 5/5 printed once, in order, Item 18 = `1,343.29` matches arithmetic, 2 pages at both commits. Single-digit rates (`3`, `2`) sit beside centavos at `b9552e27` and cleanly in the right slot next to `%` at `4321b79b`.
- `cap8` (8 rows, capacity + 3):
  - **Before (`b9552e27`):** **FAIL (Rank 1)** — 5/8 printed (rows 6–8 dropped) while Item 18 = `1,882.14` sums all 8.
  - **After (`4321b79b`):** 8/8 printed once, in order on Page 1; Item 18 = `1,882.14` matches arithmetic; single-digit rates right-aligned (`crop-cap8-rate1digit-after.png`); preview matches saved PDF. **Remaining fault:** Machine Validation / Stamp box spills to Page 2 (`Page 1 (continued)`), which prints `BIR Form No. 1600-PT / Page 1 (continued)` with **no TIN and no taxpayer name**.
- `cap31` (31 rows, max UI rows):
  - **Before (`b9552e27`):** **FAIL (Rank 1)** — 5/31 printed while Item 18 = `20,653.65` sums all 31.
  - **After (`4321b79b`):** 31/31 printed once, in order; Item 18 = `20,653.65` matches arithmetic; Items 19–27, signature, Part III, and stamp spill to Page 2 (`Page 1 (continued)`), which **omits TIN and taxpayer name**.
- `cap40` (40 rows, multi-page Part II continuation):
  - **Before (`b9552e27`):** **FAIL (Rank 1)** — 5/40 printed while Item 18 = `23,588.44` sums all 40.
  - **After (`4321b79b`):** 40/40 printed once, in order (rows 1–31 on Page 1, rows 32–40 on Page 2 `Page 1 (continued)` with repeated `Part II – Computation of Tax (continued)` + `ATC | Tax Base | Tax Rate | Tax Withheld` headings); Item 18 = `23,588.44` matches arithmetic; 0 blank pages; preview matches saved PDF. **Remaining faults:**
    1. **Rank 5 (headerless/incomplete continuation header):** Page 2 (`Page 1 (continued)`) names `1600-PT` and `Page 1 (continued)` and repeats the table head, **but omits TIN and taxpayer name** (`crop-cap40-p2-cont-after.png`).
    2. **Rank 6 (cosmetic):** Row numbers `17aa`–`17ai` touch the first letter `W` of the ATC code (`17aaW B 0 3 0`).

## Files
- `data-excerpt.txt`: held Part II ATC rows from the saved scratch draft for all 4 cases.
- `results.tsv`: one line per commit/case with Rule (b) counts extracted from the saved PDFs.
- `{before,after}-cap{5,8,31,40}.pdf` and `{before,after}-cap{5,8,31,40}-p*.png`: saved PDFs and rendered page images for every page at both commits.
- `crop-cap8-p1-part2-{before,after}.png`: Part II rows on Page 1 (5 rows before vs 8 rows after).
- `crop-cap8-rate1digit-{before,after}.png`: close crop of single-digit tax rates beside centavos (left-slot before vs right-slot next to `%` after).
- `crop-cap40-p2-cont-{before,after}.png`: close crop of Page 2 continuation header (showing repeated table head, missing TIN/name, and `17aa` touching `W`).
- `recording.mp4`: 10-second screen recording of 1600PT opened and printed in the running `4321b79b` app.
- What these files do NOT show: live BIR submission or Queue (never run).
