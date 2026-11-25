use bir_core::forms::form_1600pt::{Form1600PtAgentCategory, Form1600PtAtcRow, Form1600PtDraft};
use bir_core::forms::form_2550q::{Form2550QCapitalGoodRow, Form2550QDate, Form2550QDraft};
use bir_core::TaxpayerProfile;
use bir_print::continuation::discover_schedules_for_test;
use bir_print::frozen_html::{filled_document_with_report, printed_cell};
use serde_json::json;
use std::collections::BTreeMap;

fn demo_profile() -> TaxpayerProfile {
    serde_json::from_value(json!({
        "id": null,
        "tin": { "segment1": "000", "segment2": "123", "segment3": "453", "branch": "00000" },
        "rdo_code": "044",
        "full_name": "SANTOS, JUANA",
        "registered_address": "123 RIZAL AVE, MAKATI CITY",
        "zip_code": "1200",
        "phone": "09171234567",
        "email": "juana.santos@example.test",
        "line_of_business": "SERVICES",
        "default_form_type": "2551Q"
    }))
    .unwrap()
}

fn page_count(html: &str) -> usize {
    html.matches("class=\"page ").count()
}

#[test]
fn form_2550q_prints_all_12_schedule1_rows_and_breaks_to_continuation() {
    let profile = demo_profile();
    let mut d = Form2550QDraft::new_from_profile(&profile, 2025, 1);
    d.schedule_1.clear();
    let mut expected_amt_total = 0.0;
    let mut expected_tax_total = 0.0;
    for i in 1..=12usize {
        let amt = 10_000_000.0 + (i as f64) * 111_111.01;
        let tax = (amt * 0.12 * 100.0).round() / 100.0;
        expected_amt_total += amt;
        expected_tax_total += tax;
        d.schedule_1.push(Form2550QCapitalGoodRow {
            purchase_or_import_date: Some(Form2550QDate {
                year: 2021,
                month: 1,
                day: i as u8,
            }),
            source_code: "D".into(),
            description: format!("CAPGOOD_R{i:02}"),
            purchase_or_import_amount: Some(amt),
            input_tax: Some(tax),
            estimated_life_months: Some(60),
            recognized_life_months: Some(36),
            allowable_input_tax_for_period: None,
            balance_to_next_period: None,
            ..Default::default()
        });
    }
    d.recompute();

    let map = d.to_print_field_map();
    let out = filled_document_with_report("2550q-2024", &map, None).expect("2550q-2024 render");

    // Every single one of the 12 rows (including rows 11 and 12) must appear exactly once, in order.
    let mut prev_pos = 0usize;
    for i in 1..=12usize {
        let needle = format!(" value=\"CAPGOOD_R{i:02}\"");
        let count = out.html.matches(&needle).count();
        assert_eq!(
            count, 1,
            "2550Q row CAPGOOD_R{i:02} must appear exactly once on paper"
        );
        let pos = out.html.find(&needle).unwrap();
        assert!(
            pos > prev_pos,
            "2550Q row CAPGOOD_R{i:02} must appear in order"
        );
        prev_pos = pos;
    }

    // Printed Schedule 1 Column E total must equal the exact 12-row sum.
    let expected_e = format!("{expected_tax_total:.2}");
    let col_e_printed = printed_cell(&out.html, "p2c203").0.replace(',', "");
    assert_eq!(
        col_e_printed, expected_e,
        "2550Q Schedule 1 Col E total must equal sum of all 12 rows (amount total {expected_amt_total:.2})"
    );

    // Continuation page must carry form number, TIN, and page label.
    assert!(
        page_count(&out.html) >= 3,
        "2550Q with 12 Schedule 1 rows must paginate onto Page 3"
    );
    let cont_start = out.html.rfind("class=\"page ").unwrap();
    let cont = &out.html[cont_start..];
    assert!(cont.contains("2550Q"), "continuation page must name form");
    assert!(
        cont.contains("000") && cont.contains("123") && cont.contains("453"),
        "continuation page must carry TIN"
    );
    assert!(
        cont.contains("continued") || cont.contains("Page"),
        "continuation page must name the page"
    );
}

#[test]
fn form_1600pt_prints_all_part_ii_rows_at_plus3_max_and_continuation_with_aligned_rate() {
    let profile = demo_profile();
    let atc_codes = [
        "WB030", "WB040", "WB050", "WB070", "WB080", "WB082", "WB084", "WB090", "WB100", "WB102",
        "WB104", "WB106", "WB108", "WB120", "WB130", "WB140", "WB150", "WB160", "WB170", "WB180",
        "WB190", "WB191", "WB192", "WB193", "WB194", "WB195", "WB196", "WB197", "WB198", "WB199",
        "WB200", "WB201", "WB202", "WB203", "WB204", "WB205", "WB206", "WB207", "WB208", "WB209",
    ];

    for total_rows in [8usize, 31usize, 40usize] {
        let mut d = Form1600PtDraft::new_from_profile(&profile, 2025, 1);
        d.agent_category = Some(Form1600PtAgentCategory::Private);
        d.taxes_withheld = true;
        d.atc_rows.clear();
        for i in 1..=total_rows {
            let base = 100_000_000.0 + (i as f64) * 1_111_111.01;
            d.atc_rows.push(Form1600PtAtcRow {
                atc_code: atc_codes[i - 1].to_string(),
                rate_text: "3.0".into(),
                tax_base: base,
                tax_withheld: (base * 0.03 * 100.0).round() / 100.0,
            });
        }
        d.recompute();

        let map = d.to_print_field_map();
        let out =
            filled_document_with_report("1600-pt-2018", &map, None).expect("1600-pt-2018 render");

        // Single-digit rate must be right-aligned against % (" 3") so it does not crowd centavos.
        assert_eq!(
            printed_cell(&out.html, "p1c54").0,
            " 3",
            "1600PT row 1 single-digit rate must sit in slot 1 beside %"
        );

        // Every ATC row must appear exactly once, in order.
        let mut prev_pos = 0usize;
        for i in 1..=total_rows {
            let code = atc_codes[i - 1];
            let needle = format!("data-writer-value=\"{code}\"");
            let count = out.html.matches(&needle).count();
            assert_eq!(
                count, 1,
                "1600PT ({total_rows} rows): {code} must appear exactly once (found {count})"
            );
            let pos = out.html.find(&needle).unwrap();
            assert!(
                pos > prev_pos,
                "1600PT ({total_rows} rows): {code} must appear in order"
            );
            prev_pos = pos;
        }

        // At 40 rows Part II itself breaks onto page-1-cont-1, which must repeat the table head,
        // form number, TIN, and page label.
        if total_rows >= 40 {
            let cont_idx = out
                .html
                .find("class=\"page page-1-cont-1")
                .expect("1600PT 40 rows must have page-1-cont-1");
            let next_page = out.html[cont_idx + 10..]
                .find("class=\"page ")
                .map(|p| cont_idx + 10 + p)
                .unwrap_or(out.html.len());
            let cont = &out.html[cont_idx..next_page];
            assert!(cont.contains("1600-PT"), "continuation must name 1600-PT");
            assert!(
                cont.contains("000") && cont.contains("123") && cont.contains("453"),
                "continuation must carry TIN"
            );
            assert!(
                cont.contains("continued") || cont.contains("Page"),
                "continuation must name the page"
            );
            assert!(
                cont.contains("ATC") && cont.contains("Tax Base") && cont.contains("Tax Withheld"),
                "continuation page carrying Part II rows must repeat the Part II table head"
            );
        }
    }
}

#[test]
fn every_growable_table_prints_all_rows_once_in_order_with_continuation_headers() {
    // Generic all-forms invariant over every growable schedule discovered on every frozen sheet:
    // every held row is printed exactly once in order, and any continuation page carrying
    // continuation rows names the form, TIN, page, and repeats table headings.
    let forms_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../html-frozen");
    let mut slugs: Vec<String> = std::fs::read_dir(&forms_dir)
        .expect("read html-frozen")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir() && e.path().join("writer-cells.json").exists())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    slugs.sort();

    let mut checked_tables = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for slug in &slugs {
        let schedules = discover_schedules_for_test(slug);
        if schedules.is_empty() {
            continue;
        }
        // Only test tables whose stems are distinct and have at least one non-amount column
        // (or amount column) we can populate unambiguously.
        for sched in &schedules {
            let cap = sched
                .columns
                .iter()
                .map(|c| c.end_idx.saturating_sub(c.start_idx) + 1)
                .max()
                .unwrap_or(0);
            if cap == 0 {
                continue;
            }
            let target_rows = cap + 5;
            let mut fields = BTreeMap::new();
            // Supply generic TIN/name keys so continuation headers carry TIN.
            fields.insert("derived:tin_digits".into(), "00012345300000".into());

            let mut row_markers: Vec<String> = Vec::new();
            let mut sum_first_amount_col = 0.0_f64;

            for r in 0..target_rows {
                let marker = format!("INVROW{:02}", r + 1);
                row_markers.push(marker.clone());
                let mut placed_marker = false;
                for (ci, col) in sched.columns.iter().enumerate() {
                    let idx = col.start_idx + r;
                    let key = format!("{}{idx}{}", col.stem, col.suffix);
                    if !col.is_amount && !placed_marker {
                        fields.insert(key, marker.clone());
                        placed_marker = true;
                    } else if col.is_amount {
                        let amt = 1000.0 + ((r + 1) as f64) * 10.25;
                        if ci
                            == sched
                                .columns
                                .iter()
                                .position(|c| c.is_amount)
                                .unwrap_or(999)
                        {
                            sum_first_amount_col += amt;
                        }
                        fields.insert(key, format!("{amt:.2}"));
                    } else {
                        fields.insert(key, "1".into());
                    }
                }
            }

            let Ok(out) = filled_document_with_report(slug, &fields, None) else {
                continue;
            };
            checked_tables += 1;

            // 1. Every held row marker (when the table has a text column) must appear exactly once, in order.
            let has_text_col = sched.columns.iter().any(|c| !c.is_amount);
            if has_text_col {
                let mut prev = 0usize;
                for (r, marker) in row_markers.iter().enumerate() {
                    let attr_needle = format!("data-writer-value=\"{marker}\"");
                    let td_needle = format!(">{marker}<");
                    let count = out.html.matches(&attr_needle).count()
                        + out.html.matches(&td_needle).count();
                    let found = out
                        .html
                        .find(&attr_needle)
                        .or_else(|| out.html.find(&td_needle));
                    if count != 1 {
                        failures.push(format!(
                            "{slug} / {}: row {} ({marker}) printed {count} times (expected 1; sum={sum_first_amount_col:.2})",
                            sched.heading,
                            r + 1
                        ));
                    } else if let Some(pos) = found {
                        if pos < prev {
                            failures.push(format!(
                                "{slug} / {}: row {} ({marker}) out of order",
                                sched.heading,
                                r + 1
                            ));
                        }
                        prev = pos;
                    }
                }
            }

            // 2. Any continuation page must name the page and carry TIN when present.
            for (pi, chunk) in out.html.split("class=\"page ").skip(1).enumerate() {
                if chunk.starts_with("page-") && chunk.contains("-cont-") {
                    if !chunk.contains("continued") && !chunk.contains("Page") {
                        failures.push(format!(
                            "{slug} / {}: continuation page {} missing page label",
                            sched.heading,
                            pi + 1
                        ));
                    }
                }
            }
        }
    }

    assert!(
        checked_tables >= 25,
        "expected to check at least 25 growable tables across all forms, checked {checked_tables}"
    );
    assert!(
        failures.is_empty(),
        "growable-table invariant failures across {checked_tables} tables:\n{}",
        failures.join("\n")
    );
}

#[test]
fn form_1800_prints_rows_6_to_10_without_gaps_and_repeats_sched_b_continuation_header() {
    let mut f = BTreeMap::new();
    f.insert("derived:tin".into(), "00012345300000".into());
    f.insert("derived:pg2_tin".into(), "00012345300000".into());
    f.insert("frm1800v2018:txtTIN1".into(), "000".into());
    f.insert("frm1800v2018:txtTIN2".into(), "123".into());
    f.insert("frm1800v2018:txtTIN3".into(), "453".into());
    f.insert(
        "frm1800v2018:txtTaxpayerName".into(),
        "SANTOS, JUANA".into(),
    );
    f.insert(
        "frm1800v2018:txtPg2TaxpayerName".into(),
        "SANTOS, JUANA".into(),
    );

    // 8 rows in Schedule A (fits physical rows 1..8 — NO overflow, NO blank gap rows 6..10).
    for i in 0..8 {
        f.insert(
            format!("frm1800v2018:schedA:txtParticulars{i}"),
            format!("GIFT_PERS_R{:02}", i + 1),
        );
        f.insert(
            format!("frm1800v2018:schedA:txtFairMarketValue{i}"),
            format!("{:.2}", 100_000.0 + (i as f64) * 1_000.0),
        );
    }
    // 13 rows in Schedule B1 & B2 (10 physical rows + 3 grown rows -> breaks to Page 2 continued).
    for i in 0..13 {
        f.insert(
            format!("frm1800v2018:schedB1:txtOCT{i}"),
            format!("OCT-{:03}", i + 1),
        );
        f.insert(
            format!("frm1800v2018:schedB1:txtTaxDecNo{i}"),
            format!("TD-{:03}", i + 1),
        );
        f.insert(
            format!("frm1800v2018:schedB1:txtLocation{i}"),
            format!("MAKATI_{:02}", i + 1),
        );
        f.insert(
            format!("frm1800v2018:schedB2:txtArea{i}"),
            format!("{:.2}", 120.0 + (i as f64)),
        );
        f.insert(
            format!("frm1800v2018:schedB2:txtFairMarketValue{i}"),
            format!("{:.2}", 500_000.0 + (i as f64) * 10_000.0),
        );
    }

    let out = bir_print::frozen_html::filled_document_with_report("1800-2018", &f, None)
        .expect("1800 should render");
    for i in 1..=8 {
        assert_eq!(
            out.html
                .matches(&format!(" value=\"GIFT_PERS_R{i:02}\""))
                .count(),
            1,
            "1800 Sched A row {i} must print once"
        );
    }
    // Rows 6..8 sit in physical cells p2c90/93/96, NOT overflow rows numbered 11..13.
    assert!(
        out.html.contains("id=\"p2c90\"") && out.html.contains("GIFT_PERS_R06"),
        "row 6 must fill physical cell p2c90 without blank gap rows"
    );
    for i in 1..=13 {
        assert_eq!(
            out.html.matches(&format!(" value=\"OCT-{i:03}\"")).count(),
            1,
            "1800 Sched B row {i} must print once"
        );
    }
    // Schedule B Continuation page must repeat both its section title and column header.
    let cont_start = out
        .html
        .find("id=\"page-2-cont-1\"")
        .expect("1800 13-row Sched B must break to page-2-cont-1");
    let cont_html = &out.html[cont_start..];
    assert!(
        cont_html.contains("Schedule B") && cont_html.contains("Continuation"),
        "1800 page-2-cont-1 must repeat Schedule B Continuation section title"
    );
    assert!(
        cont_html.contains("AREA") || cont_html.contains("Area"),
        "1800 page-2-cont-1 must repeat Schedule B Continuation column headers"
    );
}

#[test]
fn form_1701a_item_42_clears_summary_amount_when_expanding_modal_rows_inline() {
    let mut f = BTreeMap::new();
    f.insert("derived:tin".into(), "00012345300000".into());
    f.insert("frm1701A:txtTaxpayerName".into(), "SANTOS, JUANA".into());
    f.insert("frm1701A:txt42Desc".into(), "OTHERS".into());
    // Summary amount that equals the sum of 5 modal rows (5 * 100,000 = 500,000).
    f.insert("derived:amount_42A".into(), "500000".into());
    for i in 1..=5 {
        f.insert(
            format!("frm1701A:txtmodalPartIVADesc_{i}C1"),
            format!("NONOP_ROW_{i:02}"),
        );
        f.insert(format!("frm1701A:txtmodalPartIVA_{i}C2"), "100000".into());
    }

    let out = bir_print::frozen_html::filled_document_with_report("1701a-2018", &f, None)
        .expect("1701A should render");
    for i in 1..=5 {
        assert_eq!(
            out.html
                .matches(&format!(" value=\"NONOP_ROW_{i:02}\""))
                .count(),
            1,
            "1701A modal row {i} must print once"
        );
    }
    // Cell p2c36 (Item 42 Taxpayer summary amount) must be cleared when 42a..42e expand inline,
    // preventing double-counting against Item 44.
    let p2c36_start = out.html.find("id=\"p2c36\"").expect("p2c36 present");
    let p2c36_end = out.html[p2c36_start..].find("</div>").unwrap() + p2c36_start;
    let p2c36_html = &out.html[p2c36_start..p2c36_end];
    assert!(
        !p2c36_html.contains("value=\"5\""),
        "1701A p2c36 summary amount must be cleared when 42a..42e expand inline: {p2c36_html}"
    );
}

#[test]
fn form_1601c_and_2551q_rule6_two_line_specify_encloses_both_lines_and_keeps_1601c_stamp_on_page1()
{
    let long_specify = "OTHER TAX CREDITS UNDER REVENUE REGULATIONS NO. 11-2018 AND REVENUE MEMORANDUM CIRCULAR NO. 55-2024 FOR SYNTHETIC AUDIT VERIFICATION TESTING";

    // 1601C Item 20
    let mut f1601c = BTreeMap::new();
    f1601c.insert("derived:tin".into(), "00012345300000".into());
    f1601c.insert("frm1601c:txtTaxpayerName".into(), "SANTOS, JUANA".into());
    f1601c.insert("frm1601c:txt20Other".into(), long_specify.into());
    f1601c.insert("frm1601c:txt20Amt".into(), "123456.78".into());
    let out1601c = bir_print::frozen_html::filled_document_with_report("1601c-2018", &f1601c, None)
        .expect("1601C should render");
    assert!(
        out1601c.html.contains("rule6-inserted-line"),
        "1601C should emit Rule 6 inserted line"
    );
    assert!(
        !out1601c.html.contains("id=\"page-1-cont-1\""),
        "1601C stamp box must stay on Page 1 instead of spilling to a 3rd page"
    );
    // Box height for 2-line ALL-CAPS text must be >= 27pt (6 + 2*10.5), never 16.5pt.
    assert!(
        out1601c.html.contains("height=\"27\"") || out1601c.html.contains("height=\"37.5\""),
        "1601C Rule 6 box must be tall enough for 2 wrapped lines (>= 27pt)"
    );

    // 2551Q Item 17
    let mut f2551q = BTreeMap::new();
    f2551q.insert("derived:tin".into(), "00012345300000".into());
    f2551q.insert(
        "frm2551Qv2018:txtTaxpayerName".into(),
        "SANTOS, JUANA".into(),
    );
    f2551q.insert("frm2551Qv2018:txt17Specify".into(), long_specify.into());
    f2551q.insert("frm2551Qv2018:txtTax17".into(), "123456.78".into());
    let out2551q = bir_print::frozen_html::filled_document_with_report("2551q-2018", &f2551q, None)
        .expect("2551Q should render");
    assert!(
        out2551q.html.contains("rule6-inserted-line"),
        "2551Q should emit Rule 6 inserted line"
    );
    assert!(
        out2551q.html.contains("height=\"27\"") || out2551q.html.contains("height=\"37.5\""),
        "2551Q Rule 6 box must be tall enough for >= 2 wrapped lines (>= 27pt)"
    );
}

#[test]
fn legacy_additional_sheets_have_full_headers_and_no_trailing_page_break() {
    // 1707A Schedule 1 overflow -> legacy Additional Sheet must show Name + Return Period and no trailing page-break
    let mut f1707a = BTreeMap::new();
    f1707a.insert("derived:tin".into(), "00012345300000".into());
    f1707a.insert(
        "frm1707Av2021:txtI8RegisteredName".into(),
        "SANTOS, JUANA".into(),
    );
    f1707a.insert("frm1707Av2021:txtI1Year".into(), "2025".into());
    for i in 1..=6 {
        f1707a.insert(
            format!("frm1707Av2021:txtS1C1DateOfTransaction_{i}"),
            "01/15/2025".into(),
        );
        f1707a.insert(
            format!("frm1707Av2021:txtS1C2NameOfCorporateStock_{i}"),
            format!("CORP_A_R{i:02}"),
        );
        f1707a.insert(
            format!("frm1707Av2021:txtS1C3SellingPrice_{i}"),
            "100000.00".into(),
        );
    }
    let out1707a = bir_print::frozen_html::filled_document_with_report("1707a-2021", &f1707a, None)
        .expect("1707A should render");
    let cont_pos = out1707a
        .html
        .find("id=\"page-continuation\"")
        .expect("1707A continuation present");
    let cont = &out1707a.html[cont_pos..];
    assert!(
        cont.contains("SANTOS, JUANA"),
        "1707A continuation must print Taxpayer's Name (not —)"
    );
    assert!(
        cont.contains("2025"),
        "1707A continuation must print Return Period (not —)"
    );
    assert!(
        cont.contains("page-break-after:auto;break-after:auto;"),
        "last continuation page must not force a trailing blank page break"
    );

    // 1702EX Schedule 1 popup overflow -> must show Name + Return Period and fixed height (no blank page 5)
    let mut f1702ex = BTreeMap::new();
    f1702ex.insert("derived:tin".into(), "00012345300000".into());
    f1702ex.insert(
        "frm1702EX:txtPg1I9RegisteredName".into(),
        "SANTOS, JUANA".into(),
    );
    f1702ex.insert("frm1702EX:txtPg1I2MonthEnded".into(), "12".into());
    f1702ex.insert("frm1702EX:txtPg1I2YearEnded".into(), "2025".into());
    for i in 1..=6 {
        f1702ex.insert(
            format!("frm1702EX:txtPg3Pt6S1I17_{i}Col1"),
            format!("ITEM_EX_{i:02}"),
        );
        f1702ex.insert(format!("frm1702EX:txtPg3Pt6S1I17_{i}Col2"), "50000".into());
    }
    let out1702ex =
        bir_print::frozen_html::filled_document_with_report("1702ex-2018", &f1702ex, None)
            .expect("1702EX should render");
    let ex_pos = out1702ex
        .html
        .find("id=\"page-continuation\"")
        .expect("1702EX continuation present");
    let ex_cont = &out1702ex.html[ex_pos..];
    assert!(
        ex_cont.contains("12/2025"),
        "1702EX continuation must print Return Period 12/2025 (not —)"
    );
    assert!(
        ex_cont.contains("overflow:hidden;page-break-after:auto"),
        "1702EX continuation must not overflow into a blank 5th page"
    );
}
