//! fixes-3 item 1: no printed document may contain a blank or header-only
//! page. The blank pages were structural: continuation sheets emitted with a
//! fixed full-page height fragment inside the height:auto `.sheet-flow`
//! print wrapper, spilling an empty slice of the page box onto the following
//! physical page. Continuation pages now size to their content.
use bir_core::forms::form_1707a::{Form1707AAtc, Form1707ADraft, Form1707ARow};
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

fn row(date: &str, name: &str, selling: f64, cost: f64, tax_paid: f64) -> Form1707ARow {
    Form1707ARow {
        date: date.into(),
        corporation: name.into(),
        selling_price: selling,
        cost,
        gain_or_loss: 0.0,
        tax_paid,
    }
}

#[test]
fn continuation_pages_do_not_carry_a_fixed_full_page_height_that_spills_blank() {
    let mut draft = Form1707ADraft::new_from_profile(&demo_profile(), 2025, 1);
    draft.set_atc(Form1707AAtc::Individual);
    draft.year_end_year = 2023;
    draft.number_of_attached_sheets = 1;
    draft.gains = vec![
        row(
            "02/15/2023",
            "Sample Corp",
            500_000.009,
            120_000.0,
            15_000.5,
        ),
        row(
            "06/30/2023",
            "Example Mining",
            1_234_567.891,
            1_000_000.105,
            0.0,
        ),
    ];
    for k in 3..=14u32 {
        draft.gains.push(row(
            "02/10/2023",
            &format!("Gain Co {k}"),
            f64::from(k) * 2_000.509,
            f64::from(k) * 1_000.0,
            0.0,
        ));
    }
    draft.losses = vec![row("09/01/2023", "Loss Co", 50_000.0, 80_000.55, 0.0)];
    draft.recompute();
    let fields = draft.to_print_field_map();
    let doc = filled_document_with_report("1707a-2021", &fields, None).unwrap();
    // A `.page-continuation` div fixed at the native page height fragments in
    // the `.sheet-flow` print wrapper and spills a pure-white page.
    let mut scan = 0;
    while let Some(at) = doc.html[scan..]
        .find("class=\"page page-continuation")
        .map(|i| scan + i)
    {
        let end = doc.html[at..]
            .find('>')
            .map(|i| at + i)
            .unwrap_or(doc.html.len());
        let tag = &doc.html[at..end];
        assert!(
            !tag.contains("height:936pt") && !tag.contains("height:936"),
            "continuation page carries a fixed full-page height (blank-spill): {}",
            &tag[..tag.len().min(160)]
        );
        scan = end;
    }
}
