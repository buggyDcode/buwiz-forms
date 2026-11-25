use bir_core::forms::form_1604c::Form1604cDraft;
use bir_core::forms::form_1604e::Form1604eDraft;
use bir_core::forms::form_1701::{
    Form1701CivilStatus, Form1701Draft, Form1701EmployerRow, Form1701Party,
};
use bir_core::forms::form_1800::{
    Form1800Donee, Form1800Draft, Form1800PersonalProperty, Form1800RealProperty,
};
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

#[test]
fn form_1800_prints_item_14_total_net_gifts() {
    let profile = demo_profile();
    let mut d = Form1800Draft::new_from_profile(&profile, 2025, 1);
    d.donation_month = 3;
    d.donation_day = 15;
    d.donation_year = 2025;
    d.donees.push(Form1800Donee {
        name: "SANTOS, PEDRO".into(),
        tin: "00098765400000".into(),
    });
    d.personal_properties.push(Form1800PersonalProperty {
        particulars: "CASH GIFT SAVINGS".into(),
        fair_market_value: 123_456_789.45,
    });
    d.real_properties.push(Form1800RealProperty {
        title_number: "TCT-9001".into(),
        tax_declaration_number: "TD-8001".into(),
        location: "MAKATI CITY".into(),
        lot_or_improvement: "LOT".into(),
        classification: "RESIDENTIAL".into(),
        area: "300 SQM".into(),
        fmv_per_tax_declaration: "85000000.00".into(),
        fmv_per_zonal_value: "110222333.50".into(),
        fair_market_value: 110_222_333.50,
    });
    d.compute();

    let map = d.print_field_map();
    let doc = filled_document("1800-2018", &map).expect("1800-2018 render");

    assert_eq!(printed_cell(&doc, "p1c62").0, "233429122");
    assert_eq!(printed_cell(&doc, "p1c64").0, "95");
    assert_eq!(printed_cell(&doc, "p1c70").0, "14005747");
    assert_eq!(printed_cell(&doc, "p1c72").0, "38");
}

#[test]
fn form_1701_prints_schedule1_employer_names_and_tins() {
    let profile = demo_profile();
    let mut d = Form1701Draft::new_from_profile(&profile, 2025, 12);
    d.civil_status = Some(Form1701CivilStatus::Married);
    d.spouse_has_income = Some(true);
    d.spouse.name = "SANTOS, JUANITO".into();
    d.spouse.tin = "00045678900000".into();
    d.spouse_rdo_code = "044".into();
    d.employers[0] = Form1701EmployerRow {
        owner: Some(Form1701Party::Taxpayer),
        employer_name: "ACME PACIFIC HOLDINGS CORPORATION PHILIPPINES".into(),
        employer_tin: "00077788800001".into(),
        compensation_income: Some(125_456_789.45),
        tax_withheld: Some(31_234_567.89),
    };
    d.employers[1] = Form1701EmployerRow {
        owner: Some(Form1701Party::Spouse),
        employer_name: "MANILA GLOBAL ENTERPRISES INCORPORATED".into(),
        employer_tin: "00099911100002".into(),
        compensation_income: Some(110_222_333.50),
        tax_withheld: Some(25_111_222.25),
    };
    d.compute();

    let map = d.print_field_map();
    let doc = filled_document("1701-2018", &map).expect("1701-2018 render");

    assert_eq!(printed_cell(&doc, "p2c54").0, "X");
    assert_eq!(
        printed_cell(&doc, "p2c55").0,
        "ACME PACIFIC HOLDINGS CORPORATION P"
    );
    assert_eq!(printed_cell(&doc, "p2c56").0, "HILIPPINES");
    assert_eq!(printed_cell(&doc, "p2c58").0, "00077788800001");

    assert_eq!(printed_cell(&doc, "p2c59").0, "X");
    assert_eq!(
        printed_cell(&doc, "p2c60").0,
        "MANILA GLOBAL ENTERPRISES INCORPORA"
    );
    assert_eq!(printed_cell(&doc, "p2c61").0, "TED");
    assert_eq!(printed_cell(&doc, "p2c63").0, "00099911100002");
}

#[test]
fn form_1604c_prints_two_digit_year_after_preprinted_20() {
    let profile = demo_profile();
    let d = Form1604cDraft::new_from_profile(&profile, 2025);
    assert_eq!(
        d.to_bir_field_map().get("frm1604c:txtYear").map(String::as_str),
        Some("2025")
    );
    let map = d.to_print_field_map();
    assert_eq!(map.get("frm1604c:txtYear").map(String::as_str), Some("25"));
    let doc = filled_document("1604c-2018", &map).expect("1604c-2018 render");
    assert_eq!(printed_cell(&doc, "p1c4").0, "25");
}

#[test]
fn form_1604e_prints_two_digit_year_after_preprinted_20() {
    let profile = demo_profile();
    let d = Form1604eDraft::new_from_profile(&profile, 2025);
    assert_eq!(
        d.to_bir_field_map().get("frm1604e:txtYear").map(String::as_str),
        Some("2025")
    );
    let map = d.to_print_field_map();
    assert_eq!(map.get("frm1604e:txtYear").map(String::as_str), Some("25"));
    let doc = filled_document("1604e-2018", &map).expect("1604e-2018 render");
    assert_eq!(printed_cell(&doc, "p1c4").0, "25");
}
