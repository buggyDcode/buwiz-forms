# Independent Band-Flow Check — Form 2550Q (Fault 1)

- **Branch under check:** `hexuria/buwiz-forms` `try/forms-band-flow` at `4321b79b` (`after`) vs `b9552e27` (`before`).
- **Build folders:** isolated `/home/ubuntu/BuwizNsh/target-bf-b9552e27` (`before`) and `/home/ubuntu/BuwizNsh/target-bf-4321b79b` (`after`), `RUSTC_WRAPPER=""`, `/home/ubuntu/.cargo/bin/cargo`; `~/BuwizNsh/target-nsh` untouched (`4.0K`).
- **App environment:** running NSH Linux apps (`nsh-bf-before` / `nsh-bf-after`) with fresh scratch SQLite databases, demo taxpayer `SANTOS, JUANA DELA CRUZ` (`TIN 000-123-453-00000`, `RDO 039`), invented distinct rows, `BUWIZ_NO_KEYCHAIN=1 EBIR_TEST_ENV=1 BIR_AUTH_BYPASS=1 BIR_SFTP_DRY_RUN=1`, `BIR_SFTP_LIVE` unset, never Queue.
- **Verdict:** **`pass`** — Fault 1 is fixed.

## Cases & Rule (b) Summary (`results.tsv`)
- `cap2` (2 rows, exact paper capacity): 2/2 printed once, in order, totals match arithmetic, 2 pages at both commits.
- `cap5` (5 rows, capacity + 3): 5/5 printed once, in order, totals match arithmetic, 2 pages at both commits.
- `cap12` (12 rows, exact Fault 1 trigger):
  - **Before (`b9552e27`):** **FAIL (Rank 1)** — only 10/12 printed (`CAPGOOD-11` and `CAPGOOD-12` missing from paper) while printed total (`285,500.66` Col E, `271,225.63` Col I) includes all 12 rows.
  - **After (`4321b79b`):** **PASS** — all 12/12 rows (`CAPGOOD-01`..`CAPGOOD-12`) printed once, in order on Page 2; totals match arithmetic (`285,500.66` / `271,225.63`); Schedule 4 flows cleanly to Page 3 (`Page 2 (continued)`) carrying form `2550Q`, TIN `000-123-453-00000`, taxpayer name, and Schedule 4 heading; 0 blank or header-only pages; Page 1 text identical to `b9552e27`; preview matches saved PDF.
- `cap40` (40 rows, multi-page stress test):
  - **Before (`b9552e27`):** **FAIL (Rank 1)** — only 10/40 printed (`CAPGOOD-11`..`CAPGOOD-40` missing) while total includes all 40.
  - **After (`4321b79b`):** **PASS** — all 40/40 printed once, in order (rows 1–29 on Page 2, rows 30–40 on Page 3 `Page 2 (continued)` with repeated `Schedule 1 – Amortized Input Tax from Capital Goods (continued)` heading and columns `(A)`–`(I)`, form `2550Q`, TIN `000-123-453-00000`, and exact totals `1,864,186.39` / `1,770,977.07`); 0 blank pages; Page 1 identical; preview matches saved PDF.
- **Pre-existing minor cosmetic notes (present at both `b9552e27` and `4321b79b`):** rows 1–2 format Estimated/Recognized Life as `60.00` while rows 3+ format as `60`; empty default Schedule 4 rows show `0.00` in Col (D).

## Files
- `data-excerpt.txt`: held Schedule 1 rows from the saved scratch draft for all 4 cases.
- `results.tsv`: one line per commit/case with Rule (b) counts extracted from the saved PDFs.
- `{before,after}-cap{2,5,12,40}.pdf` and `{before,after}-cap{2,5,12,40}-p*.png`: saved PDFs and rendered page images for every page at both commits.
- `crop-cap12-p2-sched1-{before,after}.png` & `crop-cap12-p3-cont-after.png`: close crops of Schedule 1 on Page 2 (10 rows before vs 12 rows after) and Page 3 continuation header.
- `recording.mp4`: 10-second screen recording of 2550Q opened and printed in the running `4321b79b` app.
- What these files do NOT show: live BIR submission or Queue (never run).
