# Independent Check of PR #366 (`fix/1701-schedule-gates` at `30cfaee6` vs `main` `a8aeaca8`)

**Verdict: `holds`** (with one Rank-3 UX note on saved-at-main drafts: no notice when previously saved values in now-shut boxes are hidden or silently zeroed by compute).

Report-only independent verification — no code modified, nothing pushed to `hexuria/buwiz-forms`, no PR comments posted.

## Provenance & Safety
- **Commits checked:** head `30cfaee6` (`fix/1701-schedule-gates`) vs main `a8aeaca8`
- **Isolated build folders (empty at start):**
  - `/home/ubuntu/BuwizNsh/target-366-main` (`13G`)
  - `/home/ubuntu/BuwizNsh/target-366-head` (`26G`)
  - `/home/ubuntu/BuwizNsh/target-366-head-tests` (`5.1G`)
  - `/home/ubuntu/BuwizNsh/target-nsh`: **untouched** (`4.0K`, `Nov 25 14:46`, root-owned)
- **Toolchain & env:** `RUSTC_WRAPPER=""`, `/home/ubuntu/.cargo/bin/cargo`, private `HOME` per checkout, `BUWIZ_NO_KEYCHAIN=1 EBIR_TEST_ENV=1 BIR_AUTH_BYPASS=1 BIR_SFTP_DRY_RUN=1`, `BIR_SFTP_LIVE` unset, demo taxpayer `SANTOS, JUANA DELA CRUZ` (`000-123-453-00000`), invented values, never Queue.

## Findings by Step
1. **Official rule rebuilt (`step1-official-rules.tsv`):** Rebuilt from `rules/forms/1701-v2018/fixtures/{runtime-control-inventory-v796.json,validation-function-inventory-v796.json}` and `form_1701_official.rs` for 20 selected boxes across Schedules 3.A, 4, and 6 (including Sch 4 items 1, 5, 17a; Sch 6 first NOLCO row `4Year`–`4D` and spouse `9Year`; and the three shared description boxes `19Desc`, `20Desc`, `17dDesc`). **0 disagreements** with the author's `gates-main.tsv` across all 130 standard rows (+ 3 explicit mixed-joint either-column rows).
2. **Live page at desktop width (`1430×890`, `step2-page-states.tsv`, `shots/{main,head}-s{1..5}-*.png`):** Checked all 5 states (`fresh`, `grad-itemized`, `grad-osd`, `eight-percent`, `compensation`) × `{single, joint}` = 200 box-states at both commits.
   - `main` (`a8aeaca8`): **124/200** officially-shut box-states left open.
   - `head` (`30cfaee6`): **200/200** match official rule (**0 Rank-1** officially open but shut at head; **0 Rank-2** officially shut but open at head).
3. **Either-column boxes (`step3-either-column.tsv`, `shots/head-step3-either-col.png`):** On joint filing with Taxpayer 8% (`II015`) and Spouse Graduated (`II012`) + Itemized:
   - `txtPg2IShed3_19Desc`, `txtPg2IShed3_20Desc`, and `txtPg3IShed4_17dDesc` are **open**; Taxpayer `19A`/`17dA` are **shut**; Spouse `19B`/`17dB` are **open**.
   - Invented values (`SPOUSE CONSULTING FEE 19`, `SPOUSE ROYALTY FEE 20`, `SPOUSE OFFICE SUPPLIES 17D`, `5,000.00`, `3,000.00`, `2,500.00`) persist through compute (`Item 18B = 2,500.00`), `data_json`, and upload XML (`any_graduated` / `itemized_any`).
   - **Official basis:** Matches the official v796 program (not a workaround) — BIR 1701 has a single shared DOM `<input>` per row without an A/B suffix; `enableFieldsForGRRate` enables `19Desc`/`20Desc` for either column while `disableFieldsForEightPerRate` only disables `19A`/`19B`, and `17dDesc` stays enabled when either column is itemized.
4. **Nothing lost (`step4-samples.tsv`):** Confirmed via `bir-core` `sample_payloads_test` (`146/146`), `bir-app-api` page-path test for `1701-122025` & `1701-122025-joint`, and manual live-app replay of `1701-122025-joint.steps.json` at `head` (`30cfaee6`): **0 differences** against `1701-122025-joint.official.xml` (Schedule 4 Item 18B = `71,001.00`, Schedule 5 spouse = `5,000.00`, Schedule 6 spouse = `15,000.00`, all 15 entered schedule boxes exact).
5. **Saved-at-main return opened at head (`step5-saved-return.tsv`, `shots/head-step5-saved-main.png`):**
   - **What the filer sees:** No banner, toast, or notice. Gated boxes are hidden; Schedule 4 shows only `"1 item"` (locked `0.00` total Item 18).
   - **Data behavior:** Description strings saved at `main` (`MAIN SAVED DESC 19`, `MAIN SAVED DEDUCTION 17D`) are **kept silently in `data_json`** (hidden in UI, empty in upload XML while on 8%). Numeric/NOLCO values in now-shut boxes were already silently zeroed by `main`'s compute; if a draft's `data_json` holds non-zero numeric/NOLCO values (`12345`, `6789`, `2024 / 10000`), opening and saving at `head` **zeroes/blanks them silently with no notice**.
6. **Tests at `30cfaee6`:**
   - `cargo test --locked -p bir-core --lib`: `1663 passed; 0 failed; 12 ignored`
   - `cargo test --locked -p bir-core --test sample_payloads_test`: `146 passed; 0 failed; 1 ignored`
   - `cargo test --locked -p bir-app-api --features agent --lib`: `846 passed; 0 failed; 7 ignored`
   - `nsh test screens/form/*.test.{ts,tsx}`: `46/47` files passed (`atc2551q.test.tsx` failed for the known unrelated reason on `main`; not counted)

## Artifacts
- `results.tsv`, `step1-official-rules.tsv`, `step2-page-states.tsv`, `step3-either-column.tsv`, `step4-samples.tsv`, `step5-saved-return.tsv`
- `shots/{main,head}-s{1..5}-*.png`, `shots/head-step3-either-col.png`, `shots/head-step5-saved-main.png`
- `check-366-gates.mp4` (`26.9s`, `1430×890`)
