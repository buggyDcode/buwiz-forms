# 1602Q Blank Trailing Continuation Suppression Proof

- Added `has_body_slice` guard in `pack_page()` (`crates/bir-print/src/band_flow.rs`) so band-flow never emits a header-only continuation page when all rows already fit on the preceding page.
