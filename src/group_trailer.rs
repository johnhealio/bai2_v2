//! BAI2 98 (Group Trailer) record.
//!
//! See `docs/GROUP_TRAILER.md` for the technical spec.

use std::fmt;

/// A parsed BAI2 98 (Group Trailer) record.
///
/// All three fields are genuinely mandatory — the spec states "All
/// fields are required" outright, so a blank field is a parse error, not
/// a default. Structurally almost identical to `AccountTrailer`
/// (`docs/ACCOUNT_TRAILER.md`), just one field longer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupTrailer {
    /// Algebraic sum of every 03 record's Account Control Total in this
    /// group.
    pub group_control_total: i64,
    /// Count of 03 records in this group.
    pub number_of_accounts: u32,
    /// Count of every physical record in this group, inclusive: the 02
    /// record, every 03/16/49/88 record, and this 98 record.
    pub number_of_records: u32,
}

/// An error parsing a 98 (Group Trailer) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupTrailerError {
    /// The record's first field isn't the literal `"98"`.
    WrongRecordCode(String),
    /// Group Control Total is required and was blank or entirely absent.
    MissingGroupControlTotal,
    /// Group Control Total wasn't a valid signed integer.
    InvalidGroupControlTotal(String),
    /// Number of Accounts is required and was blank or entirely absent.
    MissingNumberOfAccounts,
    /// Number of Accounts wasn't a valid non-negative integer.
    InvalidNumberOfAccounts(String),
    /// Number of Records is required and was blank or entirely absent.
    MissingNumberOfRecords,
    /// Number of Records wasn't a valid non-negative integer.
    InvalidNumberOfRecords(String),
}

impl fmt::Display for GroupTrailerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupTrailerError::WrongRecordCode(v) => {
                write!(f, "group trailer: record code {v:?} is not \"98\"")
            }
            GroupTrailerError::MissingGroupControlTotal => {
                write!(f, "group trailer: missing required group control total")
            }
            GroupTrailerError::InvalidGroupControlTotal(v) => {
                write!(f, "group trailer: invalid group control total {v:?}")
            }
            GroupTrailerError::MissingNumberOfAccounts => {
                write!(f, "group trailer: missing required number of accounts")
            }
            GroupTrailerError::InvalidNumberOfAccounts(v) => {
                write!(f, "group trailer: invalid number of accounts {v:?}")
            }
            GroupTrailerError::MissingNumberOfRecords => {
                write!(f, "group trailer: missing required number of records")
            }
            GroupTrailerError::InvalidNumberOfRecords(v) => {
                write!(f, "group trailer: invalid number of records {v:?}")
            }
        }
    }
}

impl std::error::Error for GroupTrailerError {}

impl GroupTrailer {
    /// Parses one 98 record line.
    pub fn new(line: &str) -> Result<GroupTrailer, GroupTrailerError> {
        let line = line.strip_suffix('/').unwrap_or(line);
        let fields: Vec<&str> = line.split(',').collect();

        let record_code = fields.first().copied().unwrap_or("");
        if record_code != "98" {
            return Err(GroupTrailerError::WrongRecordCode(record_code.to_string()));
        }

        let total_str = fields.get(1).copied().unwrap_or("");
        if total_str.is_empty() {
            return Err(GroupTrailerError::MissingGroupControlTotal);
        }
        let group_control_total = total_str
            .parse::<i64>()
            .map_err(|_| GroupTrailerError::InvalidGroupControlTotal(total_str.to_string()))?;

        let accounts_str = fields.get(2).copied().unwrap_or("");
        if accounts_str.is_empty() {
            return Err(GroupTrailerError::MissingNumberOfAccounts);
        }
        let number_of_accounts = accounts_str
            .parse::<u32>()
            .map_err(|_| GroupTrailerError::InvalidNumberOfAccounts(accounts_str.to_string()))?;

        let records_str = fields.get(3).copied().unwrap_or("");
        if records_str.is_empty() {
            return Err(GroupTrailerError::MissingNumberOfRecords);
        }
        let number_of_records = records_str
            .parse::<u32>()
            .map_err(|_| GroupTrailerError::InvalidNumberOfRecords(records_str.to_string()))?;

        Ok(GroupTrailer {
            group_control_total,
            number_of_accounts,
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
            GroupTrailer::new("98,11800000,2,6/"),
            Ok(GroupTrailer {
                group_control_total: 11_800_000,
                number_of_accounts: 2,
                number_of_records: 6,
            })
        );
    }

    #[test]
    fn negative_and_explicit_positive_signs_parse_correctly() {
        assert_eq!(
            GroupTrailer::new("98,-11800000,2,6/")
                .unwrap()
                .group_control_total,
            -11_800_000
        );
        assert_eq!(
            GroupTrailer::new("98,+11800000,2,6/")
                .unwrap()
                .group_control_total,
            11_800_000
        );
    }

    #[test]
    fn extra_trailing_fields_are_ignored() {
        assert_eq!(
            GroupTrailer::new("98,11800000,2,6,ignored,fields/"),
            Ok(GroupTrailer {
                group_control_total: 11_800_000,
                number_of_accounts: 2,
                number_of_records: 6,
            })
        );
    }

    #[test]
    fn wrong_record_code_errors() {
        assert_eq!(
            GroupTrailer::new("49,11800000,2,6/"),
            Err(GroupTrailerError::WrongRecordCode("49".to_string()))
        );
    }

    #[test]
    fn missing_fields_error() {
        assert_eq!(
            GroupTrailer::new("98,,2,6/"),
            Err(GroupTrailerError::MissingGroupControlTotal)
        );
        assert_eq!(
            GroupTrailer::new("98,11800000,,6/"),
            Err(GroupTrailerError::MissingNumberOfAccounts)
        );
        assert_eq!(
            GroupTrailer::new("98,11800000,2,/"),
            Err(GroupTrailerError::MissingNumberOfRecords)
        );
        assert_eq!(
            GroupTrailer::new("98"),
            Err(GroupTrailerError::MissingGroupControlTotal)
        );
    }

    #[test]
    fn invalid_fields_error() {
        assert_eq!(
            GroupTrailer::new("98,abc,2,6/"),
            Err(GroupTrailerError::InvalidGroupControlTotal(
                "abc".to_string()
            ))
        );
        assert_eq!(
            GroupTrailer::new("98,11800000,abc,6/"),
            Err(GroupTrailerError::InvalidNumberOfAccounts(
                "abc".to_string()
            ))
        );
        assert_eq!(
            GroupTrailer::new("98,11800000,2,abc/"),
            Err(GroupTrailerError::InvalidNumberOfRecords("abc".to_string()))
        );
    }

    #[test]
    fn negative_counts_error() {
        assert_eq!(
            GroupTrailer::new("98,11800000,-2,6/"),
            Err(GroupTrailerError::InvalidNumberOfAccounts("-2".to_string()))
        );
        assert_eq!(
            GroupTrailer::new("98,11800000,2,-6/"),
            Err(GroupTrailerError::InvalidNumberOfRecords("-6".to_string()))
        );
    }
}
