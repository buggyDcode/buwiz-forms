use bir_core::forms::form_1702mx::{
    Form1702MXDraft, Form1702MXRegimeAmounts, Form1702MXSpecialDeductionRow, WholePeso,
    WholePesoInput,
};
use bir_core::TaxpayerProfile;
use bir_print::frozen_html::filled_document_with_report;
use serde_json::json;

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

fn wp(n: i64) -> WholePesoInput {
    WholePesoInput::from_amount(WholePeso(n))
}

#[test]
fn form_1702mx_prints_all_base_and_modal_schedule6_rows_with_total_and_tin() {
    let profile = demo_profile();
    let mut d = Form1702MXDraft::new_from_profile(&profile, 2025);
    for i in 0..3 {
        let amt = ((i + 1) * 10_000) as i64;
        d.schedule_6.rows[i] = Form1702MXSpecialDeductionRow {
            description: format!("MX-SC6-BASE-{:02}", i + 1),
            legal_basis: format!("RA-BASE-{:02}", i + 1),
            amounts: Form1702MXRegimeAmounts {
                exempt: wp(0),
                special: wp(0),
                regular: wp(amt),
                total: wp(amt),
            },
        };
    }
    for i in 1..=7 {
        let amt = (i * 3_000) as i64;
        d.schedule_6.more_rows.push(Form1702MXSpecialDeductionRow {
            description: format!("MX-SC6-MORE-{i:02}"),
            legal_basis: format!("RA-MORE-{i:02}"),
            amounts: Form1702MXRegimeAmounts {
                exempt: wp(0),
                special: wp(0),
                regular: wp(amt),
                total: wp(amt),
            },
        });
    }
    d.recompute();
    let fields = d.to_print_field_map();
    let doc = filled_document_with_report("1702mx-2018c", &fields, None).unwrap();
    let pages = doc.html.matches("class=\"page ").count();
    let base_hits: Vec<usize> = (1..=3)
        .map(|i| doc.html.matches(&format!("MX-SC6-BASE-{i:02}")).count())
        .collect();
    let more_hits: Vec<usize> = (1..=7)
        .map(|i| doc.html.matches(&format!("MX-SC6-MORE-{i:02}")).count())
        .collect();
    let has_tin = doc.html.contains("000-123-453-00000");
    assert_eq!(
        pages, 5,
        "1702MX must have 4 base pages + 1 continuation page, got {pages}"
    );
    assert!(
        base_hits.iter().all(|&c| c >= 1),
        "all 3 base rows must print: {base_hits:?}"
    );
    assert_eq!(
        more_hits,
        vec![1, 1, 1, 1, 1, 1, 1],
        "all 7 modal rows must print exactly once: {more_hits:?}"
    );
    // Total = (10k + 20k + 30k) + (3k + 6k + 9k + 12k + 15k + 18k + 21k) = 60,000 + 84,000 = 144,000
    assert_eq!(
        fields.get("frm1702MX:txtPg6Sc6I5CD").map(String::as_str),
        Some("144000"),
        "1702MX Schedule 6 Item 5 total must be in print map"
    );
    assert!(
        doc.html.contains("84,000.00") || doc.html.contains("84,000"),
        "1702MX continuation sheet must carry modal subtotal 84,000"
    );
    assert!(has_tin, "1702MX continuation sheet must carry filer TIN");
}

#[test]
fn form_2200p_prints_all_31_schedule1_rows_including_added_28_to_31() {
    use bir_core::forms::form_2200p::{Form2200PDraft, Form2200PRow};
    let profile = demo_profile();
    let mut d = Form2200PDraft::new_from_profile(&profile, 2025, 3);
    d.day = 13;
    d.schedule.resize(27, Form2200PRow::default());
    d.schedule[0].place_of_removal = "DEPOT 1".to_string();
    d.schedule[0].local_manufactured = Some(100.0);
    for r in 28..=31usize {
        let mut row = Form2200PRow::default();
        row.atc_digits = format!("{r:03}");
        row.description = format!("ADDED FUEL {r}");
        row.unit_of_measure = "PER KG".to_string();
        row.applicable_rate = Some(10.0);
        row.place_of_removal = format!("DEPOT {r}");
        row.local_manufactured = Some(100.0 * (r as f64));
        d.schedule.push(row);
    }
    d.recompute();
    let fields = d.to_print_field_map();
    let doc = filled_document_with_report("2200p-2020", &fields, None).unwrap();
    for r in 28..=31usize {
        let desc = format!("ADDED FUEL {r}");
        let depot = format!("DEPOT {r}");
        assert_eq!(
            doc.html.matches(&desc).count(),
            1,
            "2200P added row {r} description {desc} must appear once"
        );
        assert_eq!(
            doc.html.matches(&depot).count(),
            1,
            "2200P added row {r} place_of_removal {depot} must appear once"
        );
    }
    assert!(
        doc.html.contains("000-123-453-00000"),
        "2200P continuation sheet must carry filer TIN"
    );
}

#[test]
fn form_2551q_prints_all_10_schedule1_rows_with_matching_total() {
    use bir_core::forms::form_2551q::{Form2551QDraft, Schedule1Row};
    let profile = demo_profile();
    let mut d = Form2551QDraft::new_from_profile(&profile, 2025, 1);
    let atcs = [
        "PT010", "PT040", "PT041", "PT060", "PT070", "PT090", "PT101", "PT120", "PT140", "PT180",
    ];
    d.schedule_1.clear();
    let mut expected_total = 0.0;
    for (idx, code) in atcs.iter().enumerate() {
        let mut row = Schedule1Row::new(code).unwrap();
        row.taxable_amount = ((idx + 1) as f64) * 10_000.0;
        row.recompute();
        expected_total += row.tax_due;
        d.schedule_1.push(row);
    }
    d.recompute(None);
    let fields = d.to_print_field_map();
    let doc = filled_document_with_report("2551q-2018", &fields, None).unwrap();
    let cells = bir_print::frozen_html::printed_cells(&doc.html);
    let cell_values: Vec<&str> = cells.values().map(|(v, _)| v.as_str()).collect();
    for code in &atcs {
        let hits =
            cell_values.iter().filter(|v| **v == *code).count() + doc.html.matches(code).count();
        assert!(
            hits >= 1,
            "2551Q Schedule 1 row {code} must print (cell_values={:?})",
            cell_values
                .iter()
                .filter(|v| v.starts_with("PT"))
                .collect::<Vec<_>>()
        );
    }
    let expected_digits = format!("{expected_total:.2}");
    let norm = |s: &str| s.replace(',', "");
    assert_eq!(
        fields.get("frm2551Qv2018:txt14").map(|s| norm(s)),
        Some(expected_digits.clone()),
        "2551Q Item 14 total must equal sum of all 10 rows"
    );
    assert_eq!(
        fields.get("txtTotalSched1").map(|s| norm(s)),
        Some(expected_digits),
        "2551Q Schedule 1 total must equal sum of all 10 rows"
    );
}

#[test]
fn form_1601c_prints_all_10_schedule1_rows_with_matching_item26_total() {
    use bir_core::forms::form_1601c::{Form1601CDraft, Form1601CSchedule1Row};
    let profile = demo_profile();
    let mut d = Form1601CDraft::new_from_profile(&profile, 2025, 1);
    d.schedule_1.clear();
    for i in 1..=10usize {
        let mut row = Form1601CSchedule1Row {
            previous_month: format!("{i:02}/2024"),
            date_paid: format!("{i:02}/10/2024"),
            drawee_bank_code_or_agency: format!("BDO{i:02}"),
            payment_number: format!("OR{i:04}"),
            tax_paid: 1_000.0,
            should_be_tax_due: 1_500.0,
            adjustment: 500.0,
        };
        row.recompute();
        d.schedule_1.push(row);
    }
    d.compute();
    let fields = d.to_print_field_map();
    let doc = filled_document_with_report("1601c-2018", &fields, None).unwrap();
    let cells = bir_print::frozen_html::printed_cells(&doc.html);
    let cell_values: Vec<&str> = cells.values().map(|(v, _)| v.as_str()).collect();
    for i in 1..=10usize {
        let bank = format!("BDO{i:02}");
        let hits =
            cell_values.iter().filter(|v| **v == bank).count() + doc.html.matches(&bank).count();
        assert!(hits >= 1, "1601C Schedule I row {i} ({bank}) must print");
    }
    assert_eq!(
        fields
            .get("frm1601c:sched1:txtTotal1")
            .map(|s| s.replace(',', "")),
        Some("5000.00".to_string()),
        "1601C Schedule I total must equal 5000.00"
    );
}

#[test]
fn form_1702q_prints_all_6_schedule4_other_credits_with_total_and_tin() {
    use bir_core::forms::form_1702q::{Form1702qDraft, Form1702qOtherCredit};
    let profile = demo_profile();
    let mut d = Form1702qDraft::new_from_profile(&profile, 2025, 1);
    d.atc = "IC010_25%".to_string();
    d.other_credits.clear();
    for i in 1..=6usize {
        d.other_credits.push(Form1702qOtherCredit {
            description: format!("OTHER CREDIT {i:02}"),
            amount: (i as f64) * 1_000.0,
        });
    }
    d.recompute();
    let fields = d.to_print_field_map();
    let doc = filled_document_with_report("1702q-2018", &fields, None).unwrap();
    let cells = bir_print::frozen_html::printed_cells(&doc.html);
    let norm_cells: Vec<String> = cells
        .values()
        .map(|(v, _)| v.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();
    for i in 1..=6usize {
        let desc = format!("OTHER CREDIT {i:02}");
        let hits = norm_cells.iter().filter(|v| v.contains(&desc)).count()
            + doc.html.matches(&desc).count();
        assert!(
            hits >= 1,
            "1702Q Schedule 4 Other Credit row {i} ({desc}) must print"
        );
    }
    assert!(
        doc.html.contains("000-123-453-00000"),
        "1702Q continuation header must carry filer TIN 000-123-453-00000"
    );
    assert_eq!(
        fields
            .get("frm1702q:Sched4:txtTax7")
            .map(|s| s.replace(',', "")),
        Some("21000.00".to_string()),
        "1702Q Schedule 4 total must equal 21000.00"
    );
}

#[test]
fn form_2200a_prints_all_10_others_rows_with_total_and_tin() {
    use bir_core::forms::form_2200a::{Form2200AAmounts, Form2200ADraft, Form2200AOtherRow};
    let profile = demo_profile();
    let mut d = Form2200ADraft::new_from_profile(&profile, 2025, 1);
    d.others.clear();
    for i in 1..=10usize {
        d.others.push(Form2200AOtherRow {
            atc_digits: format!("{i:03}"),
            description: format!("ALCOHOL OTHER {i:02}"),
            bracket: format!("BRACKET {i}"),
            rate: "50.00".to_string(),
            amounts: Form2200AAmounts {
                export_exempt: 0.0,
                taxable: (i as f64) * 10.0,
                tax_due: (i as f64) * 500.0,
            },
        });
    }
    d.recompute();
    let fields = d.to_print_field_map();
    let doc = filled_document_with_report("2200a-2020", &fields, None).unwrap();
    for i in 1..=10usize {
        let desc = format!("ALCOHOL OTHER {i:02}");
        assert!(
            doc.html.contains(&desc),
            "2200A Schedule 1 Others row {i} ({desc}) must print"
        );
    }
    assert!(
        doc.html.contains("000-123-453-00000"),
        "2200A continuation header must carry filer TIN 000-123-453-00000"
    );
    assert_eq!(
        fields
            .get("frm2200Av2020:txtSched1_TotalDue")
            .map(|s| s.replace(',', "")),
        Some("27500.00".to_string()),
        "2200A Schedule 1 total must equal 27500.00"
    );
}
