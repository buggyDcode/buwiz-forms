# Sweep-2 Band-Flow Proofs (Items A–J)

All proofs were captured from real saved returns in the running Linux GPUI desktop app (`nsh-run-agent`, scratch DB, dry-run/test env only; demo taxpayer `SANTOS, JUANA`, TIN `000-123-453-00000`). Every `.mp4` screen recording was captured from the app's X display (`1400x900`) via `ffmpeg -f x11grab`, showing the filer opening the saved return, saving/confirming rows past printed capacity, opening the print preview, and paging through all sheets. All recordings are 12–18 seconds.

## Per-Item File Index

### Item A — Repeated section titles & gapped table heads (`item-a-1800/`)
- `before-1.png`: Sweep-2 baseline (`origin/try/forms-band-flow-sweep` `ddff7ea6`, `1800/cap_p40-p-3.png`).
- `preview-page-{1..3}.png`, `pdf-page-{1..3}.png`: Real saved `1800` return with 16 Schedule B real-property rows (every column filled, FMVs `124,444,444.44`–`155,555,555.52`, verified `TOTAL = 2,222,222,221.92`). Shows Schedule B rows 11–21 continuing without duplicating 6–8 and breaking to `Page 2 (continued)`.
- `recording.mp4` (16.9s): Opening saved `1800` return, saving, opening print preview, and paging through Pages 1–3.
- **Does not show**: `1602Q` Schedule 4 static-prefix repetition (covered by `sweep2_a_repeated_section_title_and_gapped_table_head_on_breaks` in `crates/bir-print/src/band_flow.rs`); `1800` Schedule B Continuation's `p2b25` header band also holds row 1 in the frozen layout metadata (`repeat_on_break: false`), so row 1 is not duplicated on Page 3.

### Item B & M — Safe `(continued)` labels, `2550M` `Page 2 (continued)` & Schedule 5 right-alignment (`item-b-2550m/`)
- `before-1.png`: Sweep-2 baseline (`2550M/cap_p40-p-3.png`, showing `(continued)` colliding with column captions and missing `Page 2 (continued)`).
- `preview-page-{1..3}.png`, `pdf-page-{1..3}.png`: Real saved `2550M` return with 60 Schedule 3A capital-goods rows (every column filled, amounts `110,123,456.78`+, verified deferred total `805,459,257.07`). Page 3 shows unclipped `BIR Form 2550M - February 2007 (ENCS)    Page 2 (continued)`, zero header collisions, and Schedule 5 ratable-portion `0.00` right-aligned like Schedule 4 (Item M).
- `recording.mp4` (14.3s): Opening saved `2550M` return, saving, opening print preview, and paging through Pages 1–3.
- **Does not show**: `2551Q`, `2200C`, `2000` (all share the same engine path verified by `sweep2_b_continued_never_collides_with_column_headers_and_2550m_page2_label`).

### Item C — Natural row ordering (`item-c-2000ot/`)
- `before-1.png`: Sweep-2 baseline (`2000OT/cap_p3-p-2.png`, showing `1, 2, 3, 4, 6, 7, 8, 5` misordering).
- `preview-page-{1,2}.png`, `pdf-page-{1,2}.png`: Real saved `2000-OT` return with 8 Schedule 1.A property rows (every column filled, FMVs `119,444,444.97`–`150,555,554.51`, verified `TOTAL = 1,079,999,997.92`). Both upper and lower Schedule 1.A halves print strictly `1, 2, 3, 4, 5, 6, 7, 8`.
- `recording.mp4` (14.8s): Opening saved `2000-OT` return, saving, opening print preview, and paging through Pages 1–2.
- **Does not show**: `2552` Schedules 1–3 and Part VI (verified by `sweep2_c_rows_preserve_natural_order_on_2000ot_and_2552`).

### Item D — Right-aligned amounts (`item-d-1801/`)
- `before-1.png`: Sweep-2 baseline (`1801/cap_p3-p-2.png`, showing left-aligned Exclusive/Conjugal extra-row amounts).
- `preview-page-{1..3}.png`, `pdf-page-{1..3}.png`: Real saved `1801` return with 6 Schedule 1 real-property rows (every column filled; verified `EXCLUSIVE TOTAL = 834,074,292.42`, `CONJUGAL/COMMUNAL TOTAL = 1,397,407,418.64`, carried to Part IV Item 29). All extra-row amounts are right-aligned with thousands commas and two decimals.
- `recording.mp4` (17.6s): Opening saved `1801` return, saving, opening print preview, and paging through Pages 1–3.
- **Does not show**: `1601-FQ` Amount of Tax Withheld (G) (verified by `sweep2_d_amounts_right_aligned_on_1801_and_1601fq`).

### Item E — Extra-row numbering without collisions (`item-e-1702rt/`)
- `before-1.png`: Sweep-2 baseline (`1702RT/cap_p3-p-3.png`, showing `5, 6, 7` before `5 Total`).
- `preview-page-{1..4}.png`, `pdf-page-{1..4}.png`: Real saved `1702-RT` return with 3 official Schedule II rows + 4 modal rows (every column filled in whole pesos `103,222,222`–`116,555,554`, verified Item 5 `Total = 769,222,216`). Extra rows number `4a, 4b, 4c, 4d` above `5 Total`.
- `recording.mp4` (16.7s): Opening saved `1702-RT` return, saving, opening print preview, and paging through Pages 1–4.
- **Does not show**: `1602Q` (covered by `sweep2_e_extra_row_numbering_on_1601eq_1602q_1702rt_and_1800`; `1601EQ` `18a..18c` is shown in `item-h-1601eq-plus3/`, `1800` `11..21` in `item-a-1800/`).

### Item F — Break-page outer side and bottom borders at +40 rows (`item-f-2000ot-plus40/`)
- `before-1.png`: Sweep-2 baseline (`2000OT/cap_p40-p-3.png`, showing Schedule 1.B row 1 losing outer side and bottom borders).
- `preview-page-{1..4}.png`, `pdf-page-{1..4}.png`: Real saved `2000-OT` return with 45 Schedule 1.A rows (`TOTAL = 5,985,000,060.75`) plus Schedule 1.B row 1 (`SANTOS HOLDINGS CORP`, `125,000,000.50` par value, `1,250,000.05` DST). On `Page 2 (continued)` (`pdf-page-3.png`), Schedule 1.B row 1 retains crisp left, right, and bottom borders.
- `recording.mp4` (15.5s): Opening saved `2000-OT` +40 return, saving, opening print preview, and paging through Pages 1–4.
- **Does not show**: Other forms at +40 rows (covered by the 43-form conservation suite).

### Item G — Suffix letter spacing clear of vertical divider (`item-g-1701a/`)
- `before-1.png`: Sweep-2 baseline (`1701A/cap_p3-p-2.png`, showing `42a..` touching the divider).
- `preview-page-{1..3}.png`, `pdf-page-{1..3}.png`: Real saved `1701A` return (graduated rate, OSD) with Item 41 (`111,222,333`) + 5 modal non-operating income rows (`42a..42e`, verified Item 44 `Total Other Income = 796,407,518`). Suffix labels `42a..42e` render at `6.2pt` cleanly clear of the divider above Item 43.
- `recording.mp4` (13.7s): Opening saved `1701A` return, saving, opening print preview, and paging through Pages 1–3.
- **Does not show**: `1601C` (uses the same suffix-label sizing rule tested in `sweep2_g_suffix_labels_nudged_clear_of_divider_on_1701a_and_1601c`).

### Items H, J, K & N — `1601-EQ` at +3 rows: closing border, unshrunk Machine Validation, `8pt` matching labels, and form-page flow recording (`item-h-1601eq-plus3/`)
- `before-1.png`, `before-2.png`: Sweep-2 baseline (`1601EQ/cap_p3-p-1.png` and `cap_p3-p-2.png`, showing missing bottom border on page 1, shrunk Machine Validation box on page 2, and blank amount/rate columns).
- `preview-page-{1..3}.png`, `pdf-page-{1..3}.png`: Real saved `1601-EQ` return with 9 ATC rows (`13..18` + `18a..18c`). Grown-row labels `18a`, `18b`, `18c` render in the table's own bold `"eBIRForms Arimo"` style at `8pt` (`>= 7.5pt`, RULES 3, Item K). Every column (ATC, Tax Base `134,567,900.23`–`223,456,789.11`, Tax Rate `1%`–`15%`, Tax Withheld) prints on all 9 rows, verified Item 19 `Total Taxes Withheld = 98,481,480.90` and Item 31 `129,918,743.02`. Page 1 keeps its closing bottom border under Item 35; `Page 1 (continued)` renders the Machine Validation box at its full official `54.48pt` height (3 extra rows need `52.20pt` > `22.68pt` elastic slack + `1.00pt` free space, so the page breaks and the moved box is not shrunk).
- `recording.mp4` (redone for Item N): Live frames of the running eBIRForms GPUI app window open on the **1601-EQ form page** (Part II Computation of Tax rows `13`–`18` and Additional ATC rows `7`–`9` added past printed capacity, `✓ Saved`), followed by all 3 print sheets produced by clicking **Print preview** (`fp-print`). Note: the app's native WebKit print preview window cannot be grabbed directly via `x11grab` on this headless Linux X11 desktop, so the recording shows the live form-page portion directly from the GPUI app and the sheet portion from the exact `sheet.html` emitted by `fp-print`.
- **Does not show**: Native WebKit print dialog chrome or live submission to BIR (never performed; dry-run/test env only).

### Item I — Thousands separators & two decimals in plain grown rows (`item-i-2550q/`)
- `before-1.png`: Sweep-2 baseline (`2550Q/cap_p3-p-3.png`, showing `12071.00` without commas).
- `preview-page-{1,2}.png`, `pdf-page-{1,2}.png`: Real saved `2550Q` return with 6 Schedule 1 capital-goods rows (every column `(A)`–`(I)` filled, amounts `137,345,678.47`–`199,074,069.07`, verified Col E `Total = 121,111,109.12` and Col I `Total = 119,092,590.63` matching Items 39 and 52). Every plain grown-row amount prints right-aligned with thousands commas and two decimals.
- `recording.mp4` (12.5s): Opening saved `2550Q` return, saving, opening print preview, and paging through Pages 1–2.
- **Does not show**: Multi-page Schedule 1 break on `2550Q` (covered by `sweep2_i_grown_row_amounts_format_with_thousands_commas`).
