# 1701A Item 42 Fix Proof

- Cleared summary amount cells (`p2c36`/`p2c37` for Item 42, `p2c69`/`p2c70` for Item 51) in `crates/bir-print/src/band_flow.rs` whenever band-flow expands itemized modal rows inline directly beneath `OTHERS`, eliminating double-counting against Item 44.
