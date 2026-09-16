//! BAI2 03 (Account Identifier and Summary Status) record.
//!
//! See `docs/ACCOUNT_IDENTIFIER.md` for the technical spec.

use std::fmt;

use crate::currency_code::CurrencyCode;
use crate::funds_type::{FundsType, FundsTypeError};
use crate::type_code::{Level, TypeCode};

/// One `(Type Code, Amount, Item Count, Funds Type)` group within an 03
/// record — a single status or summary value for the account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryStatus {
    pub type_code: TypeCode,
    /// Signed; `None` if defaulted ("no amount reported").
    pub amount: Option<i64>,
    /// `None` if defaulted ("unknown").
    pub item_count: Option<u32>,
    pub funds_type: FundsType,
}

/// A parsed BAI2 03 (Account Identifier and Summary Status) record.
///
/// Parses one already-merged record: if the original transmission split
/// this record across 88 (Continuation) records, the caller must fold
/// them into one field list before calling `new` — see
/// `docs/ACCOUNT_IDENTIFIER.md`'s "Input contract" section for exactly how.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountIdentifier {
    pub customer_account_number: String,
    /// This record's own Currency Code field, or `group_currency` if that
    /// field was defaulted.
    pub currency: CurrencyCode,
    pub summaries: Vec<SummaryStatus>,
}

/// An error parsing an 03 (Account Identifier and Summary Status) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountIdentifierError {
    /// The record's first field isn't the literal `"03"`.
    WrongRecordCode(String),
    /// Customer Account Number is required and was blank or entirely absent.
    MissingAccountNumber,
    /// Customer Account Number contained a `/`, which the spec forbids.
    InvalidAccountNumber(String),
    /// An Amount field wasn't a valid signed integer.
    InvalidAmount(String),
    /// An Item Count field wasn't a valid non-negative integer.
    InvalidItemCount(String),
    /// A Funds Type composite field failed to parse.
    FundsType(FundsTypeError),
}

impl fmt::Display for AccountIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountIdentifierError::WrongRecordCode(v) => {
                write!(f, "account identifier: record code {v:?} is not \"03\"")
            }
            AccountIdentifierError::MissingAccountNumber => {
                write!(
                    f,
                    "account identifier: missing required customer account number"
                )
            }
            AccountIdentifierError::InvalidAccountNumber(v) => {
                write!(f, "account identifier: account number {v:?} contains '/'")
            }
            AccountIdentifierError::InvalidAmount(v) => {
                write!(f, "account identifier: invalid amount {v:?}")
            }
            AccountIdentifierError::InvalidItemCount(v) => {
                write!(f, "account identifier: invalid item count {v:?}")
            }
            AccountIdentifierError::FundsType(e) => write!(f, "account identifier: {e}"),
        }
    }
}

impl std::error::Error for AccountIdentifierError {}

/// A business-rule violation `AccountIdentifier::validate` can find in an
/// otherwise syntactically well-formed record. See
/// `docs/ACCOUNT_IDENTIFIER.md`'s "Business-rule validation is opt-in"
/// note for why these never prevent `new` from succeeding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountIdentifierViolation {
    /// Item Count was present on a Status-level Type Code; the spec
    /// requires it be defaulted there.
    ItemCountOnStatusTypeCode { type_code: TypeCode },
    /// A Summary-level Type Code had a negative Amount; the spec documents
    /// Summary amounts as positive/unsigned only.
    NegativeAmountOnSummaryTypeCode { type_code: TypeCode, amount: i64 },
    /// A Detail-level Type Code appeared in an 03 record; detail belongs
    /// in record 16.
    DetailTypeCodeInAccountIdentifier { type_code: TypeCode },
}

impl fmt::Display for AccountIdentifierViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountIdentifierViolation::ItemCountOnStatusTypeCode { type_code } => write!(
                f,
                "account identifier: item count present on status type code {}",
                type_code.code()
            ),
            AccountIdentifierViolation::NegativeAmountOnSummaryTypeCode { type_code, amount } => {
                write!(
                    f,
                    "account identifier: negative amount {amount} on summary type code {}",
                    type_code.code()
                )
            }
            AccountIdentifierViolation::DetailTypeCodeInAccountIdentifier { type_code } => write!(
                f,
                "account identifier: detail type code {} in 03 record",
                type_code.code()
            ),
        }
    }
}

impl AccountIdentifier {
    /// Parses one already-merged 03 record (see the module doc comment).
    ///
    /// `group_currency` is the enclosing Group Header's Currency Code,
    /// used only as the fallback when this record's own Currency Code
    /// field is blank.
    pub fn new(
        text: &str,
        group_currency: CurrencyCode,
    ) -> Result<AccountIdentifier, AccountIdentifierError> {
        // A trailing '/' would otherwise be glued onto the last present
        // token (no comma before it) and pollute its parse.
        let text = text.strip_suffix('/').unwrap_or(text);
        let fields: Vec<&str> = text.split(',').collect();

        let record_code = fields.first().copied().unwrap_or("");
        if record_code != "03" {
            return Err(AccountIdentifierError::WrongRecordCode(
                record_code.to_string(),
            ));
        }

        let account_number = fields.get(1).copied().unwrap_or("");
        if account_number.is_empty() {
            return Err(AccountIdentifierError::MissingAccountNumber);
        }
        if account_number.contains('/') {
            return Err(AccountIdentifierError::InvalidAccountNumber(
                account_number.to_string(),
            ));
        }

        let currency_str = fields.get(2).copied().unwrap_or("");
        let currency = if currency_str.is_empty() {
            group_currency
        } else {
            CurrencyCode::from(currency_str)
        };

        let mut summaries = Vec::new();
        let mut idx = 3;
        loop {
            let type_code_str = fields.get(idx).copied().unwrap_or("");
            if type_code_str.is_empty() {
                break;
            }
            let type_code = TypeCode::from(type_code_str);

            let amount_str = fields.get(idx + 1).copied().unwrap_or("");
            let amount =
                if amount_str.is_empty() {
                    None
                } else {
                    Some(amount_str.parse::<i64>().map_err(|_| {
                        AccountIdentifierError::InvalidAmount(amount_str.to_string())
                    })?)
                };

            let item_count_str = fields.get(idx + 2).copied().unwrap_or("");
            let item_count = if item_count_str.is_empty() {
                None
            } else {
                Some(item_count_str.parse::<u32>().map_err(|_| {
                    AccountIdentifierError::InvalidItemCount(item_count_str.to_string())
                })?)
            };

            let funds_type_fields: &[&str] = fields.get((idx + 3)..).unwrap_or(&[]);
            let (funds_type, consumed) = if funds_type_fields.is_empty() {
                (FundsType::Unknown, 0)
            } else {
                FundsType::parse(funds_type_fields).map_err(AccountIdentifierError::FundsType)?
            };

            summaries.push(SummaryStatus {
                type_code,
                amount,
                item_count,
                funds_type,
            });
            idx += 3 + consumed;
        }

        Ok(AccountIdentifier {
            customer_account_number: account_number.to_string(),
            currency,
            summaries,
        })
    }

    /// Checks the spec's value-level business rules that a syntactically
    /// well-formed record can still violate. An empty result means the
    /// record is fully spec-conformant, not just well-formed.
    pub fn validate(&self) -> Vec<AccountIdentifierViolation> {
        let mut violations = Vec::new();
        for summary in &self.summaries {
            match summary.type_code.level() {
                Some(Level::Status) => {
                    if summary.item_count.is_some() {
                        violations.push(AccountIdentifierViolation::ItemCountOnStatusTypeCode {
                            type_code: summary.type_code,
                        });
                    }
                }
                Some(Level::Summary) => {
                    if let Some(amount) = summary.amount
                        && amount < 0
                    {
                        violations.push(
                            AccountIdentifierViolation::NegativeAmountOnSummaryTypeCode {
                                type_code: summary.type_code,
                                amount,
                            },
                        );
                    }
                }
                Some(Level::Detail) => {
                    violations.push(
                        AccountIdentifierViolation::DetailTypeCodeInAccountIdentifier {
                            type_code: summary.type_code,
                        },
                    );
                }
                None => {}
            }
        }
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::funds_type::{DistributedAvailability, DistributionDay};

    #[test]
    fn record_with_no_status_or_summary_data_has_no_summaries() {
        let ai = AccountIdentifier::new("03,5765432,,,,,/", CurrencyCode::Usd).unwrap();
        assert_eq!(ai.customer_account_number, "5765432");
        assert_eq!(ai.currency, CurrencyCode::Usd);
        assert_eq!(ai.summaries, vec![]);
    }

    #[test]
    fn parses_multiple_summary_groups_on_one_line() {
        let ai = AccountIdentifier::new(
            "03,0975312468,,010,500000,,,190,70000000,4,0/",
            CurrencyCode::Usd,
        )
        .unwrap();
        assert_eq!(ai.customer_account_number, "0975312468");
        assert_eq!(ai.currency, CurrencyCode::Usd);
        assert_eq!(
            ai.summaries,
            vec![
                SummaryStatus {
                    type_code: TypeCode::from("010"),
                    amount: Some(500_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
                SummaryStatus {
                    type_code: TypeCode::from("190"),
                    amount: Some(70_000_000),
                    item_count: Some(4),
                    funds_type: FundsType::Immediate,
                },
            ]
        );
    }

    #[test]
    fn currency_field_overrides_group_default_when_present() {
        let defaulted = AccountIdentifier::new("03,123,,010,100/", CurrencyCode::Usd).unwrap();
        assert_eq!(defaulted.currency, CurrencyCode::Usd);

        let overridden = AccountIdentifier::new("03,123,JPY,010,100/", CurrencyCode::Usd).unwrap();
        assert_eq!(overridden.currency, CurrencyCode::Jpy);
    }

    #[test]
    fn parses_merged_multiline_03_and_88_example() {
        // Spec sample: 03,9876543210,,010,-500000,,,100,1000000,,,400,2000000,,,190/
        //              88,500000,,,110,1000000,,,072,500000,,,074,500000,,,040/
        //              88,-1500000,,/
        // Merged per the "/" + "88," -> "," rule: drop each continuation
        // line's leading "88" token and join with a single comma.
        let merged = "03,9876543210,,010,-500000,,,100,1000000,,,400,2000000,,,190,\
                       500000,,,110,1000000,,,072,500000,,,074,500000,,,040,\
                       -1500000,,";
        let ai = AccountIdentifier::new(merged, CurrencyCode::Usd).unwrap();
        assert_eq!(ai.customer_account_number, "9876543210");
        assert_eq!(
            ai.summaries,
            vec![
                SummaryStatus {
                    type_code: TypeCode::from("010"),
                    amount: Some(-500_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
                SummaryStatus {
                    type_code: TypeCode::from("100"),
                    amount: Some(1_000_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
                SummaryStatus {
                    type_code: TypeCode::from("400"),
                    amount: Some(2_000_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
                SummaryStatus {
                    type_code: TypeCode::from("190"),
                    amount: Some(500_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
                SummaryStatus {
                    type_code: TypeCode::from("110"),
                    amount: Some(1_000_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
                SummaryStatus {
                    type_code: TypeCode::from("072"),
                    amount: Some(500_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
                SummaryStatus {
                    type_code: TypeCode::from("074"),
                    amount: Some(500_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
                SummaryStatus {
                    type_code: TypeCode::from("040"),
                    amount: Some(-1_500_000),
                    item_count: None,
                    funds_type: FundsType::Unknown,
                },
            ]
        );
    }

    #[test]
    fn parses_s_and_d_funds_type_within_a_group() {
        let ai = AccountIdentifier::new(
            "03,123,,115,10000000,,S,5000000,4000000,1000000,218,20000000,,D,2,0,15000000,1,5000000/",
            CurrencyCode::Usd,
        )
        .unwrap();
        assert_eq!(
            ai.summaries[0].funds_type,
            FundsType::Distributed(DistributedAvailability {
                immediate: 5_000_000,
                one_day: 4_000_000,
                two_or_more_days: 1_000_000,
            })
        );
        assert_eq!(
            ai.summaries[1].funds_type,
            FundsType::DistributedDays(vec![
                DistributionDay {
                    days: 0,
                    amount: 15_000_000
                },
                DistributionDay {
                    days: 1,
                    amount: 5_000_000
                },
            ])
        );
    }

    #[test]
    fn signed_amounts_parse_correctly() {
        let ai = AccountIdentifier::new("03,123,,010,+4350000,,,020,-500000/", CurrencyCode::Usd)
            .unwrap();
        assert_eq!(ai.summaries[0].amount, Some(4_350_000));
        assert_eq!(ai.summaries[1].amount, Some(-500_000));
    }

    #[test]
    fn validate_flags_item_count_on_status_type_code() {
        // 010 (Opening Ledger) is a Status-level code.
        let ai = AccountIdentifier::new("03,123,,010,500000,4,0/", CurrencyCode::Usd).unwrap();
        assert_eq!(
            ai.validate(),
            vec![AccountIdentifierViolation::ItemCountOnStatusTypeCode {
                type_code: TypeCode::from("010"),
            }]
        );
    }

    #[test]
    fn validate_flags_negative_amount_on_summary_type_code() {
        // 190 (Total Loans - Summary) is a Summary-level code.
        let ai = AccountIdentifier::new("03,123,,190,-70000000,4,0/", CurrencyCode::Usd).unwrap();
        assert_eq!(
            ai.validate(),
            vec![
                AccountIdentifierViolation::NegativeAmountOnSummaryTypeCode {
                    type_code: TypeCode::from("190"),
                    amount: -70_000_000,
                }
            ]
        );
    }

    #[test]
    fn validate_flags_detail_type_code_in_account_identifier() {
        // 165 is a Detail-level code (used in 16 records).
        let ai = AccountIdentifier::new("03,123,,165,1500000/", CurrencyCode::Usd).unwrap();
        assert_eq!(
            ai.validate(),
            vec![
                AccountIdentifierViolation::DetailTypeCodeInAccountIdentifier {
                    type_code: TypeCode::from("165"),
                }
            ]
        );
    }

    #[test]
    fn validate_returns_empty_for_conformant_record() {
        let ai = AccountIdentifier::new(
            "03,0975312468,,010,500000,,,190,70000000,4,0/",
            CurrencyCode::Usd,
        )
        .unwrap();
        assert_eq!(ai.validate(), vec![]);
    }

    #[test]
    fn validate_ignores_unknown_type_code() {
        let ai = AccountIdentifier::new("03,123,,999,100,4,0/", CurrencyCode::Usd).unwrap();
        assert_eq!(ai.summaries[0].type_code, TypeCode::Unknown);
        assert_eq!(ai.validate(), vec![]);
    }

    #[test]
    fn wrong_record_code_errors() {
        assert_eq!(
            AccountIdentifier::new("16,010,100/", CurrencyCode::Usd),
            Err(AccountIdentifierError::WrongRecordCode("16".to_string()))
        );
    }

    #[test]
    fn missing_account_number_errors() {
        assert_eq!(
            AccountIdentifier::new("03,,010,100/", CurrencyCode::Usd),
            Err(AccountIdentifierError::MissingAccountNumber)
        );
        assert_eq!(
            AccountIdentifier::new("03", CurrencyCode::Usd),
            Err(AccountIdentifierError::MissingAccountNumber)
        );
    }

    #[test]
    fn invalid_account_number_errors() {
        assert_eq!(
            AccountIdentifier::new("03,123/456,,010,100/", CurrencyCode::Usd),
            Err(AccountIdentifierError::InvalidAccountNumber(
                "123/456".to_string()
            ))
        );
    }

    #[test]
    fn invalid_amount_errors() {
        assert_eq!(
            AccountIdentifier::new("03,123,,010,abc/", CurrencyCode::Usd),
            Err(AccountIdentifierError::InvalidAmount("abc".to_string()))
        );
    }

    #[test]
    fn invalid_item_count_errors() {
        assert_eq!(
            AccountIdentifier::new("03,123,,190,100,abc/", CurrencyCode::Usd),
            Err(AccountIdentifierError::InvalidItemCount("abc".to_string()))
        );
    }

    #[test]
    fn invalid_funds_type_bubbles_up() {
        assert_eq!(
            AccountIdentifier::new("03,123,,010,100,,X/", CurrencyCode::Usd),
            Err(AccountIdentifierError::FundsType(
                FundsTypeError::UnknownCode("X".to_string())
            ))
        );
    }
}
