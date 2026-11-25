# fixes-3 item 3 — 1600PT continuation pages carry TIN and name

- Fault (from the independent check): `Page 1 (continued)` carried the form banner and the repeated table head but neither the filer's TIN nor the taxpayer name.
- Root cause: `start_new_page` in `crates/bir-print/src/band_flow.rs` seeded each spawned continuation page with only the page_header band; on 1600-PT the TIN comb cells (p1c16/18/20/22) and the name live in plain_block bands below the shallow header band, so they never repeated.
- Fix: continuation pages now also repeat the filer-identity strip — every static PlainBlock/SectionTitle band between the page header and the page's first growable table (TIN with branch code, name, return period, ATC lines) — the same block the paper's own additional sheets ask for. `identity_strip_band_indices` is shared between the packer and the conservation test.
- Failing-first test: `form_1600pt_continuation_pages_carry_tin_and_name` in `crates/bir-print/tests/band_flow_fixes3.rs` (40 Part II rows). Without the fix: `TIN comb cell p1c16 prints 1 time(s) across 3 pages` — FAIL. With the fix: PASS.
- Proof here: 40-row government draft through `filled_document` + the app's real print pipeline (`prepare_html_for` + `staticize`), printed headless:
  - `before-pipe.pdf` — 3 pages; page 2 (`before-p2-2.png`) is `Page 1 (continued)` with no TIN comb and no name.
  - `after-pipe.pdf` — 4 pages; page 2 (`after-p2-2.png`) carries the TIN comb 000-123-453-00000 and `SANTOS, JUANA` above the repeated table head. The strip takes vertical room, so rows flow onto one more continuation page — the same trade the paper's own additional sheets make.
