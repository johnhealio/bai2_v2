# File Trailer — Record Type 99

## Goal

A struct, `FileTrailer`, that parses a BAI2 99 (File Trailer) record — a
fixed, fully-required three-field record with no composite sub-fields and
no repeating groups — into a well-typed Rust value, with a dedicated
error type for anything malformed. Structurally identical in shape to
`GroupTrailer` (`docs/GROUP_TRAILER.md`), just one level up the hierarchy.

## Source

The "99 – FILE TRAILER" record format in the BAI Cash Management Balance
Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed pages 23-24).

## Record Layout

```
99,<file control total>,<number of groups>,<number of records>/
```

| Field | Required? | Notes |
|---|---|---|
| Record Code | required | always the literal `99` |
| File Control Total | required | signed integer; algebraic sum of every 98 record's Group Control Total in this file. Sign defaults to `+` if absent |
| Number of Groups | required | unsigned integer; count of 02 records in this file |
| Number of Records | required | unsigned integer; total count of every record of every code in the file, including this 99 record, but excluding any device-oriented/transmission-protocol records (JCL, tape marks, etc. — outside this format entirely) |

Same as `AccountTrailer`/`GroupTrailer`: **there are no optional fields at
all** — the spec states "All fields are required" outright.

## Data Model

```rust
pub struct FileTrailer {
    pub file_control_total: i64,
    pub number_of_groups: u32,
    pub number_of_records: u32,
}

pub enum FileTrailerError {
    WrongRecordCode(String),
    MissingFileControlTotal,
    InvalidFileControlTotal(String),
    MissingNumberOfGroups,
    InvalidNumberOfGroups(String),
    MissingNumberOfRecords,
    InvalidNumberOfRecords(String),
}
```

## API

- `FileTrailer::new(line: &str) -> Result<FileTrailer, FileTrailerError>`
  — parses one 99 record line. No `CurrencyCode` parameter, same
  reasoning as `AccountTrailer`/`GroupTrailer`: the total is already a
  raw implied-decimal integer (the algebraic sum of already-resolved
  group totals), so nothing about *parsing* it needs a currency.

## Decisions & Edge Cases

This module makes exactly the same decisions `AccountTrailer`
(`docs/ACCOUNT_TRAILER.md`) and `GroupTrailer` (`docs/GROUP_TRAILER.md`)
already made:

- **All three data fields are genuinely mandatory.** A blank/absent File
  Control Total, Number of Groups, or Number of Records is a `Missing...`
  parse error, not a default.
- **`+`/`-` on File Control Total need no special handling** — Rust's
  `i64::from_str` already accepts an optional leading `+` or `-`.
- **A single trailing `/` is stripped before splitting**, for consistency,
  though (as with the other two trailers) there are no optional trailing
  fields here for a stray `/` to pollute.
- **Extra fields beyond Number of Records are ignored, not an error.**
- **No continuation (88) handling** — same reasoning as
  `AccountTrailer`/`GroupTrailer`: three small fixed-width integers are
  implausible to need continuing in practice.

## Testing Expectations

- The spec's own sample (`99,1215450000,4,36/`) parses to
  `FileTrailer { file_control_total: 1_215_450_000, number_of_groups: 4, number_of_records: 36 }`.
- A negative total with an explicit sign (`99,-1215450000,4,36/`) and an
  explicit `+` (`99,+1215450000,4,36/`) both parse correctly.
- A blank File Control Total, Number of Groups, or Number of Records each
  produce the specific `Missing...` error, not a silent default.
- A non-numeric value in any of the three fields produces the specific
  `Invalid...` error, naming the offending value.
- A negative Number of Groups or Number of Records (both documented as
  unsigned counts) produces the corresponding `Invalid...` error.
- The wrong record code produces `WrongRecordCode`.
- A line with extra trailing fields after Number of Records still parses
  successfully.

## Codegen Note

No large data table here, same as every hand-written record module so
far — `src/file_trailer.rs` is the source of truth, not generated from a
spec appendix.
