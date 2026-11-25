# Independent Band-Flow Check — Form 1801 (Official Overflow Sample `1801-09162025`)

- **Branch under check:** `hexuria/buwiz-forms` `try/forms-band-flow` at `4321b79b` (`after`) vs `b9552e27` (`before`).
- **Source sample:** `crates/bir-core/tests/sample-payloads/1801-09162025.plain.xml` (per `try/rules-coverage-1801-1707a` `340e17f0` report: 21 rows in Schedule 1, 7 rows each in Schedules 1A, 2, 2A, 3, 4; paper capacity is 2 rows per schedule).
- **Build folders:** isolated `/home/ubuntu/BuwizNsh/target-bf-b9552e27` (`before`) and `/home/ubuntu/BuwizNsh/target-bf-4321b79b` (`after`), `RUSTC_WRAPPER=""`, `/home/ubuntu/.cargo/bin/cargo`; `~/BuwizNsh/target-nsh` untouched (`4.0K`).
- **Verdict:** **`pass`** — all 56 held rows across all 6 Part V schedules print once, in order, with exact arithmetic totals and clean continuation headers at both `b9552e27` and `4321b79b`.

## Rule (b) Summary (`results.tsv`)
- **Schedule 1 (Real Properties):** 21/21 printed once, in order (both upper and lower halves), total Exclusive `24,200,028.80` matches arithmetic.
- **Schedule 1A (Family Home):** 7/7 printed once, in order (rows 1–3 on Page 2 with `Continued on the next page`, rows 4–7 on Page 3 `Page 2 (continued)` with repeated `Schedule 1A – Details of Family Home (continued)` header, `BIR Form No. 1801`, TIN `000-123-453-00000`, taxpayer name), total Conjugal `2,027,013.50` matches arithmetic.
- **Schedule 2 (Shares of Stock):** 7/7 once, in order, Exclusive `302,700.55`.
- **Schedule 2A (Other Personal):** 7/7 once, in order, Exclusive `51,881.25` / Conjugal `25,000.13`.
- **Schedule 3 (Taxable Transfers):** 7/7 once, in order, Exclusive `16,750.00`.
- **Schedule 4 (Business Interest):** 7/7 once, in order, Exclusive `40,000.00` / Conjugal `13,500.00`.
- **Pages:** 3 pages total, 0 blank or header-only pages, Schedule 5 and footer legend preserved cleanly on Page 3.

## Files
- `data-excerpt.txt`, `results.tsv`, `summary.json`, `{before,after}-09162025.pdf`, `{before,after}-09162025-p{1,2,3}.png`, `crop-p2-sched1-{before,after}.png`, `crop-p3-cont-{before,after}.png`, `recording.mp4`.
- What these files do NOT show: live BIR submission or Queue (never run).
