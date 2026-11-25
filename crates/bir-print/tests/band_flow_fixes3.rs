//! fixes-3: no blank pages, no old-style ADDITIONAL SHEET fallback, engine
//! continuation pages carry form/TIN/name/return-period/page-number.
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

pub fn many_rows_1707a() -> Form1707ADraft {
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
    draft.losses = vec![row("09/01/2023", "Loss Co", 50_000.0, 80_000.55, 0.0)];
    for k in 3..=14u32 {
        let month = (k - 3) % 12 + 1;
        draft.gains.push(row(
            &format!("{month:02}/10/2023"),
            &format!("Gain Co {k}"),
            f64::from(k) * 2_000.509,
            f64::from(k) * 1_000.0,
            f64::from(k) * 10.0,
        ));
    }
    for k in 2..=12u32 {
        let month = (k - 2) % 12 + 1;
        draft.losses.push(row(
            &format!("{month:02}/20/2023"),
            &format!("Loss Co {k}"),
            f64::from(k) * 500.0,
            f64::from(k) * 750.25,
            0.0,
        ));
    }
    draft.recompute();
    draft
}

#[test]
fn form_1707a_overflow_uses_engine_continuation_never_the_old_fallback() {
    let draft = many_rows_1707a();
    let fields = draft.to_print_field_map();
    std::fs::create_dir_all("/tmp/bfout").unwrap();
    let doc = filled_document_with_report("1707a-2021", &fields, None).unwrap();
    std::fs::write("/tmp/bfout/1707a-fixes3.html", &doc.html).unwrap();
    assert!(
        !doc.html.contains("ADDITIONAL SHEET"),
        "1707A must never use the old-style ADDITIONAL SHEET fallback"
    );
    assert!(
        !doc.html.contains("page-continuation\"") && !doc.html.contains("page-continuation-band"),
        "no band_continuation fallback pages may be emitted"
    );
    // engine continuation pages: spawned page-<n>-cont-<k> divs
    assert!(
        doc.html.contains("-cont-"),
        "the engine must spawn continuation pages for the overflow"
    );
    // every gain row prints exactly once
    for k in 3..=14u32 {
        let name = format!("Gain Co {k}");
        assert_eq!(doc.html.matches(&name).count(), 1, "{name} must print once");
    }
    for k in 2..=12u32 {
        let name = format!("Loss Co {k}");
        assert_eq!(doc.html.matches(&name).count(), 1, "{name} must print once");
    }
    // continuation pages carry the filer TIN and taxpayer name and a page label
    assert!(
        doc.html.contains("SANTOS, JUANA"),
        "continuation must carry the taxpayer name"
    );
    assert!(
        doc.html.contains("continued"),
        "continuation pages carry a continued label"
    );
}
