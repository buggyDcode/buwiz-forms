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
    let base_hits: Vec<usize> = (1..=3).map(|i| doc.html.matches(&format!("MX-SC6-BASE-{i:02}")).count()).collect();
    let more_hits: Vec<usize> = (1..=7).map(|i| doc.html.matches(&format!("MX-SC6-MORE-{i:02}")).count()).collect();
    let has_tin = doc.html.contains("000-123-453-00000");
    assert_eq!(pages, 5, "1702MX must have 4 base pages + 1 continuation page, got {pages}");
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
