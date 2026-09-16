# Account — Aggregate of Record Types 03, 16, and 49

## Goal

A struct, `Account`, that assembles the three record types belonging to one
BAI2 account — one `AccountIdentifier` (03), zero or more `TransactionDetail`
(16), and one `AccountTrailer` (49) — via an incremental `push` method that
takes each record's already-merged text in file order, enforces the
spec-mandated 03 → 16* → 49 sequence, and provides Account-level validation
on top of what `AccountIdentifier::validate` already checks.

This is the first "assembles a stream of records" phase, sitting one level
above `AccountIdentifier` (`docs/ACCOUNT_IDENTIFIER.md`), `TransactionDetail`
(`docs/TRANSACTION_DETAIL.md`), and `AccountTrailer`
(`docs/ACCOUNT_TRAILER.md`), all of which it reuses directly.

## Source

The "FILE LAYOUT" section of the BAI Cash Management Balance Reporting
Specifications, Version 2 (`bai2_doc.pdf`, printed pages 8-9), which
defines the required per-account record order (`03`, then zero or more
`16`, then `49`, with "There must be one 49 record for each 03 record" and
"All 16 and 88 records between the 03 record and the 49 record refer to
the account identified in the 03 record") — plus the 49 record's own
definition of Account Control Total as the algebraic sum of every Amount
field back to the account's 03 record (`docs/ACCOUNT_TRAILER.md`), which
this module's validation checks.

## Scope note: 88 (Continuation) records are not handled here

**`push` does not merge 88 continuation lines onto a preceding record —
that's still the caller's job, per the contract `AccountIdentifier`
already documents (`docs/ACCOUNT_IDENTIFIER.md`'s "Input contract"
section) and which applies equally to a multi-line 16 record's `Text`
field (`docs/TRANSACTION_DETAIL.md`).** A record code of `88` passed to
`push` on its own is rejected with a dedicated error
(`UnmergedContinuationRecord`) rather than silently mishandled, so the gap
is loud, not latent. Folding 88 lines into whichever 03/16 record precedes
them is a natural fit for a *future* record-stream assembler that reads
raw physical lines (this module works at the level of "one already-merged
logical record's text per call"), but isn't built yet — flagging this
explicitly since it means `Account` can't yet correctly consume a real
multi-physical-line file end to end.

## Data Model

```rust
pub struct Account {
    pub identifier: Option<AccountIdentifier>,
    pub transaction_details: Vec<TransactionDetail>,
    pub trailer: Option<AccountTrailer>,
    // private: captured at construction, used to resolve the 03 record's
    // Currency Code field and (via the resolved AccountIdentifier.currency)
    // every subsequent 16 record's Amount — see "Currency resolution" below.
    group_currency: CurrencyCode,
}

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
    /// The record code was "88" — see "Scope note" above.
    UnmergedContinuationRecord,
    AccountIdentifier(AccountIdentifierError),
    TransactionDetail(TransactionDetailError),
    AccountTrailer(AccountTrailerError),
}

pub enum AccountViolation {
    /// Wraps a violation `AccountIdentifier::validate` found in this
    /// account's 03 record.
    AccountIdentifier(AccountIdentifierViolation),
    /// The 49 record's Account Control Total doesn't equal the algebraic
    /// sum of every Amount field in the 03 record and every 16 record
    /// pushed so far.
    ControlTotalMismatch { expected: i64, computed: i64 },
}
```

## API

- `Account::new(group_currency: CurrencyCode) -> Account` — always
  succeeds; returns an empty account (`identifier: None,
  transaction_details: vec![], trailer: None`) ready for `push`.
  `group_currency` is the enclosing Group Header's Currency Code (not
  known by any type in this crate yet, since the Group Header record
  itself isn't implemented — the caller supplies it directly for now).
- `Account::push(&mut self, text: &str) -> Result<(), AccountError>` —
  inspects `text`'s record code (the substring before its first `,`) to
  decide which of `AccountIdentifier::new`, `TransactionDetail::new`, or
  `AccountTrailer::new` to call, enforces ordering (see "Order enforcement"
  below), and stores the result. Errors from the underlying `::new` calls
  are wrapped, not swallowed or re-interpreted.
- `Account::validate(&self) -> Vec<AccountViolation>` — see "Account-level
  validation" below.

## Decisions & Edge Cases

- **Order enforcement is a small state machine over
  `(identifier, trailer)`, not a position counter.** Valid transitions:
  no identifier yet + `03` → sets identifier; identifier present, no
  trailer yet + `16` → appends to `transaction_details`; identifier
  present, no trailer yet + `49` → sets trailer (closes the account).
  Everything else pushed in the wrong slot is one of the specific
  `AccountError` ordering variants above — there's deliberately no single
  generic `OutOfOrder` variant, so a caller can `match` on exactly what
  went wrong.
- **Currency resolution: `group_currency` feeds the 03 record; the 03
  record's *resolved* currency feeds every 16 record — not
  `group_currency` again.** `AccountIdentifier::new` already resolves "this
  record's own Currency Code field, or the group's if blank" into
  `AccountIdentifier.currency` (`docs/ACCOUNT_IDENTIFIER.md`). Once that's
  known, it — not the raw `group_currency` — is what every subsequent
  `TransactionDetail::new` call is given, since a 16 record has no
  currency field of its own and inherits whatever the account actually
  ended up denominated in, which may differ from the group's default if
  the 03 record overrode it.
- **`group_currency` is captured once at `new`, not re-passed to every
  `push` call.** Pure ergonomics — the caller already knows the group's
  currency before it starts pushing an account's records, so there's no
  reason to make every call site thread it through again.
- **A record code of `88` gets its own error variant, not
  `UnrecognizedRecordCode`.** It genuinely is a record type `push` doesn't
  understand yet, but *why* is specific and actionable (see "Scope note"),
  so it deserves a distinguishable error rather than being lumped in with
  truly-unexpected record codes.
- **`push` never runs `AccountIdentifier::validate` or the control-total
  check itself.** Consistent with the "parse-then-let-you-check" decision
  already made for `AccountIdentifier` (`docs/ACCOUNT_IDENTIFIER.md`) —
  structural parsing and business-rule/conformance checking stay separate
  operations at every level, not just within one record type.

## Account-level validation

`validate()` checks two things, and is meaningful to call at any point
(not just once the account is closed with a trailer) — it simply reports
what it can given what's present so far:

1. **Every violation `AccountIdentifier::validate` finds**, if `identifier`
   is `Some`, wrapped as `AccountViolation::AccountIdentifier(..)`. Nothing
   new is computed here; this is `AccountIdentifier`'s own check, exposed
   at this level so a caller doesn't need to reach into `account.identifier`
   themselves to run it.
2. **Account Control Total reconciliation**, only if both `identifier` and
   `trailer` are `Some` (otherwise there's nothing to compare against, and
   this check is silently skipped — not reported as a violation of its
   own): sum every `Amount` in `identifier.summaries` (signed, `None`
   counted as `0`) plus every `TransactionDetail.amount` in
   `transaction_details` (unsigned, `None` counted as `0`, added as a
   positive value — per spec, 16 records' Amounts are always positive on
   the wire, and the spec's "algebraic sum of all Amount fields" doesn't
   direct converting them to negative based on the Type Code's debit/credit
   direction). Amounts from Funds Type sub-fields are explicitly excluded
   per spec ("does not include amounts reported in Funds Type ... fields").
   If this computed sum doesn't equal `trailer.account_control_total`,
   push `ControlTotalMismatch { expected: trailer.account_control_total,
   computed }`.

**Not validated: Number of Records.** `AccountTrailer.number_of_records`
is meant to count every *physical* record from the 03 record through the
49 record, including any 88 continuations — but this module doesn't track
physical record/line counts (see "Scope note" above), so there's no
reliable count to check it against yet. Revisit this once continuation
merging is handled by whatever assembles this module's input text.

## Testing Expectations

- Pushing `03` then two `16`s then `49`, in that order, succeeds at every
  step and produces a fully-populated `Account`.
- Pushing `16` or `49` before any `03` produces
  `TransactionDetailBeforeAccountIdentifier`/`AccountTrailerBeforeAccountIdentifier`.
- Pushing a second `03` produces `DuplicateAccountIdentifier`; pushing
  anything after `49` produces `PushAfterAccountTrailer`.
- Pushing a record code outside `03`/`16`/`49`/`88` produces
  `UnrecognizedRecordCode`; pushing `88` on its own produces
  `UnmergedContinuationRecord`.
- A malformed `03`/`16`/`49` line (wrong field, bad amount, etc.) produces
  the corresponding wrapped `AccountError` variant, with the underlying
  error preserved exactly as the sub-module would have returned it.
- A 16 record's `TransactionDetail.currency` matches the *account's*
  resolved currency (from the 03 record), not `group_currency`, when the
  03 record explicitly overrides Currency Code to something else —
  confirming resolution order, not just that some currency gets threaded
  through.
- `validate()` on a fully conformant, closed account returns `[]`; on one
  whose `AccountIdentifier` has a business-rule violation, returns it
  wrapped; on one whose trailer's total doesn't match the summed Amounts,
  returns `ControlTotalMismatch` with both the expected and computed
  values; on one still missing its trailer, never returns
  `ControlTotalMismatch` (nothing to compare against yet), even if it
  would otherwise mismatch once one is pushed.

## Codegen Note

No large data table here — this module is hand-written, not generated
from a spec appendix, same as `TransactionDetail`, `AccountIdentifier`,
and `AccountTrailer`.
