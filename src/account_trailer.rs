//! BAI2 49 (Account Trailer) record.
//!
//! See `docs/ACCOUNT_TRAILER.md` for the technical spec.

use std::fmt;

/// A parsed BAI2 49 (Account Trailer) record.
///
/// Unlike every other record in this crate, both fields are genuinely
/// mandatory — the spec states "All fields are required" outright, so a
/// blank field is a parse error, not a default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountTrailer {
    /// Algebraic sum of every Amount field back to and including the
    /// preceding 03 record. Does not include Funds Type or Item Count
    /// amounts.
    pub account_control_total: i64,
    /// Count of every physical record from the preceding 03 record
    /// through this 49 record, inclusive.
    pub number_of_records: u32,
}

/// An error parsing a 49 (Account Trailer) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountTrailerError {
    /// The record's first field isn't the literal `"49"`.
    WrongRecordCode(String),
    /// Account Control Total is required and was blank or entirely absent.
    MissingAccountControlTotal,
    /// Account Control Total wasn't a valid signed integer.
    InvalidAccountControlTotal(String),
    /// Number of Records is required and was blank or entirely absent.
    MissingNumberOfRecords,
    /// Number of Records wasn't a valid non-negative integer.
    InvalidNumberOfRecords(String),
}

impl fmt::Display for AccountTrailerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountTrailerError::WrongRecordCode(v) => {
                write!(f, "account trailer: record code {v:?} is not \"49\"")
            }
            AccountTrailerError::MissingAccountControlTotal => {
                write!(f, "account trailer: missing required account control total")
            }
            AccountTrailerError::InvalidAccountControlTotal(v) => {
                write!(f, "account trailer: invalid account control total {v:?}")
            }
            AccountTrailerError::MissingNumberOfRecords => {
                write!(f, "account trailer: missing required number of records")
            }
            AccountTrailerError::InvalidNumberOfRecords(v) => {
                write!(f, "account trailer: invalid number of records {v:?}")
            }
        }
    }
}

impl std::error::Error for AccountTrailerError {}

impl AccountTrailer {
    /// Parses one 49 record line.
    pub fn new(line: &str) -> Result<AccountTrailer, AccountTrailerError> {
        let line = line.strip_suffix('/').unwrap_or(line);
        let fields: Vec<&str> = line.split(',').collect();

        let record_code = fields.first().copied().unwrap_or("");
        if record_code != "49" {
            return Err(AccountTrailerError::WrongRecordCode(
                record_code.to_string(),
            ));
        }

        let total_str = fields.get(1).copied().unwrap_or("");
        if total_str.is_empty() {
            return Err(AccountTrailerError::MissingAccountControlTotal);
        }
        let account_control_total = total_str
            .parse::<i64>()
            .map_err(|_| AccountTrailerError::InvalidAccountControlTotal(total_str.to_string()))?;

        let records_str = fields.get(2).copied().unwrap_or("");
        if records_str.is_empty() {
            return Err(AccountTrailerError::MissingNumberOfRecords);
        }
        let number_of_records = records_str
            .parse::<u32>()
            .map_err(|_| AccountTrailerError::InvalidNumberOfRecords(records_str.to_string()))?;

        Ok(AccountTrailer {
            account_control_total,
            number_of_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_spec_sample() {
        assert_eq!(
            AccountTrailer::new("49,18650000,3/"),
            Ok(AccountTrailer {
                account_control_total: 18_650_000,
                number_of_records: 3
            })
        );
    }

    #[test]
    fn negative_and_explicit_positive_signs_parse_correctly() {
        assert_eq!(
            AccountTrailer::new("49,-18650000,3/")
                .unwrap()
                .account_control_total,
            -18_650_000
        );
        assert_eq!(
            AccountTrailer::new("49,+18650000,3/")
                .unwrap()
                .account_control_total,
            18_650_000
        );
    }

    #[test]
    fn extra_trailing_fields_are_ignored() {
        assert_eq!(
            AccountTrailer::new("49,18650000,3,ignored,fields/"),
            Ok(AccountTrailer {
                account_control_total: 18_650_000,
                number_of_records: 3
            })
        );
    }

    #[test]
    fn wrong_record_code_errors() {
        assert_eq!(
            AccountTrailer::new("03,18650000,3/"),
            Err(AccountTrailerError::WrongRecordCode("03".to_string()))
        );
    }

    #[test]
    fn missing_account_control_total_errors() {
        assert_eq!(
            AccountTrailer::new("49,,3/"),
            Err(AccountTrailerError::MissingAccountControlTotal)
        );
        assert_eq!(
            AccountTrailer::new("49"),
            Err(AccountTrailerError::MissingAccountControlTotal)
        );
    }

    #[test]
    fn missing_number_of_records_errors() {
        assert_eq!(
            AccountTrailer::new("49,18650000,/"),
            Err(AccountTrailerError::MissingNumberOfRecords)
        );
        assert_eq!(
            AccountTrailer::new("49,18650000"),
            Err(AccountTrailerError::MissingNumberOfRecords)
        );
    }

    #[test]
    fn invalid_account_control_total_errors() {
        assert_eq!(
            AccountTrailer::new("49,abc,3/"),
            Err(AccountTrailerError::InvalidAccountControlTotal(
                "abc".to_string()
            ))
        );
    }

    #[test]
    fn invalid_number_of_records_errors() {
        assert_eq!(
            AccountTrailer::new("49,18650000,abc/"),
            Err(AccountTrailerError::InvalidNumberOfRecords(
                "abc".to_string()
            ))
        );
    }

    #[test]
    fn negative_number_of_records_errors() {
        // Number of Records is documented as an unsigned count.
        assert_eq!(
            AccountTrailer::new("49,18650000,-3/"),
            Err(AccountTrailerError::InvalidNumberOfRecords(
                "-3".to_string()
            ))
        );
    }
}
