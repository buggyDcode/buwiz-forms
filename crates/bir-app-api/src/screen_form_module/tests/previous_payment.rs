//! Previous-payment amount boxes stay open on 0619E, 0619F and 2550M.

use super::*;

fn save_reopen_and_yes_no(
    code: &str,
    period: i64,
    amount_key: &str,
    dependent_key: &str,
    sign: f64,
    yes: &str,
    no: &str,
    draft_field: &str,
) {
    setup();
    let opened = open(code, period);
    let before = amount(&opened["values"], dependent_key);
    let filled = fill(code, &opened["draft"], json!({ amount_key: "500" }));
    assert_eq!(filled["draft"][draft_field], 500.0, "{code}");
    assert_eq!(filled["values"][amount_key], "500.00", "{code}");
    assert_eq!(
        amount(&filled["values"], dependent_key),
        before + sign * 500.0,
        "{code}"
    );
    let saved = save(code, &filled["draft"]);
    let reopened = open(code, period);
    assert_eq!(reopened["draft"]["id"], saved["draft"]["id"], "{code}");
    assert_eq!(reopened["draft"][draft_field], 500.0, "{code}");
    assert_eq!(reopened["values"][amount_key], "500.00", "{code}");

    let yesed = fill(code, &reopened["draft"], json!({ yes: "true" }));
    let noed = fill(code, &yesed["draft"], json!({ no: "true", yes: "false" }));
    assert_eq!(noed["draft"][draft_field], 500.0, "{code} Yes then No");
    assert_eq!(noed["values"][amount_key], "500.00", "{code} Yes then No");
}

#[test]
fn previous_payment_survives_save_reopen_and_amended_yes_then_no() {
    save_reopen_and_yes_no(
        "0619E",
        4,
        "frm0619E:txtTax15",
        "frm0619E:txtTax16",
        -1.0,
        "frm0619E:optAmend:Y",
        "frm0619E:optAmend:N",
        "item_15_amount_remitted_previously",
    );
    save_reopen_and_yes_no(
        "0619F",
        5,
        "frm0619F:txtTax16",
        "frm0619F:txtTax17",
        -1.0,
        "frm0619F:optAmend:Y",
        "frm0619F:optAmend:N",
        "item_16_remitted_previously",
    );
    save_reopen_and_yes_no(
        "2550M",
        6,
        "frm2550m:txtTax23D",
        "frm2550m:txtTax23G",
        1.0,
        "frm2550m:OptAmendedYN1",
        "frm2550m:OptAmendedYN2",
        "vat_paid_previous",
    );
}
