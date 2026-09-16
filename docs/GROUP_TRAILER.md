# Group Trailer — Record Type 98

## Goal

A struct, `GroupTrailer`, that parses a BAI2 98 (Group Trailer) record —
a fixed, fully-required three-field record with no composite sub-fields
and no repeating groups — into a well-typed Rust value, with a dedicated
error type for anything malformed. Structurally almost identical to
`AccountTrailer` (`docs/ACCOUNT_TRAILER.md`), just one field longer.

## Source

The "98 – GROUP TRAILER" record format in the BAI Cash Management Balance
Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed page 22).

## Record Layout

```
98,<group control total>,<number of accounts>,<number of records>/
```

| Field | Required? | Notes |
|---|---|---|
| Record Code | required | always the literal `98` |
| Group Control Total | required | signed integer; algebraic sum of every 03 record's Account Control Total in this group. Sign defaults to `+` if absent |
| Number of Accounts | required | unsigned integer; count of 03 records in this group |
| Number of Records | required | unsigned integer; count of every physical record in this group, inclusive: the 02 record, every 03/16/49/88 record, and this 98 record |

Same as `AccountTrailer`: **there are no optional fields at all** — the
spec states "All fields are required" outright.

## Data Model

```rust
pub struct GroupTrailer {
    pub group_control_total: i64,
    pub number_of_accounts: u32,
    pub number_of_records: u32,
}

pub enum GroupTrailerError {
    WrongRecordCode(String),
    MissingGroupControlTotal,
    InvalidGroupControlTotal(String),
    MissingNumberOfAccounts,
    InvalidNumberOfAccounts(String),
    MissingNumberOfRecords,
    InvalidNumberOfRecords(String),
}
```

## API

- `GroupTrailer::new(line: &str) -> Result<GroupTrailer, GroupTrailerError>`
  — parses one 98 record line. No `CurrencyCode` parameter, same reasoning
  as `AccountTrailer::new` (`docs/ACCOUNT_TRAILER.md`): Group Control
  Total is already a raw implied-decimal integer (the algebraic sum of
  already-resolved 03-record totals), so nothing about *parsing* it needs
  a currency.

## Decisions & Edge Cases

This module makes exactly the same decisions `AccountTrailer` already
made (`docs/ACCOUNT_TRAILER.md`), applied to one extra mandatory field:

- **All three data fields are genuinely mandatory.** A blank/absent Group
  Control Total, Number of Accounts, or Number of Records is a
  `Missing...` parse error, not a default — there's no optional-field
  defaulting behavior anywhere in this record.
- **`+`/`-` on Group Control Total need no special handling** — Rust's
  `i64::from_str` already accepts an optional leading `+` or `-`.
- **A single trailing `/` is stripped before splitting**, for consistency
  with every other record module, though (as with `AccountTrailer`) there
  are no optional trailing fields here for a stray `/` to pollute.
- **Extra fields beyond Number of Records are ignored, not an error**,
  same leniency as every other record module toward the shape of real
  files.
- **No continuation (88) handling** — same reasoning as `AccountTrailer`:
  three small fixed-width integers are implausible to need continuing in
  practice, and if that ever proves wrong, revisit with an explicit merge
  contract the way `AccountIdentifier` (`docs/ACCOUNT_IDENTIFIER.md`) and
  `Account` (`docs/ACCOUNT.md`) already do, rather than guessing.

## Testing Expectations

- The spec's own sample (`98,11800000,2,6/`) parses to
  `GroupTrailer { group_control_total: 11_800_000, number_of_accounts: 2, number_of_records: 6 }`.
- A negative total with an explicit sign (`98,-11800000,2,6/`) and an
  explicit `+` (`98,+11800000,2,6/`) both parse correctly.
- A blank Group Control Total, Number of Accounts, or Number of Records
  each produce the specific `Missing...` error, not a silent default.
- A non-numeric value in any of the three fields produces the specific
  `Invalid...` error, naming the offending value.
- A negative Number of Accounts or Number of Records (both documented as
  unsigned counts) produces the corresponding `Invalid...` error.
- The wrong record code produces `WrongRecordCode`.
- A line with extra trailing fields after Number of Records still parses
  successfully.

## Codegen Note

No large data table here, same as every hand-written record module so
far — `src/group_trailer.rs` is the source of truth, not generated from a
spec appendix.
