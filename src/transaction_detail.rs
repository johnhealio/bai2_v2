//! BAI2 16 (Transaction Detail) record.
//!
//! See `docs/TRANSACTION_DETAIL.md` for the technical spec.

use std::fmt;

use crate::currency_code::CurrencyCode;
use crate::funds_type::{FundsType, FundsTypeError};
use crate::type_code::TypeCode;

/// A parsed BAI2 16 (Transaction Detail) record.
///
/// Parses exactly one physical/logical line. If a record's `Text` field
/// spills into subsequent 88 (Continuation) records, stitching those lines
/// together is a record-stream/file-level concern and out of scope here —
/// see `docs/TRANSACTION_DETAIL.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionDetail {
    pub type_code: TypeCode,
    /// The reported amount, in the currency's implied minor units (e.g.
    /// cents for a 2-decimal currency). `None` if the field was defaulted
    /// — a distinct state from a reported amount of zero.
    pub amount: Option<u64>,
    /// The currency `amount` is denominated in, and the decimal count it
    /// implies. Comes from the enclosing Group/Account record, not from
    /// anything in the 16 record itself — this record has no currency
    /// field of its own.
    pub currency: CurrencyCode,
    pub funds_type: FundsType,
    pub bank_reference_number: Option<String>,
    pub customer_reference_number: Option<String>,
    pub text: Option<String>,
}

/// An error parsing a 16 (Transaction Detail) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionDetailError {
    /// The record's first field isn't the literal `"16"`.
    WrongRecordCode(String),
    /// Type Code is required and was blank or entirely absent.
    MissingTypeCode,
    /// The Amount field wasn't a valid non-negative integer.
    InvalidAmount(String),
    /// The Funds Type composite field failed to parse.
    FundsType(FundsTypeError),
    /// Bank/Customer Reference Number contained a `/`, which the spec
    /// forbids in either field.
    InvalidReferenceNumber { field: &'static str, value: String },
    /// The Text field began with `/`, which the spec forbids.
    TextStartsWithSlash(String),
}

impl fmt::Display for TransactionDetailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionDetailError::WrongRecordCode(v) => {
                write!(f, "transaction detail: record code {v:?} is not \"16\"")
            }
            TransactionDetailError::MissingTypeCode => {
                write!(f, "transaction detail: missing required type code")
            }
            TransactionDetailError::InvalidAmount(v) => {
                write!(f, "transaction detail: invalid amount {v:?}")
            }
            TransactionDetailError::FundsType(e) => write!(f, "transaction detail: {e}"),
            TransactionDetailError::InvalidReferenceNumber { field, value } => {
                write!(f, "transaction detail: {field} {value:?} contains '/'")
            }
            TransactionDetailError::TextStartsWithSlash(v) => {
                write!(f, "transaction detail: text {v:?} begins with '/'")
            }
        }
    }
}

impl std::error::Error for TransactionDetailError {}

/// Returns `None` for an absent or blank field, `Some(value)` otherwise, and
/// rejects a value containing `/` (forbidden in Bank/Customer Reference
/// Number by the spec).
fn optional_reference_field(
    value: Option<&str>,
    field: &'static str,
) -> Result<Option<String>, TransactionDetailError> {
    match value {
        None | Some("") => Ok(None),
        Some(s) if s.contains('/') => Err(TransactionDetailError::InvalidReferenceNumber {
            field,
            value: s.to_string(),
        }),
        Some(s) => Ok(Some(s.to_string())),
    }
}

impl TransactionDetail {
    /// Parses one 16 record line.
    ///
    /// `currency` is the Currency Code in force for the enclosing
    /// Group/Account record; it's attached to the result so callers have
    /// `amount` and the currency it's denominated in together, without
    /// re-threading it from elsewhere.
    pub fn new(
        line: &str,
        currency: CurrencyCode,
    ) -> Result<TransactionDetail, TransactionDetailError> {
        // A trailing '/' only ever terminates a record that does *not*
        // include Text (a record ending in Text has no delimiter at all,
        // per spec) — safe to strip unconditionally before field-splitting.
        let line = line.strip_suffix('/').unwrap_or(line);
        let fields: Vec<&str> = line.split(',').collect();

        let record_code = fields.first().copied().unwrap_or("");
        if record_code != "16" {
            return Err(TransactionDetailError::WrongRecordCode(
                record_code.to_string(),
            ));
        }

        let type_code_str = fields.get(1).copied().unwrap_or("");
        if type_code_str.is_empty() {
            return Err(TransactionDetailError::MissingTypeCode);
        }
        let type_code = TypeCode::from(type_code_str);

        let amount_str = fields.get(2).copied().unwrap_or("");
        let amount = if amount_str.is_empty() {
            None
        } else {
            Some(
                amount_str
                    .parse::<u64>()
                    .map_err(|_| TransactionDetailError::InvalidAmount(amount_str.to_string()))?,
            )
        };

        // Trailing optional fields may be truncated off the line entirely
        // (not even represented by empty commas) rather than always
        // spelled out — treat "nothing left" the same as "blank".
        let funds_type_fields: &[&str] = fields.get(3..).unwrap_or(&[]);
        let (funds_type, consumed) = if funds_type_fields.is_empty() {
            (FundsType::Unknown, 0)
        } else {
            FundsType::parse(funds_type_fields).map_err(TransactionDetailError::FundsType)?
        };

        let bank_reference_number =
            optional_reference_field(fields.get(3 + consumed).copied(), "bank reference number")?;
        let customer_reference_number = optional_reference_field(
            fields.get(4 + consumed).copied(),
            "customer reference number",
        )?;

        let text_tail: &[&str] = fields.get((5 + consumed)..).unwrap_or(&[]);
        let text = if text_tail.iter().all(|s| s.is_empty()) {
            None
        } else {
            let joined = text_tail.join(",");
            if joined.starts_with('/') {
                return Err(TransactionDetailError::TextStartsWithSlash(joined));
            }
            Some(joined)
        };

        Ok(TransactionDetail {
            type_code,
            amount,
            currency,
            funds_type,
            bank_reference_number,
            customer_reference_number,
            text,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::funds_type::{DistributedAvailability, DistributionDay};

    #[test]
    fn parses_plain_record_with_text() {
        let td = TransactionDetail::new(
            "16,165,1500000,1,DD1620,, DEALER PAYMENTS",
            CurrencyCode::Usd,
        )
        .unwrap();
        assert_eq!(td.type_code, TypeCode::from("165"));
        assert_eq!(td.amount, Some(1_500_000));
        assert_eq!(td.currency, CurrencyCode::Usd);
        assert_eq!(td.funds_type, FundsType::OneDay);
        assert_eq!(td.bank_reference_number, Some("DD1620".to_string()));
        assert_eq!(td.customer_reference_number, None);
        assert_eq!(td.text, Some(" DEALER PAYMENTS".to_string()));
    }

    #[test]
    fn parses_distributed_s_funds_type_with_truncated_trailing_fields() {
        let td = TransactionDetail::new(
            "16,115,10000000,S,5000000,4000000,1000000/",
            CurrencyCode::Usd,
        )
        .unwrap();
        assert_eq!(td.amount, Some(10_000_000));
        assert_eq!(
            td.funds_type,
            FundsType::Distributed(DistributedAvailability {
                immediate: 5_000_000,
                one_day: 4_000_000,
                two_or_more_days: 1_000_000,
            })
        );
        assert_eq!(td.bank_reference_number, None);
        assert_eq!(td.customer_reference_number, None);
        assert_eq!(td.text, None);
    }

    #[test]
    fn parses_distributed_d_funds_type() {
        let td = TransactionDetail::new(
            "16,218,20000000,D,2,0,15000000,1,5000000,SP4738,YRC065321",
            CurrencyCode::Usd,
        )
        .unwrap();
        assert_eq!(
            td.funds_type,
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
        assert_eq!(td.bank_reference_number, Some("SP4738".to_string()));
        assert_eq!(td.customer_reference_number, Some("YRC065321".to_string()));
        assert_eq!(td.text, None);
    }

    #[test]
    fn parses_nonmonetary_890_record() {
        let td = TransactionDetail::new(
            "16,890,,,,,detail reports will be delayed until 11:00 AM.",
            CurrencyCode::Usd,
        )
        .unwrap();
        assert_eq!(td.type_code, TypeCode::from("890"));
        assert_eq!(td.amount, None);
        assert_eq!(td.funds_type, FundsType::Unknown);
        assert_eq!(td.bank_reference_number, None);
        assert_eq!(td.customer_reference_number, None);
        assert_eq!(
            td.text,
            Some("detail reports will be delayed until 11:00 AM.".to_string())
        );
    }

    #[test]
    fn text_may_contain_commas_and_slashes_after_the_first_character() {
        let td = TransactionDetail::new("16,890,,,,,a/b,c,d", CurrencyCode::Usd).unwrap();
        assert_eq!(td.text, Some("a/b,c,d".to_string()));
    }

    #[test]
    fn amount_is_stored_as_raw_minor_units_alongside_its_currency() {
        let jpy = TransactionDetail::new("16,010,150097", CurrencyCode::Jpy).unwrap();
        assert_eq!(jpy.amount, Some(150_097));
        assert_eq!(jpy.currency, CurrencyCode::Jpy);
        assert_eq!(jpy.currency.decimals(), 0);

        let usd = TransactionDetail::new("16,010,150097", CurrencyCode::Usd).unwrap();
        assert_eq!(usd.amount, Some(150_097));
        assert_eq!(usd.currency, CurrencyCode::Usd);
        assert_eq!(usd.currency.decimals(), 2);
    }

    #[test]
    fn amount_defaults_to_none_when_blank() {
        let td = TransactionDetail::new("16,890,,,,,", CurrencyCode::Usd).unwrap();
        assert_eq!(td.amount, None);
    }

    #[test]
    fn wrong_record_code_errors() {
        assert_eq!(
            TransactionDetail::new("03,010,100/", CurrencyCode::Usd),
            Err(TransactionDetailError::WrongRecordCode("03".to_string()))
        );
    }

    #[test]
    fn missing_type_code_errors() {
        assert_eq!(
            TransactionDetail::new("16,,100/", CurrencyCode::Usd),
            Err(TransactionDetailError::MissingTypeCode)
        );
        assert_eq!(
            TransactionDetail::new("16", CurrencyCode::Usd),
            Err(TransactionDetailError::MissingTypeCode)
        );
    }

    #[test]
    fn invalid_amount_errors() {
        assert_eq!(
            TransactionDetail::new("16,010,-100/", CurrencyCode::Usd),
            Err(TransactionDetailError::InvalidAmount("-100".to_string()))
        );
        assert_eq!(
            TransactionDetail::new("16,010,abc/", CurrencyCode::Usd),
            Err(TransactionDetailError::InvalidAmount("abc".to_string()))
        );
    }

    #[test]
    fn invalid_funds_type_bubbles_up() {
        assert_eq!(
            TransactionDetail::new("16,010,100,X/", CurrencyCode::Usd),
            Err(TransactionDetailError::FundsType(
                FundsTypeError::UnknownCode("X".to_string())
            ))
        );
    }

    #[test]
    fn reference_number_with_slash_errors() {
        assert_eq!(
            TransactionDetail::new("16,010,100,1,DD/1620/", CurrencyCode::Usd),
            Err(TransactionDetailError::InvalidReferenceNumber {
                field: "bank reference number",
                value: "DD/1620".to_string(),
            })
        );
    }

    #[test]
    fn text_starting_with_slash_errors() {
        assert_eq!(
            TransactionDetail::new("16,010,100,1,,,/oops", CurrencyCode::Usd),
            Err(TransactionDetailError::TextStartsWithSlash(
                "/oops".to_string()
            ))
        );
    }
}
