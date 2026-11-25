use bir_core::forms::form_1801::{
    Form1801Draft, Form1801Particular, Form1801RealProperty, Form1801Split,
};
use bir_core::forms::queueable::QueueableForm;
use bir_core::TaxpayerProfile;
use bir_print::frozen_html::{filled_document, printed_cell};
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

#[test]
fn form_1801_prints_item5_tin_items_16_and_24_and_specify_boxes() {
    let profile = demo_profile();
    let mut d = Form1801Draft::new_from_profile(&profile, 2025, 1);
    d.death_month = 2;
    d.death_day = 14;
    d.death_year = 2025;
    d.administrator_name = "SANTOS, JUANITO".into();
    d.administrator_tin = "000456789".into();
    d.administrator_branch_code = "00001".into();
    d.real_properties.push(Form1801RealProperty {
        title_number: "TCT-1001".into(),
        tax_declaration_number: "TD-2001".into(),
        location: "MAKATI CITY".into(),
        lot_or_improvement: "LOT".into(),
        area: "500 SQM".into(),
        classification: "RESIDENTIAL".into(),
        fmv_per_tax_declaration: "150000000.25".into(),
        fmv_per_zonal_value: "210345678.55".into(),
        value: Form1801Split {
            exclusive: 210_345_678.55,
            conjugal: 140_234_567.35,
        },
    });
    d.other_personal.push(Form1801Particular {
        particulars: "TIME DEPOSIT ACCOUNT".into(),
        value: Form1801Split {
            exclusive: 12_345_678.40,
            conjugal: 24_680_135.20,
        },
    });
    d.ordinary_deductions.claims_against_estate = Form1801Split {
        exclusive: 5_100_000.15,
        conjugal: 3_200_000.25,
    };
    d.ordinary_deductions.others_description = "COURT-APPROVED FUNERAL CLAIMS".into();
    d.ordinary_deductions.others = Form1801Split {
        exclusive: 1_250_000.50,
        conjugal: 750_000.25,
    };
    d.other_special_deduction_description = "SPECIAL TREATY ALLOWANCE".into();
    d.other_special_deduction = 4_500_000.75;
    d.compute();

    let map = d.print_field_map();
    let doc = filled_document("1801-2018", &map).expect("1801-2018 render");

    assert_eq!(printed_cell(&doc, "p1c13").0, "000");
    assert_eq!(printed_cell(&doc, "p1c14").0, "123");
    assert_eq!(printed_cell(&doc, "p1c15").0, "453");
    assert_eq!(printed_cell(&doc, "p1c56").0, "292323706");
    assert_eq!(printed_cell(&doc, "p1c57").0, "57");
    assert_eq!(printed_cell(&doc, "p1c112").0, "23600398");
    assert_eq!(printed_cell(&doc, "p1c114").0, "69");
    assert_eq!(printed_cell(&doc, "p2c259").0, "SPECIAL TREATY ALLOWANCE");
    assert_eq!(
        printed_cell(&doc, "p2c261").0,
        "COURT-APPROVED FUNERAL CLAIMS"
    );
}

