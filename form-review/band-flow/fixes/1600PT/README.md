# 1600PT Part II Band-Flow Fix Proof

- **Before (`b9552e27`)**: `Form1600PtDraft::to_print_field_map()` truncated `atc_rows` to `FORM_1600PT_ATC_ROWS` (5), `continuation.rs` did not recognize numbered Part II captions (`13 ATC`, `13 Tax Base`, `13 Tax Rate`, `13 Tax Withheld`) or `derived:rate*`, and single-digit rates sat in slot 0 right next to Tax Base centavos.
- **After**: All 8 rows (`plus3`) and all 31 rows (`max`) print once in order (`13..17`, `17a..17z`), Item 18 Total Taxes Withheld equals the exact sum of printed rows (`18,822,222.14` / `206,566,663.65`), and single-digit rates right-align into slot 1 beside `%` (`" 3"`) away from centavos.
- **Files**: `plus3-{before,after}.pdf`, `max-{before,after}.pdf`, page PNGs, `crop-fix.png`, `results.tsv`, `recording.mp4`.
