# Group Header — Record Type 02

## Goal

A struct, `GroupHeader`, that parses a BAI2 02 (Group Header) record — the
record that begins a group of accounts sharing one originator and one
As-of-Date — into a well-typed Rust value, with `GroupStatus` and
`AsOfDateModifier` as public enums in the same file, and a dedicated error
type for anything malformed.

## Source

The "02 – GROUP HEADER" record format in the BAI Cash Management Balance
Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed pages 13-14),
plus the "GROUP STATUS" and "AS-OF-DATE MODIFIER" entries in the Data
Elements appendix (printed pages 26, 32) for each code's precise meaning.

## Record Layout

```
02,<ultimate receiver id>,<originator id>,<group status>,<as-of-date>,<as-of-time>,<currency code>,<as-of-date modifier>/
```

| Field | Required? | Notes |
|---|---|---|
| Record Code | required | always the literal `02` |
| Ultimate Receiver Identification | optional | alphanumeric; final receiver of this group's data; must not contain `/` |
| Originator Identification | required | alphanumeric; the sender; must not contain `/` |
| Group Status | required | single digit, `1`-`4` — see `GroupStatus` below. Not labeled optional, so unlike every other single-digit code field in this crate so far, a blank value here is a parse error, not a default (same category as `AccountTrailer`'s two fields, `docs/ACCOUNT_TRAILER.md`) |
| As-of-Date | required | `YYMMDD`; same raw, no-century-inference encoding as `FundsType`'s `Date` (`docs/FUND_TYPE.md`) |
| As-of-Time | optional | military format (`0000`-`2400`, or the documented nonstandard `9999` end-of-day sentinel); same encoding as `FundsType`'s `Time` |
| Currency Code | optional | **defaults to a fixed `USD`** — unlike the 03 record's Currency Code, which defaults to *this* record's Currency Code (`docs/ACCOUNT_IDENTIFIER.md`). This is the actual source of the "USD" default that phase's doc references |
| As-of-Date Modifier | optional | single digit, `1`-`4` — see `AsOfDateModifier` below; "for reference only," does not affect processing |

## Data Model

```rust
/// How data in this group of accounts is to be processed. See the
/// "GROUP STATUS" Data Element for each code's full processing semantics
/// (not reproduced here — this module only parses the code, it doesn't
/// implement update/delete/correct processing).
pub enum GroupStatus {
    Update,     // 1 — most transmissions; contains an 03 record per account
    Deletion,   // 2
    Correction, // 3
    TestOnly,   // 4
}

/// Distinguishes same-day from previous-day data, and interim from final
/// data. "For identification only and does not affect processing."
pub enum AsOfDateModifier {
    InterimPreviousDay, // 1
    FinalPreviousDay,   // 2
    InterimSameDay,     // 3
    FinalSameDay,       // 4
}

pub struct GroupHeader {
    pub ultimate_receiver_identification: Option<String>,
    pub originator_identification: String,
    pub group_status: GroupStatus,
    pub as_of_date: Date,                          // reused from funds_type
    pub as_of_time: Option<Time>,                  // reused from funds_type
    pub currency: CurrencyCode,                    // resolved: this field, or USD if blank
    pub as_of_date_modifier: Option<AsOfDateModifier>,
}

pub enum GroupHeaderError {
    WrongRecordCode(String),
    InvalidUltimateReceiverIdentification(String), // contains '/'
    MissingOriginatorIdentification,
    InvalidOriginatorIdentification(String),       // contains '/'
    MissingGroupStatus,
    InvalidGroupStatus(String),
    MissingAsOfDate,
    InvalidAsOfDate(String),
    InvalidAsOfTime(String),
    InvalidAsOfDateModifier(String),
}
```

## API

- `GroupHeader::new(text: &str) -> Result<GroupHeader, GroupHeaderError>` —
  parses one 02 record line. No external parameters needed (unlike
  `AccountIdentifier::new`/`TransactionDetail::new`, which take a currency
  fallback/threading parameter) — this record is the actual *source* of
  group-level currency, not a consumer of it.

## Decisions & Edge Cases

- **Reuse `Date` and `Time` from `funds_type`, don't redefine them.**
  `docs/FUND_TYPE.md` already anticipated this: "If a later phase needs
  real date arithmetic... possibly in favor of a shared `Date` type across
  all record fields that use YYMMDD, rather than one local to Funds Type."
  As-of-Date and As-of-Time are exactly that later need — same raw
  `{year, month, day}`/4-digit-military-time encoding, same lack of
  century inference, same `9999` sentinel handling for the time field.
  This requires `funds_type`'s currently-private `parse_date`/`parse_time`
  helpers to become `pub(crate)` so `group_header` can call them without
  duplicating the validation logic — a small, deliberate visibility change
  to make when implementing, not a sign the original design was wrong.
- **Group Status and As-of-Date Modifier are parsed by value, not
  processing semantics.** The spec's "GROUP STATUS" Data Element goes on
  at length about what a receiving system should *do* for each status
  (post updates, delete matching records, etc.) — none of that belongs in
  this module. `GroupHeader::new` only turns `"1"`..`"4"` into the right
  enum variant; acting on that value is entirely the caller's concern.
- **Group Status has no default — a blank value is `MissingGroupStatus`,
  not a variant.** It's one of the three fields in this record *not*
  labeled optional (with Originator Identification and As-of-Date), so
  this follows the same "genuinely mandatory" treatment `AccountTrailer`
  established (`docs/ACCOUNT_TRAILER.md`) rather than the "blank means
  default/Unknown" treatment every *optional* code field in this crate
  gets. Same reasoning applies to a malformed (non-`1`-`4`) value:
  `InvalidGroupStatus`, not a fallback variant — the code space is a small,
  fully-enumerated set (4 values), same category as `FundsType`'s.
- **As-of-Date Modifier, being optional, defaults to `None`, not a
  `GroupStatus`-style hard requirement — but an out-of-range value is
  still an error, not a fallback.** Same "small closed set, anything else
  is malformed input" reasoning as Group Status, just layered under
  `Option` since this field actually is defaultable.
- **Currency Code's default is a fixed `CurrencyCode::Usd`, not a
  parameter.** Contrast with `AccountIdentifier::new`'s `group_currency`
  parameter (`docs/ACCOUNT_IDENTIFIER.md`) — that parameter's *value*
  ultimately traces back to here. `GroupHeader` sits above
  `AccountIdentifier` in the record hierarchy, so it has nothing to fall
  back to except the spec's own hardcoded default.
- **Ultimate Receiver Identification and Originator Identification are
  validated only for `/`**, same reasoning as `AccountIdentifier`'s
  Customer Account Number (`docs/ACCOUNT_IDENTIFIER.md`) — `,` is already
  excluded by construction (comma-split), `/` is the one delimiter
  character that could still slip into a plain alphanumeric field.
- **One trailing `/` stripped unconditionally before splitting**, and
  **trailing optional fields (As-of-Time, Currency Code, As-of-Date
  Modifier) may be truncated off the line entirely**, not just left blank
  via adjacent commas — both are the same conventions already established
  in every other record module (`docs/TRANSACTION_DETAIL.md`,
  `docs/ACCOUNT_IDENTIFIER.md`, `docs/ACCOUNT_TRAILER.md`).

## Testing Expectations

- The spec's own sample (`02,031001234,122099999,1,040620,2359,,2/`)
  parses to `ultimate_receiver_identification: Some("031001234")`,
  `originator_identification: "122099999"`, `group_status:
  GroupStatus::Update`, `as_of_date` matching `040620`, `as_of_time`
  matching `2359`, `currency: CurrencyCode::Usd` (defaulted via the blank
  field), `as_of_date_modifier: Some(AsOfDateModifier::FinalPreviousDay)`.
- A record with Ultimate Receiver Identification, As-of-Time, Currency
  Code, and As-of-Date Modifier all blank/absent still parses, with each
  resolving to its documented default (`None`/`None`/`Usd`/`None`).
- An explicit non-USD Currency Code (e.g. `JPY`) overrides the default.
- `9999` and `2400` are both accepted as valid As-of-Time values; `2401`
  is rejected as `InvalidAsOfTime`.
- A blank Group Status or As-of-Date produces `MissingGroupStatus`/
  `MissingAsOfDate`; an out-of-range Group Status (e.g. `5`) or malformed
  As-of-Date produces `InvalidGroupStatus`/`InvalidAsOfDate` naming the
  offending value.
- An out-of-range As-of-Date Modifier (e.g. `5`) produces
  `InvalidAsOfDateModifier`, distinct from it simply being absent
  (`None`).
- The wrong record code produces `WrongRecordCode`; a `/` embedded in
  Ultimate Receiver Identification or Originator Identification produces
  the corresponding `Invalid...Identification` error.

## Codegen Note

No large data table here, same as every record module so far except
`TypeCode`/`CurrencyCode` — this module is hand-written, not generated
from a spec appendix.
