# 1601C Rule 6 Specify & Stamp Box Fix Proof

- Replaced optimistic character-count line estimate in `Rule6Insertion::from_text` (`crates/bir-print/src/band_flow.rs`) with word-aware ALL-CAPS glyph measurement so 2-line specify text gets a 27pt box.
- Allowed elastic signature/stamp boxes on Rule 6-only pages to absorb the extra 10.5–27pt within the page height so the Stamp of Receiving Office stays on Page 1 (2 pages total, no headerless 3rd page).
