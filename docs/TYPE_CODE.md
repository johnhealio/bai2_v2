# Type Codes

## Goal

An enum, `TypeCode`, holding every BAI2 "Type Code" defined in Appendix A of
the spec, hardcoded in source (not loaded from a file/table at runtime), with
an `Unknown` fallback variant and an infallible `&str` parser.

## Source

Every code is transcribed from Appendix A ("Uniform BAI Balance Reporting
Type Codes and Type Code Ranges") of the BAI Cash Management Balance
Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed pages 46-68).
That appendix lists 469 defined type codes, one per row, each with:

- a 3-digit code (`000`-`999`, always zero-padded, leading zeros significant),
- a `Transaction` (`NA` / `DB` / `CR`) — whether the code applies to neither,
  a debit, or a credit,
- a `Level` (`Status` / `Summary` / `Detail`) — whether the code describes
  account status (03 record / 88 continuation), summarizes credit/debit
  activity (03 record / 88 continuation), or details one credit/debit (16
  record), and
- a description (e.g. "Opening Ledger").

To regenerate this module from scratch: transcribe all 469 rows of Appendix A
into a `(code, transaction, level, description)` table, then apply the rules
below.

## Data Model

```rust
pub enum Transaction { Na, Db, Cr }
pub enum Level { Status, Summary, Detail }

pub enum TypeCode {
    // one variant per Appendix A row, named per "Variant naming" below
    OpeningLedger,
    // ...
    Unknown, // any 3-digit code not in Appendix A
}
```

All 469 defined variants plus `Unknown` are written out as literal enum
variants — no macro, no data file, no runtime table load. This is a
deliberate spec requirement (not just the path of least resistance): the goal
is a self-contained enum a caller can exhaustively `match` on.

## API

- `impl From<&str> for TypeCode` — `TypeCode::from(code: &str) -> TypeCode`.
  Maps a 3-digit code string to its variant, or `TypeCode::Unknown` for
  anything not in Appendix A, including any string that isn't exactly 3 ASCII
  digits (wrong length, non-digit characters, empty). Infallible — there is
  no `Result`/error type, because "not a defined code" is itself a
  meaningful, expected value (`Unknown`), not malformed input.
- `code(self) -> &'static str` — the original 3-digit code string. `""` for
  `Unknown` (it has no single code — it stands for every undefined one).
- `description(self) -> &'static str` — the Appendix A description text,
  transcribed verbatim.
- `transaction(self) -> Option<Transaction>` — `None` for `Unknown`.
- `level(self) -> Option<Level>` — `None` for `Unknown`.

## Lookup Algorithm

`TypeCode::from` must be genuinely O(1), not just "a fixed number of string
comparisons that's fine at this scale." A 469-arm string-equality match
compiles to a real comparison chain (or a hash in the best case) — not a
single lookup. Instead:

1. Validate the input is exactly 3 ASCII digit bytes; anything else is
   `Unknown` immediately, no table access.
2. Parse the 3 digits into an index `0..=999` (`a*100 + b*10 + c`, each digit
   as its numeric value).
3. Index directly into a `const TYPE_CODE_TABLE: [TypeCode; 1000]`, a dense
   array with one entry per possible 3-digit value — `TypeCode::Unknown` for
   the ~531 undefined codes, the real variant for each of the 469 defined
   ones.

The 1000-entry table is written out as 1000 literal array entries (one per
line, with a `// NNN` comment) rather than built via a loop, since the
numeric code space is small enough (1000 entries, ~1KB) that a dense literal
table is still readable and needs no explanation for why a `const` (not
`static`) is fine at this size — contrast with `CurrencyCode`'s much larger,
sparser 26³ space (see `docs/CURRENCY_CODE.md`), which does need a
build-loop and a `static`.

## Decisions & Edge Cases

- **Variant naming: PascalCase of the description.** Code `010` "Opening
  Ledger" → `OpeningLedger`. This is derived, not free-form, so a
  from-scratch reimplementation should produce the same identifiers given
  the same descriptions.
- **Disambiguating identical descriptions.** A handful of descriptions repeat
  across the Debit/Credit code ranges (e.g. "Overdraft" appears as both a
  `CR` detail code and a `DB` detail code). When a PascalCase'd description
  would collide, disambiguate by appending — in this order, stopping as soon
  as the name is unique — the `Transaction` (`Credit`/`Debit`/`Na`), then the
  `Level`, then the numeric code itself.
- **Descriptions starting with a digit** (e.g. "0-Day Float") get an `N`
  prefix, since Rust identifiers can't start with a digit.
- **No century/format ambiguity** — codes are always exactly 3 digits,
  zero-padded; there's no separate "short code" form to worry about.
- **`Unknown` exists because the numeric space is large and mostly
  undefined** (531 of 1000 possible values). Contrast with `FundsType`
  (`docs/FUND_TYPE.md`), whose entire code space is 7 known characters and
  therefore uses an error type instead of an `Unknown` variant for anything
  else — the right choice depends on whether the undefined region of the
  code space is the common case (`TypeCode`) or a sign of malformed input
  (`FundsType`).

## Testing Expectations

- Every one of the 469 defined codes round-trips: `TypeCode::from(code) ==
  Variant` and `Variant.code() == code`.
- A representative set of malformed/undefined inputs (`""`, `"12"`,
  `"abc"`, `"999"`, and at least one other undefined 3-digit value) all
  parse to `Unknown`, and `Unknown.transaction()` / `Unknown.level()` are
  both `None`.

## Codegen Note

The 469 variants and their four match arms (`code`, `description`,
`transaction`, `level`) plus the 1000-entry lookup table and the round-trip
tests were generated from a CSV transcription of Appendix A by a one-off
Python script, not typed by hand — this guarantees every code in the
appendix is present and that all four accessors stay in lockstep with
`from`. The script itself isn't part of the repository (scratch tooling
only); `src/type_code.rs` is the source of truth going forward and should be
hand-edited for any future correction.
