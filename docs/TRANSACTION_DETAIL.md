# Transaction Detail — Record Type 16

## Goal

A struct, `TransactionDetail`, that parses a single BAI2 16 (Transaction
Detail) record line into a well-typed Rust value, reusing `TypeCode`
(`docs/TYPE_CODE.md`) and `FundsType` (`docs/FUND_TYPE.md`) as field values,
with a dedicated error type for anything malformed.

## Source

The "16 – TRANSACTION DETAIL" record format in the BAI Cash Management
Balance Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed page
18-19), plus the general field-delimiter rules in "Free Format
Fields/Field Delimiters" and "End of Record" (printed pages 10-11).

## Record Layout

A 16 record has these fields, in order, comma-delimited:

| Field | Required? | Notes |
|---|---|---|
| Record Code | required | always the literal `16` |
| Type Code | required | 3-digit code, parse via `TypeCode::from`. See Appendix A. Type code `890` means "non-monetary information" — see below. |
| Amount | optional | see "Amount" below |
| Funds Type | optional | composite field — parse via `FundsType::parse`, defaults to `Z`/`Unknown` when blank |
| Bank Reference Number | optional | alphanumeric, originator-defined, unbounded length; must not contain `,` or `/` |
| Customer Reference Number | optional | same rules as Bank Reference Number; for type codes 474, 475, and 395 this is the check number |
| Text | optional | free-form; must not *begin* with `/`, but may contain `,` or `/` after the first character. **This is the last field and has no closing delimiter** — see "Text field is unbounded" below |

### Amount

Optional. **Always positive (unsigned)** — unlike `FundsType`'s `S`/`D`
sub-amounts, this field cannot be negative. Expressed without a decimal
point; the number of implied decimal places is determined by the currency
code in force for the enclosing Group Header or Account Identifier record
(hence `CurrencyCode`, `docs/CURRENCY_CODE.md`, is a required input to
parsing, not something this record can determine on its own). A blank/
defaulted field means "no amount is being reported" — this is a distinct
state from an amount of zero, so the field should be `Option<u64>`, not
`u64` with `0` doing double duty.

### Type code 890 (non-monetary)

`890` is a detail type code (neither debit nor credit) used to transmit
free-text information not tied to an actual transaction, e.g.:

```
16,890,,,,,detail reports will be delayed until 11:00 AM.
```

When Type Code is `890`, Amount and Funds Type are expected to be defaulted
(blank), and the Text field carries the message. Bank/Customer Reference
Number may be used as part of the message or also left defaulted. This
should parse the same way structurally as any other 16 record — `890` is
not a special case in the *parser*, just a documented convention for how
the fields tend to be populated. Don't add special-case branching on
`TypeCode` value inside `TransactionDetail::new`; if callers need to detect
"this is a non-monetary message record," they can check `type_code ==
TypeCode::ContainsNonMonetaryInformation` themselves.

### Text field is unbounded and may continue onto 88 records

Per the spec's "End of Record" rules: Text is the *last* field in a 16
record and has no terminating delimiter (not even `/`), because it's
allowed to contain commas and slashes itself. If a Text value is too long
for one physical line, it continues into one or more subsequent 88
(Continuation) records — parsing stops only when a non-continuation record
begins.

**This module's `new(line: &str, currency: CurrencyCode)` only parses a
single line and does not read ahead for continuation records.** That's a
deliberate scope boundary, not an oversight: continuation-record stitching
requires knowing about adjacent lines/records, which belongs in a
record-stream/file-level parser that doesn't exist yet, not in a
single-record struct. Document this explicitly in this module's doc
comments so it isn't mistaken for a bug later. A defaulted Text field (no
text at all) is indicated by trailing `,/` and needs no continuation
handling.

## Data Model

```rust
pub struct TransactionDetail {
    pub type_code: TypeCode,
    pub amount: Option<u64>,       // raw minor units, e.g. cents; None if defaulted
    pub currency: CurrencyCode,    // threaded through, not parsed from this record
    pub funds_type: FundsType,
    pub bank_reference_number: Option<String>,
    pub customer_reference_number: Option<String>,
    pub text: Option<String>,
}

pub enum TransactionDetailError {
    WrongRecordCode(String),       // fields[0] wasn't literally "16"
    MissingTypeCode,               // Type Code field was blank or absent
    InvalidAmount(String),         // Amount wasn't a valid non-negative integer
    FundsType(FundsTypeError),     // bubbled up from FundsType::parse
    InvalidReferenceNumber { field: &'static str, value: String }, // '/' in a reference number
    TextStartsWithSlash(String),
}
```

## API

- `TransactionDetail::new(line: &str, currency: CurrencyCode) ->
  Result<TransactionDetail, TransactionDetailError>` — parses one already-
  isolated 16 record line (no leading/trailing continuation handling).
  `currency` is attached to the result as `TransactionDetail.currency` so
  callers have `amount` and the currency it's denominated in together,
  without re-threading it from elsewhere — see "Currency's actual role"
  below for why this is *not* the same as needing `decimals()` to parse
  the integer itself.

## Decisions & Edge Cases

- **Currency's actual role: attached for the caller's benefit, not consumed
  by parsing.** The Amount field is "expressed without a decimal" — the raw
  digits on the line already *are* the integer minor-unit value, so
  extracting `amount: Option<u64>` never needs `CurrencyCode::decimals()`.
  `currency` is still a required `new()` parameter and a struct field so
  that `amount` and the currency it's denominated in travel together
  without the caller re-plumbing it from the enclosing Group/Account
  record — useful groundwork for a future display/decimal-conversion
  helper (deliberately not built yet, see below), not something the
  current parsing logic branches on.
- **A single leading `/` strip handles the whole "record may or may not
  end in Text" ambiguity.** Per spec, a record ending in Text has *no*
  trailing delimiter at all, while a record with Text defaulted (or with
  trailing optional fields entirely truncated off the line, which real
  sample records do — see below) ends in `/`. Since `/` never legitimately
  terminates a *populated* Text field this way, it's correct to
  unconditionally strip one trailing `/` from the whole line before
  splitting on `,` — this also fixes the more subtle problem that,
  without stripping it first, the `/` would otherwise be glued (no comma)
  onto whatever the actual last data token is (e.g. `...,1000000/` would
  make the last Funds Type amount parse as `"1000000/"`, not `"1000000"`).
- **Trailing optional fields may be truncated off the line entirely, not
  just left blank via adjacent commas.** The spec's own field-delimiter
  rules say a defaulted trailing field "cannot be left off but must be
  indicated by adjacent delimiters," but the spec's own Appendix D sample
  transmissions don't follow that — e.g. `16,115,10000000,S,5000000,
  4000000,1000000/` has no trailing commas at all for the omitted Bank
  Reference Number, Customer Reference Number, and Text. Parsing treats
  "no more fields present" identically to "fields present but blank": all
  of Funds Type (defaults to `Unknown`, not a `FundsTypeError::Empty`),
  Bank/Customer Reference Number, and Text resolve to their default/`None`
  when the line simply doesn't extend that far, via bounds-checked
  (`Vec::get`) access rather than requiring a minimum field count.
- **Text reconstruction after a naive full-line comma split.** Because
  Text may itself contain `,`, the line is still split on every `,` up
  front (simplest approach), and the *known* number of leading positional
  fields — fixed except for Funds Type's variable width, which
  `FundsType::parse`'s returned consumed-count supplies — tells us exactly
  where Text starts. Everything from that index onward is rejoined with
  `,`, exactly reconstructing the original substring including any
  embedded commas/slashes. A tail where every remaining element is empty
  (however many trailing empty commas there are) is treated as "no text,"
  not `Some("")` or `Some(",")`.
- **Reuse `TypeCode` and `FundsType` as-is; don't re-derive their parsing
  logic.** `TypeCode::from` is infallible (`Unknown` for anything
  undefined), so an unrecognized 3-digit Type Code in a 16 record is not,
  by itself, a parse error at this layer — it becomes
  `TypeCode::Unknown` and parsing continues. `FundsType::parse` *is*
  fallible; propagate its `FundsTypeError` (likely wrapped in
  `TransactionDetailError`) rather than duplicating its validation.
- **Amount's `u64` width vs. other Amount-shaped fields.** `FundsType`'s
  `S`/`D` sub-amounts are signed `i64` (they can go negative per spec);
  this field is unsigned per spec. If a later phase wants one uniform
  `Amount` newtype shared across every record that has one (03's Status
  amounts can be signed too), reconcile the signedness difference then —
  don't force this field to `i64` "for consistency" ahead of that decision.
- **Reference numbers and Text are validated for `,`/`/` content, not just
  passed through.** The spec forbids `,` or `/` anywhere in Bank/Customer
  Reference Number (both delimiters, so their presence would already have
  split the line wrong before this field is ever isolated — but a caller
  handing in a raw slice out of band should still get a clear error, not a
  silently-wrong value). Text is more permissive: `,`/`/` are allowed
  *after* the first character, only a *leading* `/` is invalid.
- **No control-total/validation logic belongs here.** Comparing summed
  Amounts against Account/Group/File trailer control totals is a
  file/batch-level concern (do it in raw integer units — the trailers are
  encoded the same implied-decimal way, so no decimal conversion is
  needed and floating-point rounding never enters the picture). This
  module's job stops at producing one correctly-typed `TransactionDetail`.
- **No decimal-conversion helper (e.g. `amount_as_decimal`) yet.** Nothing
  in this phase needs a human-readable/display value — `CurrencyCode`
  already exposes `decimals()` for whoever eventually wants one. Add a
  conversion helper only when there's an actual caller (display, export)
  that needs it, not preemptively.

## Testing Expectations

- A normal record with a plain (no-sub-field) Funds Type, e.g.
  `16,165,1500000,1,DD1620,, DEALER PAYMENTS`, parses every field correctly,
  including a Text value with no trailing delimiter.
- A record with an `S` Funds Type composite and every trailing optional
  field truncated off the line entirely (e.g.
  `16,115,10000000,S,5000000,4000000,1000000/`) parses correctly, with
  `bank_reference_number`/`customer_reference_number`/`text` all `None`.
- A record with a `D` Funds Type composite followed by populated Bank and
  Customer Reference Numbers parses correctly, and the consumed-field
  count correctly lands on the reference numbers rather than mistaking
  part of the `D` composite for them.
- A `890` record with Amount and Funds Type both defaulted (e.g.
  `16,890,,,,,detail reports will be delayed until 11:00 AM.`) parses with
  `amount: None`, `funds_type: FundsType::Unknown`, and the full message in
  `text`.
- Text containing embedded commas and a slash after its first character
  (e.g. `a/b,c,d`) round-trips exactly, confirming the split-then-rejoin
  reconstruction doesn't lose or misplace the embedded delimiters.
- A record with Amount entirely defaulted (blank, not zero) yields
  `amount: None`; a record with a real Amount value stores the same raw
  integer regardless of currency, with `currency` (and its `decimals()`)
  set to whatever was passed in for at least two different currencies
  (e.g. a 2-decimal and a 0-decimal one) — confirming `currency` is
  actually threaded through and attached, not silently dropped.
- Malformed input (wrong record code, missing Type Code, a negative or
  non-numeric Amount, an invalid Funds Type sub-field, a `/` inside a
  reference number, Text beginning with `/`) each produces the specific,
  named `TransactionDetailError` variant, not a generic failure.

## Codegen Note

Unlike `TypeCode`/`CurrencyCode`, there's no large data table here — this
module was written by hand, not generated from a spec appendix. There's
nothing to keep in sync via codegen; `src/transaction_detail.rs` is simply
the source of truth.
