# Funds Type

## Goal

A type, `FundsType`, that parses the BAI2 "Funds Type" composite field (found
in the 03 Account Identifier/Summary Status record and the 16 Transaction
Detail record) from its already comma-split sub-fields into a well-typed
Rust value, with a dedicated error type for anything malformed.

## Source

Transcribed from two places in the BAI Cash Management Balance Reporting
Specifications, Version 2 (`bai2_doc.pdf`):

- the "Funds Type" field description under the 03 and 16 record formats, and
- the dedicated "FUNDS TYPE" entry in the Data Elements appendix, which
  defines it as a composite data element.

Funds Type is not a flat code → description table like `TypeCode`. It's a
composite: a single leading code character that determines how many further
comma-delimited sub-fields follow it, and what they mean:

| Code | Meaning | Sub-fields |
|---|---|---|
| `Z` (default) / blank | Unknown | none |
| `0` | Immediate availability | none |
| `1` | One day availability | none |
| `2` | Two-or-more days availability | none |
| `V` | Value dated | value date (YYMMDD), optional value time (military, `0000`-`2400`, or the nonstandard `9999` end-of-day sentinel) |
| `S` | Distributed availability, fixed form | immediate, one-day, two-or-more-days amounts (each may be signed) |
| `D` | Distributed availability, itemized form | count, then that many `(days, amount)` pairs |

To regenerate this module from scratch: encode this table directly (it's
short and exhaustive — the spec defines no other codes), then apply the
rules below.

## Data Model

```rust
pub struct Date { pub year: u8, pub month: u8, pub day: u8 }   // raw YYMMDD, no century inference
pub struct Time(pub u16);                                       // raw 4-digit military time, incl. 9999 sentinel
pub struct ValueDate { pub date: Date, pub time: Option<Time> }  // code V
pub struct DistributedAvailability { pub immediate: i64, pub one_day: i64, pub two_or_more_days: i64 } // code S
pub struct DistributionDay { pub days: u32, pub amount: i64 }   // one pair, code D

pub enum FundsType {
    Unknown,                              // Z / blank
    Immediate,                            // 0
    OneDay,                               // 1
    TwoOrMoreDays,                        // 2
    ValueDated(ValueDate),                // V
    Distributed(DistributedAvailability), // S
    DistributedDays(Vec<DistributionDay>),// D
}

pub enum FundsTypeError {
    Empty,
    UnknownCode(String),
    MissingField { code: char, field: &'static str },
    InvalidDate(String),
    InvalidTime(String),
    InvalidInteger { field: &'static str, value: String },
    DistributionCountMismatch { expected: u32, found: usize },
}
```

## API

- `FundsType::parse(fields: &[&str]) -> Result<(FundsType, usize), FundsTypeError>`
  — takes the record's fields, already comma-split, starting at the Funds
  Type code character (`fields[0]`). Returns the parsed value plus how many
  leading entries of `fields` it consumed (1 for the no-sub-field codes, 3
  for `V`, 4 for `S`, `2 + 2*count` for `D`), so a record-level parser (e.g.
  `TransactionDetail`, see `docs/TRANSACTION_DETAIL.md`) knows how far to
  advance before parsing whatever field comes next. There is no
  line/record tokenizer in this module — splitting a raw record line on `,`
  is the caller's job — since that logic is shared across every record type
  and belongs one layer up, not duplicated here.
- `code(&self) -> &'static str` — the single-character code the variant
  serializes as (`Z 0 1 2 V S D`).

## Decisions & Edge Cases

- **No `Unknown`-for-garbage catch-all.** Contrast with `TypeCode`
  (`docs/TYPE_CODE.md`), which has a genuine 000-999 numeric space where most
  values are simply undefined, and so maps anything unrecognized to
  `TypeCode::Unknown`. Funds Type's code space is 7 literal characters the
  spec enumerates exhaustively; `Z` (and blank) already *is* the spec's own
  "Unknown (default)" value. Anything else (`"X"`, `"foo"`) is malformed
  input, not a legitimate-but-undefined domain value — hence
  `FundsTypeError::UnknownCode`, not a fallback variant.
- **`V`'s time sub-field may be entirely absent, not just blank.** A record
  can end right after the date (fewer trailing fields than the format
  nominally has) as well as have the time field present-but-empty
  (`,,`/defaulted by adjacent delimiters). Both must parse to `time: None`.
- **`9999` is accepted as a time value** (a documented nonstandard
  end-of-day sentinel used by some processors) even though it's outside the
  otherwise-valid `0000`-`2400` range; anything else outside that range is
  `InvalidTime`.
- **`S`'s three amounts may be signed.** Unlike `TypeCode`/`CurrencyCode`
  fields, these are `i64`, not unsigned — the spec doesn't restrict their
  sign the way it does for e.g. a 16-record's top-level Amount field (see
  `docs/TRANSACTION_DETAIL.md`).
- **`D`'s declared count and actual pair count must match.** If `fields`
  runs out before `count` pairs are read, that's `MissingField` (a specific
  pair's sub-field wasn't there at all); if parsing succeeds but the
  resulting `Vec` length still doesn't equal `count`, that's
  `DistributionCountMismatch`. In practice the current implementation can
  only hit the former (it stops reading as soon as a field is missing), but
  both variants exist so a future refactor that reads pairs differently
  still has a place to report a mismatch.
- **Trailing fields beyond what a code consumes belong to the caller.**
  `FundsType::parse` never looks past the fields it needs; the returned
  consumed-count is how the caller knows where Funds Type ends, not where
  the record ends.
- **No date/time crate dependency.** `Date`/`Time` are hand-rolled rather
  than using `chrono`/`time` — the project has zero dependencies, and two
  raw struct fields with no calendar arithmetic didn't seem worth adding
  one for. Tradeoff: no leap-year/day validation beyond a `1..=12`/`1..=31`
  range check, and no century inference for the two-digit year (the spec
  defines none). If a later phase needs real date arithmetic (e.g. comparing
  an As-of-Date against a value date), this will likely need revisiting —
  possibly toward a `Date` type shared across every record field that uses
  YYMMDD, rather than one local to Funds Type.

## Testing Expectations

- Every no-sub-field code (`Z`, `""`, `0`, `1`, `2`) parses correctly with
  `consumed == 1`.
- `V` parses correctly with a time present, with the time field blank, and
  with the time field entirely absent (all three → `consumed == 3`,
  `time: None` for the latter two).
- `2400` and the `9999` sentinel are both accepted as valid times; `2401` is
  rejected.
- `S` parses correctly, including with a negative amount.
- `D` parses correctly with multiple pairs, with zero pairs (`consumed ==
  2`), and with trailing fields present beyond the declared count (those
  extra fields must not be consumed).
- Missing a required sub-field (e.g. `S` with only 2 of its 3 amounts, or
  `D` short of its declared pair count) produces the specific
  `MissingField` variant naming the field.
- An unrecognized code character produces `UnknownCode`; empty input
  produces `Empty`.
