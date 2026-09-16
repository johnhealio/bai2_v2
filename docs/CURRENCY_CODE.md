# Currency Codes

## Primary Goal
build an enum to store the currency types included in the BAI2 spec

include a "from" function with a &str input as currency code that returns the appropriate variant.
the enum should be able to provide the appropriate number of decimals for the currency type.
the enum should also return a description of the currency.
the enum can be a field value at the group and account levels.
it will also be used as input into the transaction detail parsing process to determine how to parse an amount.

Update this file with technical specs used to build the enum

## Technical Spec

Implemented in `src/currency_code.rs`.

### Source

Transcribed from Appendix B ("Currency Codes") of the BAI Cash Management
Balance Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed pages
69-83): a Country/Currency/Code table based on ISO 4217, plus a separate
"Implied Decimals" section listing the currency codes whose implied decimal
count isn't the 2-decimal default. All 181 codes are hard-coded as enum
variants — nothing is loaded from a file at runtime. The 5 country rows
listing "No Universal Currency" (`--`, e.g. Antarctica) are not real currency
codes and are omitted; multiple countries sharing one currency (e.g. Andorra,
Austria, Italy — all `EUR`) collapse to that currency's single variant.

### `CurrencyCode` API

- `CurrencyCode::from(code: &str) -> CurrencyCode` (via `impl From<&str> for
  CurrencyCode`) — maps a 3-letter code string to its variant, or
  `CurrencyCode::Unknown` for any code not in Appendix B (including a
  blank/defaulted field, or anything not exactly 3 uppercase ASCII letters).
  Implemented as a single array index into a 26^3-entry lookup table keyed by
  the code's base-26 value (`O(1)`, no string comparisons) — the same
  technique `TypeCode::from` uses for its numeric code space (see
  `docs/TYPE_CODE.md`).
- `code(self) -> &'static str` — the 3-letter code (`""` for `Unknown`).
- `description(self) -> &'static str` — the currency name from Appendix B.
- `decimals(self) -> u32` — the implied decimal places for Amount/Funds Type
  fields in this currency (2 by default, including for `Unknown`).

### Design notes / tradeoffs

- **Variant naming follows the code, not the currency name.** Unlike
  `TypeCode` (arbitrary numeric codes needing description-derived names),
  each `CurrencyCode` variant is named directly from its ISO code
  (`EUR` → `Eur`, `USD` → `Usd`). Codes are already unique identifiers, so
  this is simpler than name-derived variants and avoids `TypeCode`'s
  cross-code disambiguation problem entirely.
- **`Unknown` fallback, no `Result`/error type.** Matches this phase's spec
  ("a 'from' function ... that returns the appropriate variant") and mirrors
  `TypeCode`'s precedent rather than `FundsType`'s closed-set-with-error
  design: ISO 4217 is a large, evolving code space (new codes are added over
  time) rather than `FundsType`'s fixed 7-character enumeration, so treating
  an unrecognized code as "not (yet) defined" fits better than treating it as
  malformed input.
- **Sparse compile-time table build, not a dense literal array.** `TypeCode`
  writes its 1000-entry table out literally since the numeric code space is
  small and ~half populated. A currency code's 3-letter space is 26^3 =
  17,576 entries, of which only 181 are defined; writing that out literally
  would make the source file enormous for no runtime benefit. Instead,
  `CURRENCY_CODE_TABLE` is a `static` array built by a `const`-evaluated loop
  over a compact 181-entry `(code, variant)` list, compiling down to the same
  flat lookup table (confirmed via `cargo clippy`'s `large_const_arrays` lint,
  which is why the table is declared `static` rather than `const` — a `const`
  of this size gets copied at every use site instead of allocated once).
- **EUR and BRL decimals overridden to 2.** Appendix B's own "Implied
  Decimals" section lists `EUR` and `BRL` under "ZERO (0) Decimals", but only
  via what are clearly uncorrected pre-Euro entries (Andorra, Belgium, Italy,
  Luxemburg, and Spain reported under "Euro"/`EUR`; Brazil under "Brazilian
  Real"/`BRL`) — other Euro-using countries in the main table (Austria,
  France, Germany, ...) aren't listed as exceptions, and real-world EUR/BRL
  are both 2-decimal. Decided with the user to override these two to the
  correct 2-decimal value rather than transcribe the document's error, since
  `decimals()` silently feeding a wrong divisor into amount parsing has real
  financial consequences. The rest of the exceptions section (0 decimals for
  `XOF`/`XAF`/`JPY`/`KMF`/`XPF`; 1 for `MRO`; 3 for `BHD`/`EGP`/`IQD`/`JOD`/
  `KWD`/`LYD`/`MTL`/`OMR`/`TND`/`YDD`) is transcribed as printed.
- **`SDP` (3-decimal, per the exceptions list) has no corresponding
  variant.** The exceptions section lists Sudan's code as `SDP`, but the main
  Appendix B table lists Sudan's code as `SDD` — a discrepancy in the source
  document itself (Sudan has in fact used several historical currency codes:
  `SDP`, then `SDD`, then today's `SDG`). Rather than guess which code the
  3-decimal note was meant to apply to, `SDP` is simply absent from the
  generated variants and `Sdd` keeps the 2-decimal default.
- **No codegen script committed**, same as `TypeCode`: the one-off Python
  script that transcribed Appendix B into a CSV and generated the enum,
  match arms, and round-trip tests is scratch tooling, not part of the repo.
  `src/currency_code.rs` is the source of truth going forward.
