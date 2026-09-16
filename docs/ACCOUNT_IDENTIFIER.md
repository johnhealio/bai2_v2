# Account Identifier and Summary Status — Record Type 03

## Goal

A struct, `AccountIdentifier`, that parses a BAI2 03 (Account Identifier and
Summary Status) record — with any 88 (Continuation) records already merged
into it — into a well-typed Rust value, reusing `TypeCode` (`docs/TYPE_CODE.md`),
`FundsType` (`docs/FUND_TYPE.md`), and `CurrencyCode` (`docs/CURRENCY_CODE.md`),
with a dedicated error type for anything malformed.

## Source

The "03 – ACCOUNT IDENTIFIER AND SUMMARY STATUS" record format in the BAI
Cash Management Balance Reporting Specifications, Version 2 (`bai2_doc.pdf`,
printed pages 15-17), plus the "88 – CONTINUATION RECORD" section (printed
pages 20-21) for how continuation lines fold into this record's field list.

## Input contract: continuation records are already merged

**This module takes one already-assembled `&str` containing the full field
list, with any 88 records folded in — it does not itself see physical line
boundaries or the literal `88` record code.** That merging is a
record-stream/file-level concern living in a later phase, but this doc
specifies the *contract* that assembler must satisfy, since getting it
subtly wrong (naive line concatenation) would corrupt the field list:

- A record continued by 88 always ends its own physical line with `/`, even
  though more fields follow — per spec, this specific `/` does **not** mean
  "end of record," it means "insert exactly one field-delimiting comma
  here, then keep going with the 88 line's fields" (the 88 line's own
  literal leading `"88"` token is dropped, not treated as a field).
- The 03 record has no `Text`-like field that can be split mid-value across
  physical lines (unlike the 16 record) — the spec explicitly forbids
  splitting any non-text field across records ("If a nontext field is begun
  in one record, it must be completed in that record"). So the merge is
  always "concatenate whole fields," never "concatenate partial field
  content," which is what makes a flat, single comma-joined string a
  faithful and sufficient representation of the whole logical record.
- Concretely: `03,...,190/` followed by `88,500000,,,110,...` must become
  `...,190,500000,,,110,...` (the `/` + `88,` pair collapses to a single
  `,`) — not `...,190/88,500000,,,110,...` or similar naive concatenation.

Whatever assembles this string may leave a single trailing `/` on the final
result (or not) — this module tolerates either, the same way `TransactionDetail`
does (see `docs/TRANSACTION_DETAIL.md`).

## Record Layout

```
03,<customer account number>,<currency code>,[<type code>,<amount>,<item count>,<funds type>[,...]]*
```

| Field | Required? | Notes |
|---|---|---|
| Record Code | required | always the literal `03` |
| Customer Account Number | required | alphanumeric; leading zeros significant; must not contain `,` or `/` |
| Currency Code | optional | defaults to the *enclosing Group Header's* currency code, not a fixed value — parse via `CurrencyCode::from` |
| (repeating group, zero or more times) | | |
| ↳ Type Code | optional | 3-digit code, parse via `TypeCode::from`; identifies one status or summary value. A **blank Type Code ends the repeating group** — see "Repeating groups" below |
| ↳ Amount | optional | signed (`+`/`-`), unlike the 16 record's `Amount`. Status amounts may be `+` or `-` (default `+`); Summary amounts are documented as positive/unsigned-only (see "Business-rule validation" below) |
| ↳ Item Count | optional | integer, no implied decimal; documented as applying to Summary type codes only, and as required to be defaulted for Status type codes (see "Business-rule validation") |
| ↳ Funds Type | optional | composite field — parse via `FundsType::parse`, defaults to `Z`/`Unknown` when blank |

A single 03 record can report several different status/summary values for
the same account by repeating the `(Type Code, Amount, Item Count, Funds
Type)` group — e.g. ledger balance, available balance, total credits, and
total debits can all appear in one record. An 03 record that exists purely
to introduce a run of 16 (Transaction Detail) records, with no status or
summary data of its own, has *zero* repeating groups (spec example:
`03,5765432,,,,,/`).

## Data Model

```rust
pub struct AccountIdentifier {
    pub customer_account_number: String,
    pub currency: CurrencyCode,     // resolved: this record's field, or the group's, if defaulted
    pub summaries: Vec<SummaryStatus>,
}

pub struct SummaryStatus {
    pub type_code: TypeCode,
    pub amount: Option<i64>,        // signed; None if defaulted ("no amount reported")
    pub item_count: Option<u32>,    // None if defaulted ("unknown")
    pub funds_type: FundsType,
}

pub enum AccountIdentifierError {
    WrongRecordCode(String),
    MissingAccountNumber,
    InvalidAccountNumber(String),   // contains '/'
    InvalidAmount(String),
    InvalidItemCount(String),
    FundsType(FundsTypeError),
}

/// A business-rule violation `validate()` can find in an otherwise
/// syntactically well-formed `AccountIdentifier`. See "Business-rule
/// validation is opt-in, not enforced by `new`" below.
pub enum AccountIdentifierViolation {
    /// Item Count was present on a Status-level Type Code; the spec
    /// requires it be defaulted there (Item Count applies to Summary
    /// type codes only).
    ItemCountOnStatusTypeCode { type_code: TypeCode },
    /// A Summary-level Type Code had a negative Amount; the spec
    /// documents Summary amounts as positive/unsigned only.
    NegativeAmountOnSummaryTypeCode { type_code: TypeCode, amount: i64 },
    /// A Detail-level Type Code appeared in an 03 record; the spec says
    /// 03 records cannot report transaction detail (that's record 16's
    /// job).
    DetailTypeCodeInAccountIdentifier { type_code: TypeCode },
}
```

## API

- `AccountIdentifier::new(text: &str, group_currency: CurrencyCode) ->
  Result<AccountIdentifier, AccountIdentifierError>` — parses one fully
  merged 03 record (see "Input contract" above). `group_currency` is the
  enclosing Group Header's Currency Code, used only as the fallback when
  this record's own Currency Code field is blank — mirrors
  `TransactionDetail::new`'s `currency` parameter (`docs/TRANSACTION_DETAIL.md`),
  but here it's a *fallback default* rather than a value that's always
  just attached verbatim, since this record's own field can override it.
- `AccountIdentifier::validate(&self) -> Vec<AccountIdentifierViolation>` —
  checks the three business rules below across every entry in `summaries`.
  An empty `Vec` means the record is fully spec-conformant, not just
  syntactically well-formed; a caller that doesn't need strict conformance
  can simply not call this. See "Business-rule validation is opt-in" below
  for why `new` itself never fails on these.

## Decisions & Edge Cases

- **Repeating groups: a blank Type Code — not an unrecognized one — ends
  the loop.** `TypeCode::from("")` already returns `TypeCode::Unknown` (see
  `docs/TYPE_CODE.md`), but an *absent/empty* Type Code sub-field means "no
  further status/summary group is present here" and must produce zero
  entries in `summaries` for that slot, not a spurious
  `SummaryStatus { type_code: TypeCode::Unknown, .. }`. A *present but
  undefined* 3-digit code (garbage, not blank) still produces a real
  `SummaryStatus` entry with `type_code: TypeCode::Unknown` — the loop only
  stops on blank/absent, never on "recognized or not." This is the same
  distinction `TransactionDetail` draws between "field is blank" and "field
  is present but doesn't parse to anything recognized."
- **Trailing optional fields may be truncated off the string entirely**,
  same leniency as `TransactionDetail` (`docs/TRANSACTION_DETAIL.md`) — a
  group's Amount/Item Count/Funds Type, or an entire subsequent group, can
  simply not be present rather than spelled out with empty commas. Use
  bounds-checked access throughout, not a required minimum field count.
- **One trailing `/` stripped unconditionally before splitting**, for the
  same reason as `TransactionDetail`: a `/` glued directly onto the last
  present token (no comma before it) would otherwise pollute that token's
  parse (e.g. a Funds Type amount ending in `.../1000000/`).
- **Amount is signed `i64`, unlike the 16 record's unsigned `Option<u64>`.**
  The spec explicitly allows `+`/`-` here (default `+`), so this can't reuse
  `TransactionDetail`'s Amount type. This is the reconciliation point
  `docs/TRANSACTION_DETAIL.md` flagged as deferred — it's now clear the two
  Amount fields are genuinely different (signed vs. unsigned) by spec, not
  just an arbitrary implementation choice, so they should probably *stay*
  distinct types rather than be unified.
- **Customer Account Number is a `String`, not a numeric type.** Leading
  zeros are spec-significant ("`0087654` must not become `87654`"), so any
  numeric type would need explicit zero-padding logic to round-trip
  correctly; a `String` preserves the original digits (and any legitimate
  non-digit characters — the field is documented as alphanumeric) with no
  extra work. Validated only for the one explicit spec constraint (must not
  contain `/`; `,` is already excluded by construction, same reasoning as
  `TransactionDetail`'s reference-number fields).
- **Currency Code resolution happens once, at the top, before any
  repeating-group parsing.** All groups' Funds Type sub-amounts and each
  group's own Amount share this one resolved currency (per spec: "the
  currency code will determine the implied decimal" for every Amount in the
  record) — there's no per-group currency override.
- **Business-rule validation is opt-in, not enforced by `new`.** Three spec
  statements describe constraints on *values*, not on *format* — a record
  can parse cleanly and still violate one: Item Count "must be defaulted
  for Status type codes" (`None` whenever `type_code.level() ==
  Some(Level::Status)`); "Summary amounts may only be positive or unsigned"
  (a Summary-level group's `amount` should never be negative); "03 records
  cannot report transaction detail" (no group's `type_code.level()` should
  ever be `Some(Level::Detail)`). Decided: `new` never fails because of
  these — it parses whatever's structurally there, since real-world
  producers may not be strict about them and a parse failure here would
  make an otherwise-readable file unreadable. `validate()` checks all three
  and returns every violation found, so a caller that *does* want strict
  spec conformance can opt into checking it without it being forced on
  every caller. `type_code == TypeCode::Unknown` (an undefined 3-digit
  code, `level()` returns `None`) triggers none of these three checks —
  there's no known level to validate against.

## Testing Expectations

- A record with zero repeating groups (e.g. `03,5765432,,,,,/`, used only
  to introduce 16 records) parses with `summaries: vec![]`, not a
  one-element vec of `TypeCode::Unknown`.
- A record with multiple repeating groups (e.g. the spec's own
  `03,0975312468,,010,500000,,,190,70000000,4,0/`) parses each group into
  the right `SummaryStatus`, correctly advancing past a `Funds Type` of
  varying width (`0`/`1`/`2` consume 1 field; `S` consumes 4; `V` consumes
  2-3; `D` consumes `2 + 2*count`) before reading the next group's Type
  Code.
- A record whose Currency Code field is blank resolves to whatever
  `group_currency` was passed in; a record with its own explicit Currency
  Code overrides it — both observable via `.currency` on the result.
- A multi-line 03+88 example from the spec (e.g.
  `03,9876543210,,010,-500000,,,100,1000000,,,400,2000000,,,190/` merged
  with `88,500000,,,110,1000000,,,072,500000,,,074,500000,,,040/` and
  `88,-1500000,,/`, joined per the "Input contract" rule above) parses
  identically to how it would if it had fit on one physical line —
  confirming the merge contract, not just single-line parsing, is what
  gets tested.
- A signed Amount (`+4350000` and a bare, unsigned-but-still-`i64`
  `500000`, and a negative `-500000`) all parse correctly.
- `validate()` on a record with a Status-level group carrying a non-`None`
  Item Count returns `ItemCountOnStatusTypeCode`; on a Summary-level group
  with a negative Amount returns `NegativeAmountOnSummaryTypeCode`; on a
  group whose Type Code is Detail-level returns
  `DetailTypeCodeInAccountIdentifier`; on a fully conformant record returns
  an empty `Vec`.
- Malformed input (wrong record code, missing account number, an account
  number containing `/`, a non-numeric Amount or Item Count, an invalid
  Funds Type sub-field) each produce the specific, named
  `AccountIdentifierError` variant.

## Codegen Note

No large data table here, same as `TransactionDetail` — this module is
hand-written, not generated from a spec appendix.
