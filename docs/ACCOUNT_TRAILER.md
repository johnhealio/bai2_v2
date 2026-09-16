# Account Trailer — Record Type 49

## Goal

A struct, `AccountTrailer`, that parses a BAI2 49 (Account Trailer) record
— a fixed, fully-required two-field record with no composite sub-fields
and no repeating groups — into a well-typed Rust value, with a dedicated
error type for anything malformed.

## Source

The "49 – ACCOUNT TRAILER" record format in the BAI Cash Management
Balance Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed page
22).

## Record Layout

```
49,<account control total>,<number of records>/
```

| Field | Required? | Notes |
|---|---|---|
| Record Code | required | always the literal `49` |
| Account Control Total | required | signed integer; algebraic sum of every Amount field back to and including the preceding 03 record (03's own Amounts, and every 16/88 record's Amount between it and this trailer). Does **not** include Funds Type or Item Count amounts. Sign defaults to `+` if absent |
| Number of Records | required | unsigned integer; count of every physical record from the preceding 03 record through this 49 record, inclusive (03 + all 16/88 in between + this 49) |

Unlike every prior record in this crate, **there are no optional fields at
all** — the spec states "All fields are required" outright, so there's no
adjacent-delimiter-defaulting behavior to account for here.

## Data Model

```rust
pub struct AccountTrailer {
    pub account_control_total: i64,
    pub number_of_records: u32,
}

pub enum AccountTrailerError {
    WrongRecordCode(String),
    MissingAccountControlTotal,
    InvalidAccountControlTotal(String),
    MissingNumberOfRecords,
    InvalidNumberOfRecords(String),
}
```

## API

- `AccountTrailer::new(line: &str) -> Result<AccountTrailer, AccountTrailerError>`
  — parses one 49 record line. No `CurrencyCode` parameter, unlike
  `TransactionDetail::new`/`AccountIdentifier::new`
  (`docs/TRANSACTION_DETAIL.md`, `docs/ACCOUNT_IDENTIFIER.md`): Account
  Control Total is already a raw implied-decimal integer on the wire (the
  same encoding as every Amount it sums), so nothing about *parsing* it
  needs the currency — only interpreting/displaying its magnitude would,
  and that's the caller's concern using whatever `CurrencyCode` was in
  force for the account, exactly as already decided for control-total
  validation in `docs/TRANSACTION_DETAIL.md`.

## Decisions & Edge Cases

- **Both fields are genuinely mandatory — a blank/absent field is an
  error, not a default.** Every other record parsed so far
  (`FundsType`, `TransactionDetail`, `AccountIdentifier`) has fields that
  default to `None`/`Unknown` when blank. This record has none: "no sign"
  defaults the *sign* of Account Control Total to `+`, but the field
  itself, and Number of Records, must both be present. Use
  `MissingAccountControlTotal`/`MissingNumberOfRecords`, not
  `Option<i64>`/`Option<u32>`, so a blank field is a hard parse error here.
- **`+`/`-` need no special handling.** Rust's `i64::from_str` already
  accepts an optional leading `+` or `-` (verified directly, not assumed —
  `"+4350000".parse::<i64>()` and `"-500000".parse::<i64>()` both succeed),
  so `account_control_total_str.parse::<i64>()` alone handles the spec's
  signed-with-default-positive rule with no extra logic. Same approach
  `docs/ACCOUNT_IDENTIFIER.md` already uses for its signed Amount fields.
- **A single trailing `/` is stripped before splitting**, same as every
  other record in this crate — mostly for consistency and to tolerate a
  line handed in with or without it, though with no optional trailing
  fields here there's no risk of it polluting a data token the way it
  could for `TransactionDetail`/`AccountIdentifier`.
- **Extra fields beyond Number of Records are ignored, not an error.**
  Consistent with this crate's general leniency toward the *shape* of real
  files (see `docs/TRANSACTION_DETAIL.md`/`docs/ACCOUNT_IDENTIFIER.md`'s
  truncation notes) — parsing only looks at the first three comma-split
  fields (Record Code, Account Control Total, Number of Records) and
  doesn't validate that nothing follows them.
- **No continuation (88) handling.** The spec allows any record type to be
  continued by an 88 record, but 49's two fields are small, fixed-width
  integers that (per the general rule that a non-text field can't be split
  across records) would only ever need continuing if one of them were
  implausibly long — not worth accounting for here. If this ever proves
  necessary in practice, revisit it the same way `docs/ACCOUNT_IDENTIFIER.md`
  did: define the exact merge contract the caller must satisfy, don't
  parse physical lines directly.

## Testing Expectations

- The spec's own sample (`49,18650000,3/`) parses to
  `AccountTrailer { account_control_total: 18_650_000, number_of_records: 3 }`.
- A negative total with an explicit sign (`49,-18650000,3/`) and a
  positive total with an explicit `+` (`49,+18650000,3/`) both parse
  correctly.
- A blank Account Control Total or Number of Records (`49,,3/`,
  `49,18650000,/`) produces the specific `Missing...` error, not a silent
  default.
- A non-numeric value in either field produces the specific
  `Invalid...Total`/`Invalid...Records` error, naming the offending value.
- The wrong record code produces `WrongRecordCode`.
- A line with extra trailing fields after Number of Records still parses
  successfully, confirming the deliberate leniency there.

## Codegen Note

No large data table here, same as `TransactionDetail` and
`AccountIdentifier` — this module is hand-written, not generated from a
spec appendix.
