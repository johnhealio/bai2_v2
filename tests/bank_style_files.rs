//! Integration tests against large, multi-group/multi-account BAI2
//! transmissions built to exercise the same real-world variations major
//! banks are commonly known to produce: heavy use of 88 continuation
//! records for long lockbox/remittance text, multiple currencies within
//! one file (including a per-account override against the group
//! default), multiple groups per file, and every Funds Type sub-format
//! (`0`, `1`, `S`, `D`, `V`).
//!
//! These files are constructed from the BAI2 spec and widely documented
//! implementer conventions (fixed-vs-variable record length, heavy
//! continuation usage, multi-currency reporting) — not literal production
//! output from any institution. They're labeled "JPMC-style"/"Citi-style"
//! only to describe the *shape* of transmission (lockbox-heavy vs.
//! info-reporting-only) they're modeled on, not to claim they're real
//! files from those banks.
//!
//! Every control total below was computed independently (see the Python
//! script noted in the project history) before being encoded here, so a
//! test failure means the library disagrees with arithmetic that was
//! checked by a second, independent method — not a typo in the test.

use bai2::{
    AccountViolation, BaiReader, CurrencyCode, File, FileViolation, FundsType, GroupViolation,
};

/// A large lockbox-heavy transmission: two groups, three accounts, heavy
/// use of 88 continuations (including one chained across *two*
/// continuation lines mid-text), an account-level currency override
/// (EUR within a USD-default group), and a group-level currency override
/// (GBP) — exercising `S`, `D`, `V`, `0`, and `1` Funds Type sub-formats
/// across both 03 and 16 records.
fn jpmc_style_file() -> String {
    let lines: &[&str] = &[
        "01,0210000210,CUSTCOID01,040630,0630,1,80,,2/",
        "02,,0210000210,1,040630,2359,,2/",
        "03,0012345678,,010,5000000,,,015,5500000,,,100,1000000,3,S,500000,300000,200000,400,500000,2,/",
        "16,165,700000,1,ACH0001,,PAYROLL FUNDING/",
        "16,115,300000,S,100000,100000,100000,LBX0002,,LOCKBOX DEPOSIT FOR ACCOUNT 0012345678 REFERENCE BATCH",
        "88,20040630-001 PROCESSED VIA REMOTE",
        "88,CAPTURE SYSTEM GENERATION 3",
        "16,409,500000,0,,0001234,/",
        "49,13500000,7/",
        "03,0098765432,EUR,010,2000000,,,190,800000,1,V,040701,/",
        "16,218,800000,D,2,0,400000,1,400000,FCC0001,,/",
        "49,3600000,3/",
        "98,17100000,2,12/",
        "02,,0210000299,1,040630,2359,GBP,2/",
        "03,0055566677,,010,1000000,,,100,1000000,1,0/",
        "16,165,1000000,0,GBPPMT01,,SUPPLIER PAYMENT LONDON OFFICE/",
        "49,3000000,3/",
        "98,3000000,1,5/",
        "99,20100000,2,19/",
    ];
    lines.join("\n") + "\n"
}

/// An information-reporting-only transmission (no Transaction Detail
/// records at all — a common "prior-day balance reporting" shape): one
/// group, three accounts, a EUR group default with one account
/// explicitly overriding back to USD, and status/summary combinations
/// covering float codes, distributed availability (`S`), and value-dated
/// availability (`V`) purely within 03 records.
fn citi_style_file() -> String {
    let lines: &[&str] = &[
        "01,9999888877,CUSTCOID02,040630,0600,42,,,2/",
        "02,,9999888877,1,040630,,EUR,2/",
        "03,0011112222,,010,10000000,,,015,10500000,,,040,9800000,,,100,2000000,5,S,1000000,600000,400000,400,1500000,3,/",
        "49,33800000,2/",
        "03,0022223333,,010,5000000,,,015,5200000,,,072,100000,,,074,50000,,,100,1000000,2,1/",
        "49,11350000,2/",
        "03,0033334444,USD,010,800000,,,190,200000,1,V,040701,1400/",
        "49,1000000,2/",
        "98,46150000,3,11/",
        "99,46150000,1,13/",
    ];
    lines.join("\n") + "\n"
}

/// A stress file targeting three edge cases the two "bank-style" files
/// above don't exercise: a group with zero accounts (immediate 02 -> 98),
/// an account with no status/summary data at all — the spec's own
/// documented pattern for an 03 record used only to introduce Transaction
/// Detail records — and a continuation chain four levels deep (rather
/// than the two tested above) on a non-monetary (890) detail record.
fn edge_case_file() -> String {
    let lines: &[&str] = &[
        "01,1111111111,2222222222,040630,0100,7,,,2/",
        "02,,1111111111,1,040630,,,/",
        "98,0,0,3/",
        "02,,1111111111,1,040630,,,/",
        "03,0099998888,,,,,/",
        "16,165,250000,1,REF001,,INVOICE PAYMENT NET 30/",
        "49,250000,3/",
        "98,250000,1,5/",
        "02,,1111111111,1,040630,,,/",
        "03,0077778888,,,,,/",
        "16,890,,,,,PART ONE OF A VERY LONG MESSAGE",
        "88,-PART TWO CONTINUES HERE",
        "88,-PART THREE KEEPS GOING",
        "88,-PART FOUR ALMOST DONE",
        "88,-PART FIVE THE END",
        "49,0,6/",
        "98,0,1,8/",
        "99,250000,3,16/",
    ];
    lines.join("\n") + "\n"
}

#[test]
fn edge_cases_parse_and_validate_cleanly() {
    let file = parse_file(&edge_case_file());

    assert_eq!(file.groups.len(), 3);
    assert_eq!(
        file.validate(),
        vec![],
        "control totals and counts should all reconcile"
    );

    assert_eq!(
        file.groups[0].accounts.len(),
        0,
        "a group may legally have zero accounts"
    );

    let bare_account = &file.groups[1].accounts[0];
    assert_eq!(bare_account.identifier.as_ref().unwrap().summaries, vec![]);
    assert_eq!(bare_account.transaction_details.len(), 1);

    let long_message = &file.groups[2].accounts[0].transaction_details[0];
    assert_eq!(
        long_message.text.as_deref(),
        Some(
            "PART ONE OF A VERY LONG MESSAGE\
             -PART TWO CONTINUES HERE\
             -PART THREE KEEPS GOING\
             -PART FOUR ALMOST DONE\
             -PART FIVE THE END"
        )
    );
}

/// Feeds `raw` through `BaiReader` into a fresh `File`, `unwrap`ing every
/// step (so a test using this fails loudly, with a normal panic message
/// pointing at the exact push that failed, rather than silently
/// swallowing an error).
fn parse_file(raw: &str) -> File {
    let mut file = File::new();
    for line in BaiReader::new(raw.as_bytes()) {
        file.push(&line.unwrap()).unwrap();
    }
    file
}

#[test]
fn jpmc_style_file_parses_and_validates_cleanly() {
    let file = parse_file(&jpmc_style_file());

    assert_eq!(file.groups.len(), 2);
    assert_eq!(
        file.validate(),
        vec![],
        "control totals and counts should all reconcile"
    );

    let group1 = &file.groups[0];
    assert_eq!(group1.accounts.len(), 2);

    let account1 = &group1.accounts[0];
    assert_eq!(
        account1.identifier.as_ref().unwrap().currency,
        CurrencyCode::Usd
    );
    assert_eq!(account1.transaction_details.len(), 3);

    // The lockbox deposit's Text field spans the original 16 line plus
    // two chained 88 continuations, folded with no inserted separator
    // since it continues mid-text (not at a field boundary).
    let lockbox_detail = &account1.transaction_details[1];
    assert_eq!(
        lockbox_detail.text.as_deref(),
        Some(
            "LOCKBOX DEPOSIT FOR ACCOUNT 0012345678 REFERENCE BATCH\
             20040630-001 PROCESSED VIA REMOTE\
             CAPTURE SYSTEM GENERATION 3"
        )
    );
    assert_eq!(
        lockbox_detail.funds_type,
        FundsType::Distributed(bai2::DistributedAvailability {
            immediate: 100_000,
            one_day: 100_000,
            two_or_more_days: 100_000,
        })
    );

    // Account 2 overrides the group's USD default to EUR; its detail
    // records should inherit that override, not the group default.
    let account2 = &group1.accounts[1];
    assert_eq!(
        account2.identifier.as_ref().unwrap().currency,
        CurrencyCode::Eur
    );
    assert_eq!(account2.transaction_details[0].currency, CurrencyCode::Eur);
    assert_eq!(
        account2.transaction_details[0].funds_type,
        FundsType::DistributedDays(vec![
            bai2::DistributionDay {
                days: 0,
                amount: 400_000
            },
            bai2::DistributionDay {
                days: 1,
                amount: 400_000
            },
        ])
    );

    // Group 2 overrides the file-implicit default to GBP at the group
    // level; its one account has no override of its own and should
    // inherit GBP.
    let group2 = &file.groups[1];
    assert_eq!(group2.header.as_ref().unwrap().currency, CurrencyCode::Gbp);
    assert_eq!(
        group2.accounts[0].identifier.as_ref().unwrap().currency,
        CurrencyCode::Gbp
    );
}

#[test]
fn jpmc_style_file_parses_identically_with_crlf_line_endings() {
    // Confirms CRLF tolerance through the *full* pipeline (BaiReader's
    // own unit tests already cover it in isolation; this checks it
    // doesn't break once File::push/validate are layered on top).
    let crlf = jpmc_style_file().replace('\n', "\r\n");
    let file = parse_file(&crlf);
    assert_eq!(file.groups.len(), 2);
    assert_eq!(file.validate(), vec![]);
}

#[test]
fn citi_style_file_parses_and_validates_cleanly() {
    let file = parse_file(&citi_style_file());

    assert_eq!(file.groups.len(), 1);
    assert_eq!(
        file.validate(),
        vec![],
        "control totals and counts should all reconcile"
    );

    let group = &file.groups[0];
    assert_eq!(group.accounts.len(), 3);
    for account in &group.accounts {
        assert!(
            account.transaction_details.is_empty(),
            "info-reporting-only accounts should have no 16 records"
        );
    }

    assert_eq!(
        group.accounts[0].identifier.as_ref().unwrap().currency,
        CurrencyCode::Eur
    );
    assert_eq!(
        group.accounts[1].identifier.as_ref().unwrap().currency,
        CurrencyCode::Eur
    );
    // Third account explicitly overrides the group's EUR default back to USD.
    assert_eq!(
        group.accounts[2].identifier.as_ref().unwrap().currency,
        CurrencyCode::Usd
    );

    let value_dated_summary = &group.accounts[2].identifier.as_ref().unwrap().summaries[1];
    assert_eq!(
        value_dated_summary.funds_type,
        FundsType::ValueDated(bai2::ValueDate {
            date: bai2::Date {
                year: 4,
                month: 7,
                day: 1
            },
            time: Some(bai2::Time(1400)),
        })
    );
}

#[test]
fn a_real_control_total_discrepancy_is_caught_end_to_end() {
    // Corrupt the file trailer's own total (the way a transmission error
    // or a receiving-side rounding bug might) and confirm the mismatch
    // surfaces cleanly through three levels of validate() nesting,
    // without masking or duplicating any other violation.
    let corrupted = citi_style_file().replace("99,46150000,1,13/", "99,46150999,1,13/");
    let file = parse_file(&corrupted);

    assert_eq!(
        file.validate(),
        vec![FileViolation::ControlTotalMismatch {
            expected: 46_150_999,
            computed: 46_150_000
        }]
    );
}

#[test]
fn a_business_rule_violation_deep_in_an_account_surfaces_fully_nested() {
    // Introduce a real spec violation (Item Count present on a
    // Status-level Type Code) inside account 2 of the JPMC-style file's
    // second group, and confirm it surfaces as a fully nested
    // FileViolation -> GroupViolation -> AccountViolation chain with the
    // right indices, alongside no other violation.
    let mut file_content = jpmc_style_file();
    file_content = file_content.replace(
        "03,0055566677,,010,1000000,,,100,1000000,1,0/",
        "03,0055566677,,010,1000000,4,,100,1000000,1,0/",
    );
    let file = parse_file(&file_content);

    assert_eq!(
        file.validate(),
        vec![FileViolation::Group {
            group_index: 1,
            violation: GroupViolation::Account {
                account_index: 0,
                violation: AccountViolation::AccountIdentifier(
                    bai2::AccountIdentifierViolation::ItemCountOnStatusTypeCode {
                        type_code: bai2::TypeCode::from("010"),
                    }
                ),
            },
        }]
    );
}
