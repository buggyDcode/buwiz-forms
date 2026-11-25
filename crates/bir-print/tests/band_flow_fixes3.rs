//! fixes-3 item 2: continuation pages carry every element of the engine's
//! own continuation page — the form, the filer's TIN with branch code, the
//! taxpayer name, the return period, a page number and the table head.
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
fn form_1707a_continuation_pages_carry_the_full_engine_header() {
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
    for k in 2..=12u32 {
        draft.losses.push(row(
            "09/20/2023",
            &format!("Loss Co {k}"),
            f64::from(k) * 500.0,
            f64::from(k) * 750.25,
            0.0,
        ));
    }
    draft.recompute();
    let fields = draft.to_print_field_map();
    let doc = filled_document_with_report("1707a-2021", &fields, None).unwrap();
    std::fs::create_dir_all("/tmp/bfout").unwrap();
    std::fs::write("/tmp/bfout/1707a-fixes3.html", &doc.html).unwrap();

    assert!(
        doc.html.contains("Additional sheet") || doc.html.contains("-cont-"),
        "overflow must produce continuation pages"
    );
    for needle in [
        "000-123-453-00000",
        "SANTOS, JUANA",
        "12/31/2023",
        "— Page 3",
        "(A) Date of Transaction",
    ] {
        assert!(
            doc.html.contains(needle),
            "continuation must carry {needle}"
        );
    }
    assert!(
        !doc.html.contains("Return Period:</span> —"),
        "the return period must never be the unknown dash"
    );
    let upper = doc.html.to_uppercase();
    let count = |name: &str| {
        // paper rows print inside one input (VALUE= plus DATA-WRITER-VALUE=);
        // continuation rows print once as table text.
        regex_count(&upper, name)
    };
    fn regex_count(hay: &str, name: &str) -> usize {
        let mut n = 0;
        let mut from = 0;
        while let Some(i) = hay[from..].find(name) {
            let at = from + i + name.len();
            if !hay[at..].starts_with(|c: char| c.is_ascii_digit()) {
                n += 1;
            }
            from = from + i + 1;
        }
        n
    }
    for k in 3..=14u32 {
        let name = format!("GAIN CO {k}");
        let want = if k == 3 { 2 } else { 1 };
        assert_eq!(count(&name), want, "{name} must print once");
    }
    for k in 2..=12u32 {
        let name = format!("LOSS CO {k}");
        let want = if k <= 3 { 2 } else { 1 };
        assert_eq!(count(&name), want, "{name} must print once");
    }
}

/// fixes-3 item 3: 1600-PT's "Page 1 (continued)" must carry the filer's TIN
/// and taxpayer name, not the form banner alone. 40 Part II rows put rows
/// 32-40 onto the continuation page; the TIN comb cells (p1c16/18/20/22) and
/// the name cell must repeat there with the identity strip.
#[test]
fn form_1600pt_continuation_pages_carry_tin_and_name() {
    use bir_core::forms::form_1600pt::{
        Form1600PtAgentCategory, Form1600PtAtcRow, Form1600PtDraft,
    };
    let mut draft = Form1600PtDraft::new_from_profile(&demo_profile(), 2025, 6);
    draft.set_agent_category(Form1600PtAgentCategory::Government);
    draft.set_taxes_withheld(true);
    for k in 0..40u32 {
        draft.atc_rows.push(Form1600PtAtcRow {
            atc_code: "WB030".into(),
            rate_text: "3.0".into(),
            tax_base: 1_000.0 + f64::from(k),
            tax_withheld: 0.0,
        });
    }
    draft.recompute();
    let fields = draft.to_print_field_map();
    let doc = filled_document_with_report("1600-pt-2018", &fields, None).unwrap();
    std::fs::write("/tmp/bfout/1600pt-fixes3.html", &doc.html).unwrap();

    let page_count = doc.html.matches("class=\"page page-").count();
    assert!(
        page_count >= 2,
        "40 Part II rows must overflow onto a continuation page, got {page_count} page(s)"
    );
    // The TIN comb cells of page 1 must print once per page, not once total.
    for cell in ["p1c16", "p1c18", "p1c20", "p1c22"] {
        let occurrences = doc.html.matches(&format!("id=\"{cell}\"")).count();
        assert!(
            occurrences >= 2,
            "TIN comb cell {cell} prints {occurrences} time(s) across {page_count} pages:              the continuation page carries no TIN"
        );
    }
    // The taxpayer name must appear on every page fragment, not only page 1.
    let frags: Vec<&str> = doc.html.split("class=\"page page-").collect();
    for (idx, frag) in frags.iter().enumerate().skip(1) {
        assert!(
            frag.to_uppercase().contains("SANTOS"),
            "page fragment {idx} of {page_count} lacks the taxpayer name"
        );
    }
}
