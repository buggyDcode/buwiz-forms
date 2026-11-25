# 1800 Band-Flow Fix Proof

- Joined Schedule A/B1/B2 physical rows 6..10 in `html-frozen/1800-2018/writer-cells.json` (eliminates 5 blank gap rows and misnumbering).
- Split fused `[column-header + row-1]` `PlainBlock` (`p2b25`) in `crates/bir-print/src/band_flow.rs` and repeated prefix bands when breaking after row 1 so Page 2 (continued) repeats both `Schedule B – Continuation` section title and column headers.
