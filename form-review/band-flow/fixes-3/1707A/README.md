# fixes-3 — 1707A blank pages and continuation header

- before-pipe.pdf: the real pipeline (staticize + prepare_html_for) at feac5d2f — 6 physical pages, pages 4 and 6 pure white (the fixed-height continuation page box fragments inside the height:auto .sheet-flow print wrapper and spills its empty bottom slice).
- after-pipe.pdf: same print at the fix — 4 physical pages, no blanks; the continuation pages size to their content. Page 3/4 carry the form number, the filer TIN with branch code (000-123-453-00000), the taxpayer name, the return period (12/31/2023, previously the unknown dash), a physical page number (Page 3 / Page 4) and the schedule table heads.
- Tests: crates/bir-print/tests/band_flow_blank_pages.rs and band_flow_fixes3.rs.
