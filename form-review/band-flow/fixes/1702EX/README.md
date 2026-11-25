# 1702EX Continuation Sheet Fix Proof

- Populated Taxpayer's Name (`txtPg1I9RegisteredName`) and Return Period (`txtPg1I2MonthEnded`/`txtPg1I2YearEnded`) in `crates/bir-print/src/continuation.rs`, and switched continuation page height to fixed `height:{height};overflow:hidden;page-break-after:auto` on the final page so Chrome never spills a blank 5th page.
