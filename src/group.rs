//! An assembled BAI2 group: record type 02 (Group Header), zero or more
//! `Account`s (each themselves an 03 + 16* + 49 sequence), and record
//! type 98 (Group Trailer).
//!
//! See `docs/GROUP.md` for the technical spec.

use std::fmt;

use crate::account::{Account, AccountError, AccountViolation};
use crate::group_header::{GroupHeader, GroupHeaderError};
use crate::group_trailer::{GroupTrailer, GroupTrailerError};

/// One BAI2 group: a `GroupHeader` (02), zero or more `Account`s, and a
/// `GroupTrailer` (98), assembled incrementally via `push` in that order
/// — the same shape as `Account` one level up.
///
/// `push` assumes any 88 (Continuation) records have already been merged
/// into the text of whichever record they continue — see `docs/GROUP.md`'s
/// "Scope note" (inherited unchanged from `docs/ACCOUNT.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub header: Option<GroupHeader>,
    pub accounts: Vec<Account>,
    pub trailer: Option<GroupTrailer>,
}

/// An error from `Group::push`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupError {
    /// A second 02 record was pushed; only one is allowed per group.
    DuplicateGroupHeader,
    /// A 98 record was pushed before this group's 02 record.
    GroupTrailerBeforeGroupHeader,
    /// A 03 record was pushed before this group's 02 record.
    AccountBeforeGroupHeader,
    /// A 03 record (opening a new account) or a 98 record (closing the
    /// group) was pushed while the previous account hasn't been closed
    /// with its own 49 record yet.
    PreviousAccountNotClosed,
    /// A 16 or 49 record was pushed with no account currently open —
    /// either none has been started yet, or the last one already closed.
    NoOpenAccount,
    /// A record was pushed after this group's 98 record already closed it.
    PushAfterGroupTrailer,
    /// The record code wasn't "02", "03", "16", "49", or "98".
    UnrecognizedRecordCode(String),
    /// The record code was "88" — continuation merging isn't handled by
    /// this module; the caller must fold it into the preceding record's
    /// text before calling `push`.
    UnmergedContinuationRecord,
    GroupHeader(GroupHeaderError),
    GroupTrailer(GroupTrailerError),
    /// Wraps whatever `Account::push` returned when a 03/16/49 line was
    /// delegated to the currently-open account.
    Account(AccountError),
}

impl fmt::Display for GroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupError::DuplicateGroupHeader => {
                write!(f, "group: a second group header (02) was pushed")
            }
            GroupError::GroupTrailerBeforeGroupHeader => {
                write!(
                    f,
                    "group: group trailer (98) pushed before a group header (02)"
                )
            }
            GroupError::AccountBeforeGroupHeader => {
                write!(
                    f,
                    "group: account identifier (03) pushed before a group header (02)"
                )
            }
            GroupError::PreviousAccountNotClosed => {
                write!(
                    f,
                    "group: pushed before the previous account was closed with a 49 record"
                )
            }
            GroupError::NoOpenAccount => {
                write!(
                    f,
                    "group: no account is currently open; push a 03 record first"
                )
            }
            GroupError::PushAfterGroupTrailer => {
                write!(f, "group: record pushed after this group's trailer (98)")
            }
            GroupError::UnrecognizedRecordCode(v) => {
                write!(f, "group: unrecognized record code {v:?}")
            }
            GroupError::UnmergedContinuationRecord => write!(
                f,
                "group: a continuation (88) record was pushed on its own; merge it into the preceding record's text first"
            ),
            GroupError::GroupHeader(e) => write!(f, "group: {e}"),
            GroupError::GroupTrailer(e) => write!(f, "group: {e}"),
            GroupError::Account(e) => write!(f, "group: {e}"),
        }
    }
}

impl std::error::Error for GroupError {}

/// A violation `Group::validate` can find, beyond what `Account::validate`
/// already checks on its own for each account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupViolation {
    /// A violation found in one of this group's accounts, identified by
    /// its position in `accounts`.
    Account {
        account_index: usize,
        violation: AccountViolation,
    },
    /// The 98 record's Group Control Total doesn't equal the algebraic
    /// sum of every account's Account Control Total.
    ControlTotalMismatch { expected: i64, computed: i64 },
    /// The 98 record's Number of Accounts doesn't equal the actual number
    /// of accounts pushed.
    AccountCountMismatch { expected: u32, actual: usize },
}

impl fmt::Display for GroupViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupViolation::Account {
                account_index,
                violation,
            } => {
                write!(f, "group: account {account_index}: {violation}")
            }
            GroupViolation::ControlTotalMismatch { expected, computed } => write!(
                f,
                "group: control total {expected} doesn't match computed sum {computed}"
            ),
            GroupViolation::AccountCountMismatch { expected, actual } => write!(
                f,
                "group: number of accounts {expected} doesn't match actual count {actual}"
            ),
        }
    }
}

/// Whether the last account in `accounts`, if any, is still open (no
/// trailer yet).
fn last_account_is_open(accounts: &[Account]) -> bool {
    matches!(accounts.last(), Some(account) if account.trailer.is_none())
}

impl Group {
    /// Returns an empty group ready for `push`. Unlike
    /// `Account::new(group_currency)`, this takes no parameters — `Group`
    /// is where currency actually originates (via the 02 record's own
    /// Currency Code field), not something supplied from outside.
    pub fn new() -> Group {
        Group {
            header: None,
            accounts: Vec::new(),
            trailer: None,
        }
    }

    /// Parses one already-merged record's text and adds it to the group,
    /// dispatching on its record code: `02`/`98` are handled directly;
    /// `03` opens the next `Account`; `16`/`49` are delegated to the
    /// currently-open account.
    pub fn push(&mut self, text: &str) -> Result<(), GroupError> {
        let record_code = text.split(',').next().unwrap_or("");
        match record_code {
            "02" => {
                if self.trailer.is_some() {
                    return Err(GroupError::PushAfterGroupTrailer);
                }
                if self.header.is_some() {
                    return Err(GroupError::DuplicateGroupHeader);
                }
                self.header = Some(GroupHeader::new(text).map_err(GroupError::GroupHeader)?);
                Ok(())
            }
            "03" => {
                if self.trailer.is_some() {
                    return Err(GroupError::PushAfterGroupTrailer);
                }
                let header = self
                    .header
                    .as_ref()
                    .ok_or(GroupError::AccountBeforeGroupHeader)?;
                if last_account_is_open(&self.accounts) {
                    return Err(GroupError::PreviousAccountNotClosed);
                }
                let mut account = Account::new(header.currency);
                account.push(text).map_err(GroupError::Account)?;
                self.accounts.push(account);
                Ok(())
            }
            "16" | "49" => {
                if self.trailer.is_some() {
                    return Err(GroupError::PushAfterGroupTrailer);
                }
                if !last_account_is_open(&self.accounts) {
                    return Err(GroupError::NoOpenAccount);
                }
                let account = self
                    .accounts
                    .last_mut()
                    .expect("just checked an open account exists");
                account.push(text).map_err(GroupError::Account)?;
                Ok(())
            }
            "98" => {
                if self.trailer.is_some() {
                    return Err(GroupError::PushAfterGroupTrailer);
                }
                if self.header.is_none() {
                    return Err(GroupError::GroupTrailerBeforeGroupHeader);
                }
                if last_account_is_open(&self.accounts) {
                    return Err(GroupError::PreviousAccountNotClosed);
                }
                self.trailer = Some(GroupTrailer::new(text).map_err(GroupError::GroupTrailer)?);
                Ok(())
            }
            "88" => Err(GroupError::UnmergedContinuationRecord),
            other => Err(GroupError::UnrecognizedRecordCode(other.to_string())),
        }
    }

    /// Checks `Account::validate`'s rules on every account in `accounts`,
    /// plus reconciles the 98 record's Group Control Total and Number of
    /// Accounts against what was actually pushed (only if `trailer` is
    /// present — otherwise both checks are simply skipped, not reported
    /// as violations).
    pub fn validate(&self) -> Vec<GroupViolation> {
        let mut violations = Vec::new();

        for (account_index, account) in self.accounts.iter().enumerate() {
            violations.extend(account.validate().into_iter().map(|violation| {
                GroupViolation::Account {
                    account_index,
                    violation,
                }
            }));
        }

        if let Some(trailer) = &self.trailer {
            let computed: i64 = self
                .accounts
                .iter()
                .filter_map(|account| account.trailer.map(|t| t.account_control_total))
                .sum();
            if computed != trailer.group_control_total {
                violations.push(GroupViolation::ControlTotalMismatch {
                    expected: trailer.group_control_total,
                    computed,
                });
            }

            let actual = self.accounts.len();
            if trailer.number_of_accounts as usize != actual {
                violations.push(GroupViolation::AccountCountMismatch {
                    expected: trailer.number_of_accounts,
                    actual,
                });
            }
        }

        violations
    }
}

impl Default for Group {
    fn default() -> Group {
        Group::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_identifier::AccountIdentifierViolation;
    use crate::currency_code::CurrencyCode;
    use crate::transaction_detail::TransactionDetailError;
    use crate::type_code::TypeCode;

    fn push_full_account(group: &mut Group, account_number: &str, control_total: i64) {
        group
            .push(&format!("03,{account_number},,190,{control_total},4,0/"))
            .unwrap();
        group.push(&format!("49,{control_total},2/")).unwrap();
    }

    #[test]
    fn pushes_header_then_accounts_then_trailer_in_order() {
        let mut group = Group::new();
        group
            .push("02,031001234,122099999,1,040620,2359,,2/")
            .unwrap();
        push_full_account(&mut group, "111", 100);
        push_full_account(&mut group, "222", 200);
        group.push("98,300,2,6/").unwrap();

        assert_eq!(
            group.header.as_ref().unwrap().originator_identification,
            "122099999"
        );
        assert_eq!(group.accounts.len(), 2);
        assert_eq!(group.trailer.unwrap().group_control_total, 300);
    }

    #[test]
    fn account_before_group_header_errors() {
        let mut group = Group::new();
        assert_eq!(
            group.push("03,123,,010,100/"),
            Err(GroupError::AccountBeforeGroupHeader)
        );
    }

    #[test]
    fn no_open_account_for_16_and_49_before_any_account() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        assert_eq!(group.push("16,010,100/"), Err(GroupError::NoOpenAccount));
        assert_eq!(group.push("49,100,2/"), Err(GroupError::NoOpenAccount));
    }

    #[test]
    fn no_open_account_after_account_already_closed() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        push_full_account(&mut group, "111", 100);
        assert_eq!(group.push("16,010,100/"), Err(GroupError::NoOpenAccount));
    }

    #[test]
    fn group_trailer_before_group_header_errors() {
        let mut group = Group::new();
        assert_eq!(
            group.push("98,100,1,2/"),
            Err(GroupError::GroupTrailerBeforeGroupHeader)
        );
    }

    #[test]
    fn duplicate_group_header_errors() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        assert_eq!(
            group.push("02,,ORIG2,1,040620/"),
            Err(GroupError::DuplicateGroupHeader)
        );
    }

    #[test]
    fn previous_account_not_closed_blocks_new_account_and_group_trailer() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        group.push("03,111,,010,100/").unwrap();
        assert_eq!(
            group.push("03,222,,010,100/"),
            Err(GroupError::PreviousAccountNotClosed)
        );
        assert_eq!(
            group.push("98,100,1,2/"),
            Err(GroupError::PreviousAccountNotClosed)
        );
    }

    #[test]
    fn push_after_group_trailer_errors() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        push_full_account(&mut group, "111", 100);
        group.push("98,100,1,2/").unwrap();
        assert_eq!(
            group.push("16,010,100/"),
            Err(GroupError::PushAfterGroupTrailer)
        );
        assert_eq!(
            group.push("03,222,,010,100/"),
            Err(GroupError::PushAfterGroupTrailer)
        );
        assert_eq!(
            group.push("98,100,1,2/"),
            Err(GroupError::PushAfterGroupTrailer)
        );
    }

    #[test]
    fn unrecognized_and_unmerged_continuation_record_codes_error() {
        let mut group = Group::new();
        assert_eq!(
            group.push("77,text/"),
            Err(GroupError::UnrecognizedRecordCode("77".to_string()))
        );
        assert_eq!(
            group.push("88,text"),
            Err(GroupError::UnmergedContinuationRecord)
        );
    }

    #[test]
    fn malformed_lines_bubble_up_the_exact_underlying_error() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        group.push("03,111,,010,100/").unwrap();
        assert_eq!(
            group.push("16,010,-100/"),
            Err(GroupError::Account(AccountError::TransactionDetail(
                TransactionDetailError::InvalidAmount("-100".to_string())
            )))
        );
    }

    #[test]
    fn account_inherits_group_header_currency() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620,,JPY,2/").unwrap();
        group.push("03,111,,010,100/").unwrap();
        group.push("16,010,100/").unwrap();
        assert_eq!(
            group.accounts[0].identifier.as_ref().unwrap().currency,
            CurrencyCode::Jpy
        );
        assert_eq!(
            group.accounts[0].transaction_details[0].currency,
            CurrencyCode::Jpy
        );
    }

    #[test]
    fn validate_returns_empty_for_conformant_closed_group() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        push_full_account(&mut group, "111", 100);
        push_full_account(&mut group, "222", 200);
        group.push("98,300,2,6/").unwrap();
        assert_eq!(group.validate(), vec![]);
    }

    #[test]
    fn validate_wraps_account_violations_with_index() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        // 010 is Status-level; a present Item Count violates the spec.
        group.push("03,111,,010,100,4,0/").unwrap();
        group.push("49,100,2/").unwrap();
        assert_eq!(
            group.validate(),
            vec![GroupViolation::Account {
                account_index: 0,
                violation: AccountViolation::AccountIdentifier(
                    AccountIdentifierViolation::ItemCountOnStatusTypeCode {
                        type_code: TypeCode::from("010"),
                    }
                ),
            }]
        );
    }

    #[test]
    fn validate_flags_control_total_and_account_count_mismatches() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        push_full_account(&mut group, "111", 100);
        group.push("98,999,5,2/").unwrap();
        assert_eq!(
            group.validate(),
            vec![
                GroupViolation::ControlTotalMismatch {
                    expected: 999,
                    computed: 100
                },
                GroupViolation::AccountCountMismatch {
                    expected: 5,
                    actual: 1
                },
            ]
        );
    }

    #[test]
    fn validate_skips_trailer_checks_without_a_trailer() {
        let mut group = Group::new();
        group.push("02,,ORIG,1,040620/").unwrap();
        push_full_account(&mut group, "111", 100);
        assert_eq!(group.validate(), vec![]);
    }
}
