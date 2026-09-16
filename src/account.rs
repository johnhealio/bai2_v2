//! An assembled BAI2 account: record types 03 (Account Identifier), 16
//! (Transaction Detail, zero or more), and 49 (Account Trailer).
//!
//! See `docs/ACCOUNT.md` for the technical spec.

use std::fmt;

use crate::account_identifier::{
    AccountIdentifier, AccountIdentifierError, AccountIdentifierViolation,
};
use crate::account_trailer::{AccountTrailer, AccountTrailerError};
use crate::currency_code::CurrencyCode;
use crate::transaction_detail::{TransactionDetail, TransactionDetailError};

/// One BAI2 account: an `AccountIdentifier` (03), zero or more
/// `TransactionDetail`s (16), and an `AccountTrailer` (49), assembled
/// incrementally via `push` in that order.
///
/// `push` assumes any 88 (Continuation) records have already been merged
/// into the text of whichever 03/16 record they continue — see
/// `docs/ACCOUNT.md`'s "Scope note" for why that merging isn't done here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub identifier: Option<AccountIdentifier>,
    pub transaction_details: Vec<TransactionDetail>,
    pub trailer: Option<AccountTrailer>,
    /// The enclosing Group Header's Currency Code. Used to resolve the 03
    /// record's own Currency Code field when it's blank; every 16 record
    /// then inherits the *resolved* account currency, not this field
    /// directly — see `push`'s "16" case.
    group_currency: CurrencyCode,
}

/// An error from `Account::push`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountError {
    /// A 16 record was pushed before this account's 03 record.
    TransactionDetailBeforeAccountIdentifier,
    /// A 49 record was pushed before this account's 03 record.
    AccountTrailerBeforeAccountIdentifier,
    /// A second 03 record was pushed; only one is allowed per account.
    DuplicateAccountIdentifier,
    /// A record was pushed after this account's 49 record already closed it.
    PushAfterAccountTrailer,
    /// The record code wasn't "03", "16", or "49".
    UnrecognizedRecordCode(String),
    /// The record code was "88" — continuation merging isn't handled by
    /// this module; the caller must fold it into the preceding record's
    /// text before calling `push`.
    UnmergedContinuationRecord,
    AccountIdentifier(AccountIdentifierError),
    TransactionDetail(TransactionDetailError),
    AccountTrailer(AccountTrailerError),
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::TransactionDetailBeforeAccountIdentifier => write!(
                f,
                "account: transaction detail (16) pushed before an account identifier (03)"
            ),
            AccountError::AccountTrailerBeforeAccountIdentifier => write!(
                f,
                "account: account trailer (49) pushed before an account identifier (03)"
            ),
            AccountError::DuplicateAccountIdentifier => {
                write!(f, "account: a second account identifier (03) was pushed")
            }
            AccountError::PushAfterAccountTrailer => {
                write!(
                    f,
                    "account: record pushed after this account's trailer (49)"
                )
            }
            AccountError::UnrecognizedRecordCode(v) => {
                write!(f, "account: unrecognized record code {v:?}")
            }
            AccountError::UnmergedContinuationRecord => write!(
                f,
                "account: a continuation (88) record was pushed on its own; merge it into the preceding record's text first"
            ),
            AccountError::AccountIdentifier(e) => write!(f, "account: {e}"),
            AccountError::TransactionDetail(e) => write!(f, "account: {e}"),
            AccountError::AccountTrailer(e) => write!(f, "account: {e}"),
        }
    }
}

impl std::error::Error for AccountError {}

/// A violation `Account::validate` can find, beyond what
/// `AccountIdentifier::validate` already checks on its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountViolation {
    /// A violation found in this account's 03 record.
    AccountIdentifier(AccountIdentifierViolation),
    /// The 49 record's Account Control Total doesn't equal the algebraic
    /// sum of every Amount field in the 03 record and every 16 record
    /// pushed so far.
    ControlTotalMismatch { expected: i64, computed: i64 },
}

impl fmt::Display for AccountViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountViolation::AccountIdentifier(v) => write!(f, "account: {v}"),
            AccountViolation::ControlTotalMismatch { expected, computed } => write!(
                f,
                "account: control total {expected} doesn't match computed sum {computed}"
            ),
        }
    }
}

impl Account {
    /// Returns an empty account ready for `push`. `group_currency` is the
    /// enclosing Group Header's Currency Code.
    pub fn new(group_currency: CurrencyCode) -> Account {
        Account {
            identifier: None,
            transaction_details: Vec::new(),
            trailer: None,
            group_currency,
        }
    }

    /// Parses one already-merged record's text and adds it to the account,
    /// dispatching on its record code and enforcing the 03 → 16* → 49
    /// order.
    pub fn push(&mut self, text: &str) -> Result<(), AccountError> {
        let record_code = text.split(',').next().unwrap_or("");
        match record_code {
            "03" => {
                if self.trailer.is_some() {
                    return Err(AccountError::PushAfterAccountTrailer);
                }
                if self.identifier.is_some() {
                    return Err(AccountError::DuplicateAccountIdentifier);
                }
                let identifier = AccountIdentifier::new(text, self.group_currency)
                    .map_err(AccountError::AccountIdentifier)?;
                self.identifier = Some(identifier);
                Ok(())
            }
            "16" => {
                if self.trailer.is_some() {
                    return Err(AccountError::PushAfterAccountTrailer);
                }
                let currency = match &self.identifier {
                    Some(identifier) => identifier.currency,
                    None => return Err(AccountError::TransactionDetailBeforeAccountIdentifier),
                };
                let detail = TransactionDetail::new(text, currency)
                    .map_err(AccountError::TransactionDetail)?;
                self.transaction_details.push(detail);
                Ok(())
            }
            "49" => {
                if self.trailer.is_some() {
                    return Err(AccountError::PushAfterAccountTrailer);
                }
                if self.identifier.is_none() {
                    return Err(AccountError::AccountTrailerBeforeAccountIdentifier);
                }
                let trailer = AccountTrailer::new(text).map_err(AccountError::AccountTrailer)?;
                self.trailer = Some(trailer);
                Ok(())
            }
            "88" => Err(AccountError::UnmergedContinuationRecord),
            other => Err(AccountError::UnrecognizedRecordCode(other.to_string())),
        }
    }

    /// Checks `AccountIdentifier::validate`'s rules on this account's 03
    /// record (if present), plus reconciles the 49 record's Account
    /// Control Total against the summed Amounts seen so far (only if both
    /// the 03 and 49 records are present — otherwise that check is simply
    /// skipped, not reported as a violation).
    pub fn validate(&self) -> Vec<AccountViolation> {
        let mut violations = Vec::new();

        if let Some(identifier) = &self.identifier {
            violations.extend(
                identifier
                    .validate()
                    .into_iter()
                    .map(AccountViolation::AccountIdentifier),
            );
        }

        if let (Some(identifier), Some(trailer)) = (&self.identifier, &self.trailer) {
            let summaries_sum: i64 = identifier.summaries.iter().filter_map(|s| s.amount).sum();
            let details_sum: i64 = self
                .transaction_details
                .iter()
                .filter_map(|d| d.amount)
                .map(|amount| amount as i64)
                .sum();
            let computed = summaries_sum + details_sum;
            if computed != trailer.account_control_total {
                violations.push(AccountViolation::ControlTotalMismatch {
                    expected: trailer.account_control_total,
                    computed,
                });
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_code::TypeCode;

    #[test]
    fn pushes_identifier_then_details_then_trailer_in_order() {
        let mut account = Account::new(CurrencyCode::Usd);
        account
            .push("03,0975312468,,010,500000,,,190,70000000,4,0/")
            .unwrap();
        account
            .push("16,165,1500000,1,DD1620,, DEALER PAYMENTS")
            .unwrap();
        account
            .push("16,115,10000000,S,5000000,4000000,1000000/")
            .unwrap();
        account.push("49,82000000,5/").unwrap();

        assert_eq!(
            account.identifier.as_ref().unwrap().customer_account_number,
            "0975312468"
        );
        assert_eq!(account.transaction_details.len(), 2);
        assert_eq!(account.trailer.unwrap().account_control_total, 82_000_000);
    }

    #[test]
    fn transaction_detail_before_identifier_errors() {
        let mut account = Account::new(CurrencyCode::Usd);
        assert_eq!(
            account.push("16,010,100/"),
            Err(AccountError::TransactionDetailBeforeAccountIdentifier)
        );
    }

    #[test]
    fn trailer_before_identifier_errors() {
        let mut account = Account::new(CurrencyCode::Usd);
        assert_eq!(
            account.push("49,100,1/"),
            Err(AccountError::AccountTrailerBeforeAccountIdentifier)
        );
    }

    #[test]
    fn duplicate_identifier_errors() {
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,123,,010,100/").unwrap();
        assert_eq!(
            account.push("03,456,,010,100/"),
            Err(AccountError::DuplicateAccountIdentifier)
        );
    }

    #[test]
    fn push_after_trailer_errors() {
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,123,,010,100/").unwrap();
        account.push("49,100,2/").unwrap();
        assert_eq!(
            account.push("16,010,100/"),
            Err(AccountError::PushAfterAccountTrailer)
        );
        assert_eq!(
            account.push("03,456,,010,100/"),
            Err(AccountError::PushAfterAccountTrailer)
        );
        assert_eq!(
            account.push("49,100,2/"),
            Err(AccountError::PushAfterAccountTrailer)
        );
    }

    #[test]
    fn unrecognized_and_unmerged_continuation_record_codes_error() {
        let mut account = Account::new(CurrencyCode::Usd);
        assert_eq!(
            account.push("02,123/"),
            Err(AccountError::UnrecognizedRecordCode("02".to_string()))
        );
        assert_eq!(
            account.push("88,text"),
            Err(AccountError::UnmergedContinuationRecord)
        );
    }

    #[test]
    fn malformed_lines_bubble_up_the_exact_underlying_error() {
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,123,,010,100/").unwrap();
        assert_eq!(
            account.push("16,010,-100/"),
            Err(AccountError::TransactionDetail(
                TransactionDetailError::InvalidAmount("-100".to_string())
            ))
        );
    }

    #[test]
    fn transaction_detail_currency_follows_the_resolved_account_currency() {
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,123,JPY,010,100/").unwrap();
        account.push("16,010,100/").unwrap();
        assert_eq!(
            account.identifier.as_ref().unwrap().currency,
            CurrencyCode::Jpy
        );
        assert_eq!(account.transaction_details[0].currency, CurrencyCode::Jpy);
    }

    #[test]
    fn transaction_detail_currency_falls_back_to_group_currency() {
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,123,,010,100/").unwrap();
        account.push("16,010,100/").unwrap();
        assert_eq!(account.transaction_details[0].currency, CurrencyCode::Usd);
    }

    #[test]
    fn validate_returns_empty_for_conformant_closed_account() {
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,0975312468,,190,70500000,4,0/").unwrap();
        account
            .push("16,165,1500000,1,DD1620,, DEALER PAYMENTS")
            .unwrap();
        account
            .push("16,115,10000000,S,5000000,4000000,1000000/")
            .unwrap();
        account.push("49,82000000,3/").unwrap();
        assert_eq!(account.validate(), vec![]);
    }

    #[test]
    fn validate_wraps_account_identifier_violations() {
        // 010 (Opening Ledger) is Status-level; a present Item Count violates the spec.
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,123,,010,500000,4,0/").unwrap();
        assert_eq!(
            account.validate(),
            vec![AccountViolation::AccountIdentifier(
                AccountIdentifierViolation::ItemCountOnStatusTypeCode {
                    type_code: TypeCode::from("010"),
                }
            )]
        );
    }

    #[test]
    fn validate_flags_control_total_mismatch() {
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,0975312468,,190,70500000,4,0/").unwrap();
        account
            .push("16,165,1500000,1,DD1620,, DEALER PAYMENTS")
            .unwrap();
        account.push("49,99999999,2/").unwrap();
        assert_eq!(
            account.validate(),
            vec![AccountViolation::ControlTotalMismatch {
                expected: 99_999_999,
                computed: 72_000_000,
            }]
        );
    }

    #[test]
    fn validate_skips_control_total_check_without_a_trailer() {
        let mut account = Account::new(CurrencyCode::Usd);
        account.push("03,0975312468,,190,70500000,4,0/").unwrap();
        account
            .push("16,165,1500000,1,DD1620,, DEALER PAYMENTS")
            .unwrap();
        assert_eq!(account.validate(), vec![]);
    }
}
