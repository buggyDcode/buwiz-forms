//! Band-flow invariant test across every growable/overflow form.
//!
//! For every form the engine flows, the rows go through the form's REAL draft
//! and print map at three sizes: paper capacity (`cap`), `cap + 3`, and
//! `cap + 40`. On the rendered output:
//!   1. every row prints exactly once, in order;
//!   2. the printed total equals the sum of the printed rows;
//!   3. every spawned continuation page carries the filer's TIN, taxpayer name,
//!      and a page number;
//!   4. no page is blank or header-only.
//!
//! Row markers are distinct per row so a duplicated or dropped row cannot
//! hide behind a count.

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

/// One growable-table case: the filled document plus what must hold on it.
struct InvariantCase<'a> {
    label: &'a str,
    slug: &'a str,
    html: &'a str,
    /// Distinct per-row markers in row order.
    row_markers: &'a [String],
    /// The printed total, formatted with decimals as the sheet prints it.
    total_text: &'a str,
}

fn strip_tags(fragment: &str) -> String {
    let mut out = String::with_capacity(fragment.len());
    let mut in_tag = false;
    for ch in fragment.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

/// Visible print content in document order: text nodes plus input `value=`
/// attributes, so comb-split digits read as the contiguous numbers a reader
/// sees. Whitespace is condensed out so character-by-character combs join.
fn rendered_text(html: &str) -> String {
    let mut out = String::new();
    let mut rest = html;
    while !rest.is_empty() {
        if let Some(lt) = rest.find('<') {
            out.push_str(&rest[..lt]);
            rest = &rest[lt..];
            if let Some(gt) = rest.find('>') {
                let tag = &rest[..=gt];
                if let Some(vp) = tag.find("value=\"") {
                    let after = &tag[vp + 7..];
                    if let Some(end) = after.find('\"') {
                        out.push_str(&after[..end]);
                    }
                }
                rest = &rest[gt + 1..];
            } else {
                break;
            }
        } else {
            out.push_str(rest);
            break;
        }
    }
    out.split_whitespace().collect()
}

fn check_invariant(case: &InvariantCase) {
    let label = case.label;
    let slug = case.slug;
    // 1. every row exactly once, in order — counted on the rendered text a
    // reader sees (comb cells joined), never on raw html structure.
    let text = rendered_text(case.html);
    let upper = text.to_uppercase();
    let upper_digits: String = upper.chars().filter(|c| *c != ',' && *c != '.').collect();
    let mut last_pos = 0usize;
    for (i, marker) in case.row_markers.iter().enumerate() {
        let is_numeric = marker
            .chars()
            .all(|c| c.is_ascii_digit() || c == ',' || c == '.');
        let (haystack, needle) = if is_numeric {
            let n: String = marker.chars().filter(|c| c.is_ascii_digit()).collect();
            (upper_digits.as_str(), n)
        } else {
            let n: String = marker.split_whitespace().collect::<String>().to_uppercase();
            (upper.as_str(), n)
        };
        let count = haystack.matches(&needle).count();
        if count != 1 {
            let mut where_found = Vec::new();
            for (fi, f) in case.html.split("<div class=\"page").enumerate().skip(1) {
                let ft = rendered_text(f).to_uppercase();
                let fh = if is_numeric {
                    ft.chars().filter(|c| *c != ',' && *c != '.').collect::<String>()
                } else {
                    ft
                };
                let c = fh.matches(&needle).count();
                if c > 0 {
                    let pos = fh.find(&needle).unwrap();
                    let s = pos.saturating_sub(30);
                    let e = (pos + needle.len() + 30).min(fh.len());
                    where_found.push(format!("p{fi}:{c}({:?})", &fh[s..e]));
                }
            }
            panic!(
                "{label} ({slug}): row {} marker {marker} prints {count} times (must be exactly once; pages={where_found:?})",
                i + 1
            );
        }
        let pos = haystack.find(&needle).unwrap();
        assert!(
            pos >= last_pos || i == 0,
            "{label} ({slug}): row {} prints out of order",
            i + 1
        );
        last_pos = pos;
    }
    // 2. printed total present (digits may print comb-split: compare digit
    // streams, commas stripped)
    if !case.total_text.is_empty() {
        let digits: String = case
            .total_text
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let text_no_commas = text.replace(',', "");
        assert!(
            text_no_commas.contains(&digits.replace('.', "")) || text_no_commas.contains(&digits),
            "{label} ({slug}): printed total {} missing from the rendered pages",
            case.total_text
        );
    }
    // 3/4. page fragments: none blank, continuations carry identity + number
    let frags: Vec<&str> = case.html.split("class=\"page page-").collect();
    for (idx, frag) in frags.iter().enumerate().skip(1) {
        let text = strip_tags(frag);
        let condensed: String = text.split_whitespace().collect();
        assert!(
            condensed.len() > 40,
            "{label} ({slug}): page fragment {idx} is blank or nearly blank ({} chars of text)",
            condensed.len()
        );
    }
    // Continuation fragments (any spawned continuation page: band-flow
    // "continued" page or fallback `page-continuation` sheet) must EACH carry
    // the filer's TIN and taxpayer name and a page number. Check on
    // `rendered_text(frag)` so comb-split names and TINs are read as the
    // contiguous strings a reader sees.
    for (idx, frag) in frags.iter().enumerate().skip(1) {
        let lower = frag.to_ascii_lowercase();
        let is_fallback_cont = frag.starts_with("continuation");
        let is_cont = is_fallback_cont || lower.contains("(continued)");
        if !is_cont {
            continue;
        }
        if is_fallback_cont {
            let open_tag = &frag[..frag.find('>').unwrap_or(0)];
            assert!(
                !open_tag.contains(";height:") && !open_tag.contains("style=\"height:"),
                "{label} ({slug}): continuation page fragment {idx} carries fixed full-page height (spills blank page): {open_tag}"
            );
        }
        let rt = rendered_text(frag).to_uppercase();
        assert!(
            !rt.contains("RETURNPERIOD:—"),
            "{label} ({slug}): continuation page fragment {idx} prints placeholder 'Return Period: —'"
        );
        assert!(
            rt.contains("SANTOS"),
            "{label} ({slug}): continuation page fragment {idx} carries no taxpayer name"
        );
        let has_tin = rt.contains("000-123-453") || rt.contains("000123453");
        assert!(
            has_tin,
            "{label} ({slug}): continuation page fragment {idx} carries no TIN"
        );
        let has_page_num = rt.contains("PAGE");
        assert!(
            has_page_num,
            "{label} ({slug}): continuation page fragment {idx} carries no page number"
        );
    }
}

fn fill(slug: &str, fields: &std::collections::BTreeMap<String, String>) -> String {
    let html = filled_document_with_report(slug, fields, None)
        .unwrap()
        .html;
    // Paper input cells carry their content twice in one tag (`value=` and
    // `data-writer-value=`); strip the mirror attribute so exactly-once row
    // counting counts printed rows, not attributes.
    let mut out = String::with_capacity(html.len());
    let mut rest = html.as_str();
    while let Some(pos) = rest.find("data-writer-value=\"") {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + "data-writer-value=\"".len()..];
        match after.find('"') {
            Some(end) => rest = &after[end + 1..],
            None => {
                rest = "";
                break;
            }
        }
    }
    out.push_str(rest);
    out
}

/// 1702MX Schedule 6 — frozen sheet holds 4 rows (rows 1–3 + row 4; beyond 4,
/// row 4 becomes OTHERS and rows 4+ go into `more_rows`).
#[test]
fn invariant_1702mx_schedule6() {
    use bir_core::forms::form_1702mx::{
        Form1702MXDraft, Form1702MXRegimeAmounts, Form1702MXSpecialDeductionRow, WholePeso,
        WholePesoInput,
    };
    fn wp(n: i64) -> WholePesoInput {
        WholePesoInput::from_amount(WholePeso(n))
    }
    let profile = demo_profile();
    let cap = 4usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1702MXDraft::new_from_profile(&profile, 2025);
        let mut markers = Vec::new();
        let mut expected_total = 0i64;
        if count <= cap {
            for i in 0..count {
                let amt = ((i + 1) * 10_000) as i64;
                expected_total += amt;
                let desc = format!("MX-SC6-ROW-{:02}", i + 1);
                markers.push(desc.clone());
                let row = Form1702MXSpecialDeductionRow {
                    description: desc,
                    legal_basis: format!("RA-ROW-{:02}", i + 1),
                    amounts: Form1702MXRegimeAmounts {
                        exempt: wp(0),
                        special: wp(0),
                        regular: wp(amt),
                        total: wp(amt),
                    },
                };
                d.schedule_6.rows[i] = row;
            }
        } else {
            for i in 0..3 {
                let amt = ((i + 1) * 10_000) as i64;
                expected_total += amt;
                let desc = format!("MX-SC6-BASE-{:02}", i + 1);
                markers.push(desc.clone());
                d.schedule_6.rows[i] = Form1702MXSpecialDeductionRow {
                    description: desc,
                    legal_basis: format!("RA-BASE-{:02}", i + 1),
                    amounts: Form1702MXRegimeAmounts {
                        exempt: wp(0),
                        special: wp(0),
                        regular: wp(amt),
                        total: wp(amt),
                    },
                };
            }
            for i in 1..=(count - 3) {
                let amt = (i * 3_000) as i64;
                expected_total += amt;
                let desc = format!("MX-SC6-MORE-{i:02}");
                markers.push(desc.clone());
                d.schedule_6.more_rows.push(Form1702MXSpecialDeductionRow {
                    description: desc,
                    legal_basis: format!("RA-MORE-{i:02}"),
                    amounts: Form1702MXRegimeAmounts {
                        exempt: wp(0),
                        special: wp(0),
                        regular: wp(amt),
                        total: wp(amt),
                    },
                });
            }
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1702mx-2018c", &fields);
        assert_eq!(
            fields.get("frm1702MX:txtPg6Sc6I5CD").map(String::as_str),
            Some(expected_total.to_string().as_str()),
            "1702MX ({count} rows): print-map total must equal sum"
        );
        check_invariant(&InvariantCase {
            label: &format!("1702MX Schedule 6 ({count} rows)"),
            slug: "1702mx-2018c",
            html: &html,
            row_markers: &markers,
            total_text: &expected_total.to_string(),
        });
    }
}

/// 2200P Schedule 1 — frozen sheet holds 27 rows (rows 1–25 preprinted ATCs +
/// rows 26–27 Others).
#[test]
fn invariant_2200p_schedule1() {
    use bir_core::forms::form_2200p::{Form2200PDraft, Form2200PRow};
    let profile = demo_profile();
    let cap = 27usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form2200PDraft::new_from_profile(&profile, 2025, 3);
        d.day = 13;
        d.schedule.resize(cap, Form2200PRow::default());
        let mut markers = Vec::new();
        let mut sum_due = 0.0f64;
        for idx in 0..cap {
            let r = idx + 1;
            let place = format!("PDEPOT-{r:02}");
            let due = (r as f64) * 100.0;
            sum_due += due;
            d.schedule[idx].place_of_removal = place.clone();
            d.schedule[idx].local_manufactured = Some(100.0);
            d.schedule[idx].basic_excise_tax_due = Some(due);
            if idx >= 25 {
                d.schedule[idx].atc_digits = format!("{r:03}");
                d.schedule[idx].description = format!("FUEL-BASE-{r:02}");
                d.schedule[idx].unit_of_measure = "PER KG".to_string();
                d.schedule[idx].applicable_rate = Some(10.0);
            }
            markers.push(place);
        }
        for r in (cap + 1)..=count {
            let mut row = Form2200PRow::default();
            let place = format!("PDEPOT-{r:02}");
            let due = (r as f64) * 100.0;
            sum_due += due;
            row.atc_digits = format!("{r:03}");
            row.description = format!("FUEL-ADD-{r:02}");
            row.unit_of_measure = "PER KG".to_string();
            row.applicable_rate = Some(10.0);
            row.place_of_removal = place.clone();
            row.local_manufactured = Some(100.0);
            row.basic_excise_tax_due = Some(due);
            markers.push(place);
            d.schedule.push(row);
        }
        d.recompute();
        assert!((d.schedule_total - sum_due).abs() < 0.01);
        let fields = d.to_print_field_map();
        let html = fill("2200p-2020", &fields);
        let expected_total = fields
            .get("frm2200P:txtSched1TotalTaxDue")
            .cloned()
            .unwrap_or_default();
        assert!(!expected_total.is_empty(), "2200P total must be present");
        check_invariant(&InvariantCase {
            label: &format!("2200P Schedule 1 ({count} rows)"),
            slug: "2200p-2020",
            html: &html,
            row_markers: &markers,
            total_text: &expected_total,
        });
    }
}

/// 2200A Schedule 1 Others — frozen sheet holds 3 rows (`xa_others1..=3`).
#[test]
fn invariant_2200a_others() {
    use bir_core::forms::form_2200a::{Form2200AAmounts, Form2200ADraft, Form2200AOtherRow};
    let profile = demo_profile();
    let cap = 3usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form2200ADraft::new_from_profile(&profile, 2025, 4);
        d.day = 15;
        let mut markers = Vec::new();
        let mut sum_due = 0.0f64;
        for i in 1..=count {
            let desc = format!("SPIRIT ITEM {i:02}");
            let due = (i as f64) * 500.0;
            sum_due += due;
            markers.push(desc.clone());
            d.others.push(Form2200AOtherRow {
                atc_digits: format!("{:03}", 100 + i),
                description: desc,
                bracket: "PROOF".into(),
                rate: "50.00".into(),
                amounts: Form2200AAmounts {
                    export_exempt: 0.0,
                    taxable: (i as f64) * 10.0,
                    tax_due: due,
                },
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("2200a-2020", &fields);
        check_invariant(&InvariantCase {
            label: &format!("2200A Others ({count} rows)"),
            slug: "2200a-2020",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum_due:.2}"),
        });
    }
}

/// 1702Q Schedule 4 Other Credits — frozen sheet holds 2 rows (Items 6a, 6b).
#[test]
fn invariant_1702q_schedule4() {
    use bir_core::forms::form_1702q::{Form1702qDraft, Form1702qOtherCredit};
    let profile = demo_profile();
    let cap = 2usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1702qDraft::new_from_profile(&profile, 2025, 1);
        d.atc = "IC010_25%".to_string();
        d.other_credits.clear();
        let mut markers = Vec::new();
        let mut sum = 0.0f64;
        for i in 1..=count {
            let amt = (i as f64) * 1_000.0;
            sum += amt;
            let desc = format!("OTHER CREDIT {i:02}");
            markers.push(desc.clone());
            d.other_credits.push(Form1702qOtherCredit {
                description: desc,
                amount: amt,
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1702q-2018", &fields);
        assert_eq!(
            fields
                .get("frm1702q:Sched4:txtTax7")
                .map(|s| s.replace(',', "")),
            Some(format!("{sum:.2}")),
            "1702Q ({count} rows): total must equal sum of all rows"
        );
        check_invariant(&InvariantCase {
            label: &format!("1702Q Schedule 4 ({count} rows)"),
            slug: "1702q-2018",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum:.2}"),
        });
        if count == cap + 3 {
            let frags: Vec<&str> = html.split("class=\"page page-").collect();
            assert!(
                frags.len() >= 4,
                "1702Q at {count} rows must spawn a continuation page"
            );
            let cont = frags[3..].concat();
            assert!(!cont.contains("OTHER CREDIT 01") && !cont.contains("OTHER CREDIT 02"));
            for n in 3..=count {
                assert!(
                    cont.contains(&format!("<td class=\"num\">{n}</td>"))
                        || cont.contains(&format!(">{n}</td>")),
                    "1702Q continuation must number overflow row {n}"
                );
            }
        }
    }
}

/// 1601C Schedule 1 — frozen sheet holds 3 rows (rows 1–3).
#[test]
fn invariant_1601c_schedule1() {
    use bir_core::forms::form_1601c::{Form1601CDraft, Form1601CSchedule1Row};
    let profile = demo_profile();
    let cap = 3usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1601CDraft::new_from_profile(&profile, 2025, 1);
        d.schedule_1.clear();
        let mut markers = Vec::new();
        for i in 1..=count {
            markers.push(format!("BDO{i:02}"));
            let mut row = Form1601CSchedule1Row {
                previous_month: format!("{:02}/2024", ((i - 1) % 12) + 1),
                date_paid: format!("{:02}/10/2024", ((i - 1) % 12) + 1),
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
        let html = fill("1601c-2018", &fields);
        let expected_sum = count * 500;
        assert_eq!(
            fields
                .get("frm1601c:sched1:txtTotal1")
                .map(|s| s.replace(',', "")),
            Some(format!("{expected_sum}.00")),
            "1601C ({count} rows): total must equal sum of all rows"
        );
        check_invariant(&InvariantCase {
            label: &format!("1601C Schedule 1 ({count} rows)"),
            slug: "1601c-2018",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{expected_sum}.00"),
        });
    }
}

/// 1707A Schedule 1 (Gains) — frozen sheet holds 4 rows (rows 1–4; beyond 4,
/// row 4 becomes OTHERS and rows 4+ print on the continuation sheet).
#[test]
fn invariant_1707a_schedule1() {
    use bir_core::forms::form_1707a::{Form1707AAtc, Form1707ADraft, Form1707ARow};
    let profile = demo_profile();
    let cap = 4usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1707ADraft::new_from_profile(&profile, 2025, 1);
        d.set_atc(Form1707AAtc::Individual);
        d.year_end_year = 2023;
        d.number_of_attached_sheets = 1;
        let mut markers = Vec::new();
        let mut sum_gain = 0.0f64;
        let mut sum_tax = 0.0f64;
        for k in 1..=count {
            markers.push(format!("GAINCO-{k:02}"));
            let sp = (k as f64) * 2_000.0;
            let cost = (k as f64) * 1_000.0;
            let tp = (k as f64) * 100.0;
            sum_gain += sp - cost;
            sum_tax += tp;
            d.gains.push(Form1707ARow {
                date: "02/10/2023".into(),
                corporation: format!("GAINCO-{k:02}"),
                selling_price: sp,
                cost,
                gain_or_loss: 0.0,
                tax_paid: tp,
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1707a-2021", &fields);
        // Official paper (BIR Form 1707-A April 2021 ENCS Page 2 PDF and
        // html-frozen/1707a-2021/writer-cells.json): Schedule 1 Row 5 TOTAL
        // prints ONLY Column (E) Capital Gains (`p2c57`) and Column (F) Tax
        // Paid (`p2c58`).
        assert_eq!(
            fields
                .get("frm1707Av2021:txtS1I20TotalCapitalGains")
                .map(|s| s.replace(',', "")),
            Some(format!("{sum_gain:.2}")),
            "1707A Sch 1 ({count} rows): Column (E) total must equal sum of gains"
        );
        assert_eq!(
            fields
                .get("frm1707Av2021:txtS1I20TotalTaxPaid")
                .map(|s| s.replace(',', "")),
            Some(format!("{sum_tax:.2}")),
            "1707A Sch 1 ({count} rows): Column (F) total must equal sum of tax paid"
        );
        check_invariant(&InvariantCase {
            label: &format!("1707A Schedule 1 ({count} rows)"),
            slug: "1707a-2021",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum_gain:.2}"),
        });
    }
}

/// 1707A Schedule 2 (Losses) — frozen sheet holds 4 rows (rows 1–4).
#[test]
fn invariant_1707a_schedule2() {
    use bir_core::forms::form_1707a::{Form1707AAtc, Form1707ADraft, Form1707ARow};
    let profile = demo_profile();
    let cap = 4usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1707ADraft::new_from_profile(&profile, 2025, 1);
        d.set_atc(Form1707AAtc::Individual);
        d.year_end_year = 2023;
        let mut markers = Vec::new();
        let mut sum_loss = 0.0f64;
        for k in 1..=count {
            markers.push(format!("LOSSCO-{k:02}"));
            let sp = (k as f64) * 1_000.0;
            let cost = (k as f64) * 2_000.0;
            sum_loss += cost - sp;
            d.losses.push(Form1707ARow {
                date: "03/15/2023".into(),
                corporation: format!("LOSSCO-{k:02}"),
                selling_price: sp,
                cost,
                gain_or_loss: 0.0,
                tax_paid: 0.0,
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1707a-2021", &fields);
        // Official paper (BIR Form 1707-A April 2021 ENCS Page 2 PDF and
        // html-frozen/1707a-2021/writer-cells.json): Schedule 2 Row 5 TOTAL
        // prints ONLY Column (E) Capital Loss (`p2c105`).
        check_invariant(&InvariantCase {
            label: &format!("1707A Schedule 2 ({count} rows)"),
            slug: "1707a-2021",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum_loss:.2}"),
        });
    }
}

/// 1707 Schedule 2 (Shares) & Schedule 3 (Expenses) — frozen sheet holds 4
/// rows each (rows A–D).
#[test]
fn invariant_1707_schedules() {
    use bir_core::forms::form_1707::{Form1707Draft, Form1707Expense, Form1707Shares};
    let profile = demo_profile();
    let cap = 4usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1707Draft::new_from_profile(&profile, 2025, 1);
        d.shares.clear();
        d.expenses.clear();
        let mut share_markers = Vec::new();
        let mut exp_markers = Vec::new();
        let mut sum_sp = 0.0f64;
        let mut sum_exp = 0.0f64;
        for k in 1..=count {
            let corp = format!("SHARE-CORP-{k:02}");
            let sp = (k as f64) * 1_000.0;
            sum_sp += sp;
            share_markers.push(corp.clone());
            d.shares.push(Form1707Shares {
                corporation: corp,
                number_of_shares: Some(100.0),
                certificate_number: format!("CERT-{k:02}"),
                selling_price: sp,
            });
        }
        for k in 1..=count {
            let part = format!("EXP-ITEM-{k:02}");
            let amt = (k as f64) * 500.0;
            sum_exp += amt;
            exp_markers.push(part.clone());
            d.expenses.push(Form1707Expense {
                particulars: part,
                amount: amt,
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1707-2021", &fields);
        check_invariant(&InvariantCase {
            label: &format!("1707 Schedule 2 ({count} rows)"),
            slug: "1707-2021",
            html: &html,
            row_markers: &share_markers,
            total_text: &format!("{sum_sp:.2}"),
        });
        check_invariant(&InvariantCase {
            label: &format!("1707 Schedule 3 ({count} rows)"),
            slug: "1707-2021",
            html: &html,
            row_markers: &exp_markers,
            total_text: &format!("{sum_exp:.2}"),
        });
    }
}

/// 1700 Schedule 1 (Employers) — frozen sheet holds 4 rows (Items 1–4).
#[test]
fn invariant_1700_schedule1() {
    use bir_core::forms::form_1700::{Form1700Draft, Form1700Employer, Form1700TaxpayerType};
    let profile = demo_profile();
    let cap = 4usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1700Draft::new_from_profile(&profile, 2025);
        d.taxpayer_type = Form1700TaxpayerType::Employee;
        d.employers.clear();
        let mut markers = Vec::new();
        let mut sum_reg = 0.0f64;
        for k in 1..=count {
            let name = format!("EMPLOYER-{k:02}");
            let reg = (k as f64) * 10_000.0;
            sum_reg += reg;
            markers.push(name.clone());
            d.employers.push(Form1700Employer {
                for_spouse: false,
                name,
                name2: String::new(),
                tin: format!("1112223{:02}00000", k % 100),
                regular: reg,
                flat: 0.0,
                withheld: (k as f64) * 1_000.0,
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1700-2018", &fields);
        check_invariant(&InvariantCase {
            label: &format!("1700 Schedule 1 ({count} rows)"),
            slug: "1700-2018",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum_reg:.0}"),
        });
    }
}

/// 2000OT Schedule 1.A (Real Property) & 1.B (Shares) — frozen sheet holds 3
/// rows each (`0..=2`).
#[test]
fn invariant_2000ot_schedules() {
    use bir_core::forms::form_2000ot::{Form2000OTDraft, Form2000OTPropertyRow, Form2000OTShareRow};
    let profile = demo_profile();
    let cap = 3usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form2000OTDraft::new_from_profile(&profile, 2025);
        d.transaction_date = "03/15/2025".into();
        d.properties.clear();
        d.shares.clear();
        let mut markers = Vec::new();
        let mut sum_fmv = 0.0f64;
        for k in 1..=count {
            let tct = format!("TCT-{k:02}");
            let fmv = (k as f64) * 100_000.0;
            sum_fmv += fmv;
            markers.push(tct.clone());
            d.properties.push(Form2000OTPropertyRow {
                title_number: tct,
                tax_declaration_number: format!("TD-{k:02}"),
                location: format!("LOC-{k:02}"),
                lot: "LOT".into(),
                classification: "RR".into(),
                area: "100".into(),
                fmv_per_td: fmv,
                fmv_zonal: fmv,
                fmv,
            });
        }
        for k in 1..=count {
            let corp = format!("OT-CORP-{k:02}");
            markers.push(corp.clone());
            d.shares.push(Form2000OTShareRow {
                corporation: corp,
                certificate_number: format!("SC-{k:02}"),
                shares_sold: "100".into(),
                par_value: (k as f64) * 1_000.0,
                dst_paid_on_issue: (k as f64) * 100.0,
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("2000-ot-2018", &fields);
        check_invariant(&InvariantCase {
            label: &format!("2000OT Schedules 1.A & 1.B ({count} rows)"),
            slug: "2000-ot-2018",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum_fmv:.2}"),
        });
    }
}

/// 1702EX Schedule 1 Item 17 Others — frozen sheet holds 6 rows (`17d..=17i`).
#[test]
fn invariant_1702ex_schedule1_others() {
    use bir_core::forms::form_1702ex::{Form1702ExDraft, Form1702ExRow};
    let profile = demo_profile();
    let cap = 6usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1702ExDraft::new_from_profile(&profile, 2025);
        d.other_deductions.clear();
        let mut markers = Vec::new();
        let mut sum = 0.0f64;
        for k in 1..=count {
            let desc = format!("EX-DED-{k:02}");
            let amt = (k as f64) * 1_000.0;
            sum += amt;
            markers.push(desc.clone());
            d.other_deductions.push(Form1702ExRow {
                description: desc,
                amount: amt,
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1702ex-2018", &fields);
        check_invariant(&InvariantCase {
            label: &format!("1702EX Schedule 1 Others ({count} rows)"),
            slug: "1702ex-2018",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum:.0}"),
        });
    }
}

/// 2551Q Schedule 1 — frozen sheet holds 6 rows (rows 1–6).
#[test]
fn invariant_2551q_schedule1() {
    use bir_core::forms::form_2551q::{Form2551QDraft, Schedule1Row};
    let profile = demo_profile();
    let cap = 6usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form2551QDraft::new_from_profile(&profile, 2025, 1);
        d.schedule_1.clear();
        let mut markers = Vec::new();
        let mut sum_due = 0.0f64;
        for k in 1..=count {
            let mut row = Schedule1Row::default_pt010();
            let amt = 10_000.0 + (k as f64) * 113.0;
            row.taxable_amount = amt;
            row.recompute();
            sum_due += row.tax_due;
            markers.push(format!("{amt:.2}"));
            d.schedule_1.push(row);
        }
        d.recompute(None);
        let fields = d.to_print_field_map();
        let html = fill("2551q-2018", &fields);
        check_invariant(&InvariantCase {
            label: &format!("2551Q Schedule 1 ({count} rows)"),
            slug: "2551q-2018",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum_due:.2}"),
        });
    }
}

/// 2200M Schedule 1 — frozen sheet holds 10 rows (rows 1–8 preprinted + rows
/// 9–10 XM060 others).
#[test]
fn invariant_2200m_schedule1() {
    use bir_core::forms::form_2200m::{Form2200MDraft, Form2200MRow};
    let profile = demo_profile();
    let cap = 10usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form2200MDraft::new_from_profile(&profile, 2025, 4);
        d.day = 15;
        d.schedule.resize(cap, Form2200MRow::default());
        let mut markers = Vec::new();
        for idx in 0..cap {
            let r = idx + 1;
            let place = format!("MSITE-{r:02}");
            d.schedule[idx].place_of_removal = place.clone();
            d.schedule[idx].local_value_taxable = Some(10_000.0);
            if idx >= 8 {
                d.schedule[idx].description = format!("MINERAL-{r:02}");
                d.schedule[idx].local_rate = Some(4.0);
            }
            markers.push(place);
        }
        for r in (cap + 1)..=count {
            let mut row = Form2200MRow::default();
            let desc = format!("MINERAL-{r:02}");
            row.description = desc.clone();
            row.place_of_removal = format!("MSITE-{r:02}");
            row.local_value_taxable = Some(10_000.0);
            row.local_rate = Some(4.0);
            markers.push(desc);
            d.schedule.push(row);
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("2200m-2018", &fields);
        let expected_total = fields
            .get("frm2200M:txtSched1_TotalTaxDue")
            .cloned()
            .unwrap_or_else(|| format!("{:.2}", d.schedule_total));
        check_invariant(&InvariantCase {
            label: &format!("2200M Schedule 1 ({count} rows)"),
            slug: "2200m-2018",
            html: &html,
            row_markers: &markers,
            total_text: &expected_total,
        });
    }
}

/// 1801 Schedule 1 (Real Properties) — frozen sheet holds 2 rows (`1..=2`).
#[test]
fn invariant_1801_schedule1() {
    use bir_core::forms::form_1801::{Form1801Draft, Form1801RealProperty, Form1801Split};
    let profile = demo_profile();
    let cap = 2usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1801Draft::new_from_profile(&profile, 2025, 1);
        d.real_properties.clear();
        let mut markers = Vec::new();
        let mut sum_exc = 0.0f64;
        for k in 1..=count {
            let oct = format!("EST-TCT-{k:02}");
            let val = (k as f64) * 50_000.0;
            sum_exc += val;
            markers.push(oct.clone());
            d.real_properties.push(Form1801RealProperty {
                title_number: oct,
                tax_declaration_number: format!("EST-TD-{k:02}"),
                location: format!("EST-LOC-{k:02}"),
                lot_or_improvement: "LOT".into(),
                area: "200".into(),
                classification: "RR".into(),
                fmv_per_tax_declaration: format!("{val:.2}"),
                fmv_per_zonal_value: format!("{val:.2}"),
                value: Form1801Split {
                    exclusive: val,
                    conjugal: 0.0,
                },
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1801-2018", &fields);
        check_invariant(&InvariantCase {
            label: &format!("1801 Schedule 1 ({count} rows)"),
            slug: "1801-2018",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum_exc:.2}"),
        });
    }
}

/// 1600PT Part II (ATC rows) — frozen sheet holds 5 rows (Items 13–17).
#[test]
fn invariant_1600pt_part2() {
    use bir_core::forms::form_1600pt::{Form1600PtAtcRow, Form1600PtDraft};
    let profile = demo_profile();
    let cap = 5usize;
    for &count in &[cap, cap + 3, cap + 40] {
        let mut d = Form1600PtDraft::new_from_profile(&profile, 2025, 1);
        d.taxes_withheld = true;
        d.atc_rows.clear();
        let mut markers = Vec::new();
        let mut sum_wh = 0.0f64;
        for k in 1..=count {
            let base = 10_000.0 + (k as f64) * 113.0;
            let wh = ((base * 0.03) * 100.0).round() / 100.0;
            sum_wh += wh;
            markers.push(format!("{base:.2}"));
            d.atc_rows.push(Form1600PtAtcRow {
                atc_code: "WB080".into(),
                rate_text: "3.0".into(),
                tax_base: base,
                tax_withheld: wh,
            });
        }
        d.recompute();
        let fields = d.to_print_field_map();
        let html = fill("1600-pt-2018", &fields);
        check_invariant(&InvariantCase {
            label: &format!("1600PT Part II ({count} rows)"),
            slug: "1600-pt-2018",
            html: &html,
            row_markers: &markers,
            total_text: &format!("{sum_wh:.2}"),
        });
    }
}
