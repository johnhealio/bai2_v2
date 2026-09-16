# Funds Type

## Primary Goal
Parse the BAI2 "Funds Type" composite field (03 and 16 records).

## Technical Spec

Implemented in `src/funds_type.rs`.

### Source

Transcribed from two places in the BAI Cash Management Balance Reporting
Specifications, Version 2 (`bai2_doc.pdf`):
- the "Funds Type" field description under the 03 (Account Identifier &
  Summary Status) and 16 (Transaction Detail) record formats, and
- the dedicated "FUNDS TYPE" entry in the Data Elements appendix, which
  defines it as a composite data element.

Unlike `TypeCode` (a flat code → description table), Funds Type is a
composite: a single leading code character that determines how many further
comma-delimited sub-fields follow it, and what they mean.

### Codes

| Code | Meaning | Sub-fields |
|---|---|---|
| `Z` (default) / blank | Unknown | none |
| `0` | Immediate availability | none |
| `1` | One day availability | none |
| `2` | Two-or-more days availability | none |
| `V` | Value dated | value date (YYMMDD), optional value time (military) |
| `S` | Distributed availability, fixed form | immediate, one-day, two-or-more-days amounts |
| `D` | Distributed availability, itemized form | count, then that many (days, amount) pairs |

### Types

- `FundsType` — one variant per code above (`Unknown`, `Immediate`, `OneDay`,
  `TwoOrMoreDays`, `ValueDated`, `Distributed`, `DistributedDays`), carrying
  the parsed sub-fields as payload for `V`, `S`, and `D`.
- `Date` — raw `{ year, month, day }` as transcribed from a YYMMDD field. The
  spec defines no century-inference rule for the two-digit year, so none is
  applied here.
- `Time` — a raw military time (`0000`-`2400`), kept as the four digits
  rather than decomposed into hour/minute so the documented nonstandard
  `9999` end-of-day sentinel (mentioned three times in the spec) still fits.
- `ValueDate` — `{ date, time: Option<Time> }` for code `V`. `time` is `None`
  when defaulted by adjacent delimiters.
- `DistributedAvailability` — `{ immediate, one_day, two_or_more_days }`
  (signed `i64`) for code `S`.
- `DistributionDay` — `{ days: u32, amount: i64 }`, one per pair for code `D`.
- `FundsTypeError` — parse failures: unknown code, missing sub-field, bad
  date/time/integer, or a `D` distribution count that doesn't match the
  number of (days, amount) pairs present.

### `FundsType` API

- `FundsType::parse(fields: &[&str]) -> Result<(FundsType, usize), FundsTypeError>`
  — takes the already comma-split fields starting at the code character
  (`fields[0]`) and returns the parsed value plus how many leading entries of
  `fields` it consumed. There is no line/record tokenizer yet, so the caller
  is responsible for splitting the raw record on `,` first; this keeps
  `funds_type` decoupled from record parsing, which lands in a later phase.
- `code(&self) -> &'static str` — the single-character code the variant
  serializes as.

### Design notes / tradeoffs

- **No `Unknown`-for-garbage catch-all.** `TypeCode` has a genuine 000-999
  numeric space where most codes are simply undefined, so it maps anything
  not in Appendix A to `TypeCode::Unknown`. Funds Type's code space is a
  small fixed set of 7 literal characters the spec enumerates exhaustively
  (`Z 0 1 2 V S D`); `Z` (and blank) already *is* the spec's own "Unknown
  (default)" value. Anything else (`"X"`, `"foo"`) is malformed input, not a
  legitimate-but-undefined domain value, so it's a `FundsTypeError`, not a
  variant.
- **No date/time crate dependency.** `Date`/`Time` are hand-rolled raw
  structs rather than using `chrono` or `time`. The project currently has
  zero dependencies; adding one for two struct fields with no calendar
  arithmetic needed yet didn't seem worth it. Tradeoff: no leap-year/day
  validation beyond a 1-31/1-12 range check, and no century inference for the
  two-digit year (the spec doesn't define one anyway). If a later phase needs
  real date arithmetic (e.g. comparing As-of-Date against a value date), this
  will likely need revisiting — possibly in favor of a shared `Date` type
  across all record fields that use YYMMDD, rather than one local to Funds
  Type.
- **`parse` returns consumed-field count instead of an iterator/cursor.** The
  variable width of the `D` case (count-prefixed) means the caller needs to
  know how many fields to skip to reach whatever comes after Funds Type in
  the record; returning `usize` keeps this explicit without introducing a
  stateful field-cursor type before record parsing exists to need one.
