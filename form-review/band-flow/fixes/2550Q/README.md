# 2550Q Schedule 1 Band-Flow Fix Proof

- **Before (`b9552e27`)**: With 12 Schedule 1 rows (`CAPGOOD_R01`..`CAPGOOD_R12`), `form_2550q.rs` emitted keys with `suffix = 10 + offset`, producing `txtDatePurchase10`, `txtDatePurchase11`, ... instead of `11`, `12` (`format!("1{offset}")`), colliding with row 10 (`10`) and causing rows 11 and 12 (`CAPGOOD_R11`, `CAPGOOD_R12`) to be dropped while Column E Total (`15,439,999.04`) still summed all 12.
- **After**: All 12 rows print once in order (`CAPGOOD_R01`..`CAPGOOD_R12`), Column E Total equals `15,439,999.04`, and Schedule 4 flows cleanly to Page 3 (`page-2-cont-1`) with form number, TIN, and page label.
- **Files**: `plus3-{before,after}.pdf`, `max-{before,after}.pdf`, page PNGs, `crop-fix.png`, `results.tsv`, `recording.mp4`.
