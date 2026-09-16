# File Header — Record Type 01

## Goal

A struct, `FileHeader`, that parses a BAI2 01 (File Header) record — the
record that begins an entire file, identifying sender, receiver, and file
structure — into a well-typed Rust value, with a dedicated error type for
anything malformed. No composite sub-fields, no repeating groups — basic
field-by-field parsing, same category as `GroupHeader`
(`docs/GROUP_HEADER.md`) rather than `AccountIdentifier`.

## Source

The "01 – FILE HEADER" record format in the BAI Cash Management Balance
Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed pages 12-13),
plus the Data Elements appendix entries for each field (`SENDER
IDENTIFICATION`, `RECEIVER IDENTIFICATION`, `FILE CREATION DATE`, `FILE
CREATION TIME`, `FILE IDENTIFICATION NUMBER`, `PHYSICAL RECORD LENGTH`,
`BLOCK SIZE`, `VERSION NUMBER`; printed pages 30, 36, 39, 39).

## Input contract

Same as every other record module: **`new` assumes any 88 (Continuation)
records have already been folded into `text`** — see
`docs/ACCOUNT_IDENTIFIER.md`'s "Input contract" section for the exact
merge rule this implies, and `docs/ACCOUNT.md`/`docs/GROUP.md` for where
that folding actually happens once it's built.

## Record Layout

```
01,<sender id>,<receiver id>,<file creation date>,<file creation time>,<file id number>,<physical record length>,<block size>,<version number>/
```

| Field | Required? | Notes |
|---|---|---|
| Record Code | required | always the literal `01` |
| Sender Identification | required | alphanumeric; transmitter of the file; must not contain `/` |
| Receiver Identification | required | alphanumeric; next recipient of the file; must not contain `/` |
| File Creation Date | required | `YYMMDD`; same encoding as `FundsType`'s `Date` (`docs/FUND_TYPE.md`) |
| File Creation Time | required | military format (`0000`-`2400`, or the documented `9999` sentinel); same encoding as `FundsType`'s `Time`. **Required here** — unlike every other Time-shaped field parsed so far (`GroupHeader`'s As-of-Time, `FundsType`'s value time), which are optional |
| File Identification Number | required | numeric; sender-assigned, must be unique per sender/receiver/creation-date combination (a uniqueness constraint this module has no way to check — it just parses the number) |
| Physical Record Length | optional | numeric; characters per physical record. Blank means variable-length records |
| Block Size | optional | numeric; physical records per block. Blank means variable block size |
| Version Number | required | numeric, one digit; "For this version, always 2" |

## Data Model

```rust
pub struct FileHeader {
    pub sender_identification: String,
    pub receiver_identification: String,
    pub file_creation_date: Date,           // reused from funds_type
    pub file_creation_time: Time,           // reused from funds_type; required, not Option
    pub file_identification_number: u32,
    pub physical_record_length: Option<u32>,
    pub block_size: Option<u32>,
    pub version_number: u32,
}

pub enum FileHeaderError {
    WrongRecordCode(String),
    MissingSenderIdentification,
    InvalidSenderIdentification(String),       // contains '/'
    MissingReceiverIdentification,
    InvalidReceiverIdentification(String),     // contains '/'
    MissingFileCreationDate,
    InvalidFileCreationDate(String),
    MissingFileCreationTime,
    InvalidFileCreationTime(String),
    MissingFileIdentificationNumber,
    InvalidFileIdentificationNumber(String),
    InvalidPhysicalRecordLength(String),
    InvalidBlockSize(String),
    MissingVersionNumber,
    InvalidVersionNumber(String),
}
```

## API

- `FileHeader::new(text: &str) -> Result<FileHeader, FileHeaderError>` —
  parses one already-merged 01 record. No external parameters needed —
  unlike `AccountIdentifier::new`/`Account::new`, nothing in this record
  depends on state from anywhere else; it's the very start of the file.

## Decisions & Edge Cases

- **Reuse `Date`/`Time` from `funds_type`, same as `GroupHeader`.** File
  Creation Date and File Creation Time get the exact same treatment
  `docs/GROUP_HEADER.md` already established for As-of-Date/As-of-Time —
  no new date/time logic, just `funds_type::parse_date`/`parse_time`
  (already `pub(crate)`).
- **File Creation Time is required, not `Option<Time>`.** Every other
  Time-shaped field parsed so far in this crate is optional (`GroupHeader`'s
  As-of-Time, `FundsType`'s `V` value time) — this is the first one that
  isn't. The field description has no "Optional." prefix, unlike As-of-Time's,
  so a blank value here is `MissingFileCreationTime`, not `None`.
- **"Numeric" spec fields become Rust integer types; "Alphanumeric" fields
  stay `String`.** This is the general mapping convention this crate has
  followed all along without stating it explicitly until now: File
  Identification Number, Physical Record Length, Block Size, and Version
  Number are all documented as "Numeric" in the Data Elements appendix and
  become `u32`s (parsed, not treated as opaque strings, even though only
  Physical Record Length/Block Size are ever used arithmetically) — Sender
  and Receiver Identification are "Alphanumeric" and stay `String`, same
  reasoning as `GroupHeader`'s Ultimate Receiver/Originator Identification.
- **Version Number is parsed, not validated against `2`.** The spec says
  "For this version, always 2," but that's a fact about the *current*
  spec version, not an enumerated set of meaningfully different values the
  way Group Status's `1`-`4` is (`docs/GROUP_HEADER.md`) — there's nothing
  to `match` into named variants. `new` parses whatever integer is there;
  rejecting anything other than literally `2` would make this module
  needlessly brittle against a future spec revision, and checking it isn't
  this module's job at all (no `validate()` method exists here, unlike
  `Account`/`Group` — the user's brief for this phase was explicitly "no
  sub-types, basic parse process," and there's no meaningful business rule
  here the way there was for Account/Group Control Totals).
- **File Creation Time's Data Element text says "`0001` through `2400`,"
  contradicting the very next sentence's "`0000` indicates the beginning
  of the day"** — the same kind of internal inconsistency already seen
  elsewhere in this spec (e.g. the EUR/BRL decimals issue,
  `docs/CURRENCY_CODE.md`). Low-stakes here (only excludes literal
  midnight, `0000`), so this module just reuses `parse_time`'s existing
  `0000`-`2400`-or-`9999` range unchanged rather than special-casing it.
- **Sender/Receiver Identification validated only for `/`**, same
  reasoning as every other plain identifier field in this crate (`,` is
  already excluded by construction).
- **One trailing `/` stripped unconditionally before splitting**, and
  **Physical Record Length/Block Size may be truncated off the line
  entirely** (not just left blank via commas) — both are the same
  conventions already established everywhere else in this crate.

## Testing Expectations

- The spec's own sample (`01,122099999,123456789,040621,0200,1,55,,2/`)
  parses to `sender_identification: "122099999"`,
  `receiver_identification: "123456789"`, `file_creation_date` matching
  `040621`, `file_creation_time` matching `0200`,
  `file_identification_number: 1`, `physical_record_length: Some(55)`,
  `block_size: None` (defaulted), `version_number: 2`.
- A record with Physical Record Length and Block Size both blank/absent
  still parses, with both resolving to `None`.
- `9999` and `2400` are both accepted as valid File Creation Time values;
  `2401` is rejected as `InvalidFileCreationTime`.
- A blank Sender/Receiver Identification, File Creation Date, File
  Creation Time, File Identification Number, or Version Number produces
  the corresponding `Missing...` error; a malformed (non-numeric, wrong
  length, etc.) value in any field produces the corresponding
  `Invalid...` error naming the offending value.
- A Version Number other than `2` (e.g. `3`) still parses successfully as
  `version_number: 3` — confirming it's parsed, not validated against a
  fixed expected value.
- The wrong record code produces `WrongRecordCode`; a `/` embedded in
  Sender or Receiver Identification produces the corresponding
  `Invalid...Identification` error.

## Codegen Note

No large data table here — hand-written, not generated from a spec
appendix, same as every record module except `TypeCode`/`CurrencyCode`.
