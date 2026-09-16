# Currency Codes

## Goal

An enum, `CurrencyCode`, holding every ISO 4217 currency code listed in
Appendix B of the spec, hardcoded in source, with an infallible `&str`
parser, a decimal-places accessor, and a description accessor. Used as a
field value at the group/account level and as input to transaction detail
amount parsing (to know how many implied decimal places an `Amount` field
has).

## Source

Transcribed from Appendix B ("Currency Codes") of the BAI Cash Management
Balance Reporting Specifications, Version 2 (`bai2_doc.pdf`, printed pages
69-83): a Country/Currency/Code table based on ISO 4217, plus a separate
"Implied Decimals" section listing the currency codes whose implied decimal
count isn't the 2-decimal default.

To regenerate this module from scratch:

1. Transcribe the Country/Currency/Code table. Skip the handful of rows
   whose Code is `--` ("No Universal Currency", e.g. Antarctica) — not real
   currency codes. Multiple countries share a currency (e.g. Andorra,
   Austria, Italy are all `EUR`); collapse each unique **code** to one
   variant, keeping any one of its currency names as the description. As of
   this writing that's 181 unique codes.
2. Transcribe the "Implied Decimals" exceptions section, which groups codes
   under "ZERO (0) Decimals", "One (1) Decimal", and "Three (3) Decimals"
   headings. Every code not listed there defaults to 2 decimals.
3. Apply the two corrections in "Decisions & Edge Cases" below before
   finalizing the decimals table — transcribing the exceptions section
   completely literally reproduces a real error in the source document.

## Data Model

```rust
pub enum CurrencyCode {
    // one variant per unique Appendix B code, named after the code itself
    Usd,
    Eur,
    // ...
    Unknown, // any code not in Appendix B, or malformed input
}
```

## API

- `impl From<&str> for CurrencyCode` — `CurrencyCode::from(code: &str) ->
  CurrencyCode`. Maps a 3-letter code string to its variant, or
  `CurrencyCode::Unknown` for anything not in Appendix B, including any
  string that isn't exactly 3 uppercase ASCII letters (wrong length,
  lowercase, digits, `"--"`, blank). Infallible, matching `TypeCode`'s
  precedent (see "Unknown vs. error type" below).
- `code(self) -> &'static str` — the 3-letter code. `""` for `Unknown`.
- `description(self) -> &'static str` — the currency name from Appendix B.
- `decimals(self) -> u32` — implied decimal places for Amount/Funds Type
  fields in this currency. `2` by default, including for `Unknown`.

## Lookup Algorithm

Same motivation as `TypeCode::from` (see `docs/TYPE_CODE.md`): a 181-arm
string-equality match is not O(1). The code space here is 3 uppercase ASCII
letters, so:

1. Validate the input is exactly 3 bytes, each in `b'A'..=b'Z'`; anything
   else is `Unknown` immediately.
2. Map the 3 letters to a base-26 index in `0..17576` (`(a-'A')*676 +
   (b-'A')*26 + (c-'A')`).
3. Index into a `static [CurrencyCode; 17576]` table.

Unlike `TypeCode`'s 1000-entry table, this one is **not** written out as
17,576 literal lines — the code space (26³) is over an order of magnitude
bigger than `TypeCode`'s (10³) while only ~1% populated, so a dense literal
listing would be enormous for no runtime benefit. Instead:

- A compact `const` list of the 181 `(code, variant)` pairs is written
  literally (this is the only place all 181 codes appear as data).
- The full table is built from that list by a `const`-evaluated loop
  (`while` loop over the list, writing into an otherwise-`Unknown`-filled
  array) — this still runs entirely at compile time, so the *runtime*
  behavior is identical to a literal table: one array index, no loop, no
  string comparison.
- The table is declared `static`, not `const`. A `const` array is inlined
  at every use site; at 17,576 entries that means real code-size bloat.
  `cargo clippy`'s `large_const_arrays` lint (part of `cargo clippy
  --all-targets`, not caught by `cargo test` or plain compilation) flags
  this — run clippy on any newly generated large table to catch this class
  of "looks efficient, isn't quite" mistake.

## Decisions & Edge Cases

- **Variant naming follows the code, not the currency name.** Unlike
  `TypeCode` (arbitrary numeric codes, named from their description), each
  `CurrencyCode` variant is named directly from its ISO code (`EUR` →
  `Eur`, `USD` → `Usd`, first letter uppercase, rest lowercase). Codes are
  already unique identifiers by construction (one variant per unique code),
  so this sidesteps `TypeCode`'s cross-code disambiguation problem entirely
  — there is nothing to disambiguate.
- **`Unknown` fallback, no `Result`/error type.** ISO 4217 is a large,
  *evolving* code space — new codes get added over time — unlike
  `FundsType`'s fixed 7-character enumeration (`docs/FUND_TYPE.md`), which
  treats anything outside its small closed set as malformed input. Treating
  an unrecognized currency code as "not (yet) defined" (`Unknown`) fits
  better here than treating it as an error, and matches how `TypeCode`
  itself already handles its analogous large/sparse code space.
- **EUR and BRL are hardcoded to 2 decimals, overriding Appendix B's own
  exceptions list.** That list marks `EUR`/`BRL` as 0-decimal, but only via
  what are clearly uncorrected pre-Euro entries — Andorra, Belgium, Italy,
  Luxemburg, and Spain are listed reporting in "Euro"/`EUR`, and Brazil in
  "Brazilian Real"/`BRL`, while other Euro-using countries in the *main*
  table (Austria, France, Germany, ...) aren't flagged as exceptions at all.
  Real-world EUR and BRL are both 2-decimal currencies. This was a
  deliberate, discussed correction (not a silent "fix"), because
  `decimals()` feeding a wrong divisor into amount parsing has real
  financial consequences — a euro amount parsed with 0 implied decimals is
  off by a factor of 100. The rest of the exceptions section is transcribed
  as printed: 0 decimals for `XOF`/`XAF`/`JPY`/`KMF`/`XPF`; 1 for `MRO`; 3
  for `BHD`/`EGP`/`IQD`/`JOD`/`KWD`/`LYD`/`MTL`/`OMR`/`TND`/`YDD`.
- **`SDP` (listed as 3-decimal in the exceptions section) has no
  corresponding variant.** The exceptions section lists Sudan's code as
  `SDP`; the *main* Appendix B table lists Sudan's code as `SDD` — a
  discrepancy in the source document itself (Sudan has in fact used several
  historical currency codes over time: `SDP`, then `SDD`, then today's
  `SDG`). Rather than guess which code the 3-decimal note was meant to
  apply to, `SDP` is simply absent from the generated variants, and `Sdd`
  keeps the 2-decimal default. If this ever matters in practice, resolve it
  by checking which code real-world transmissions actually use, not by
  guessing from the spec text alone.

## Testing Expectations

- Every one of the 181 defined codes round-trips: `CurrencyCode::from(code)
  == Variant` and `Variant.code() == code`.
- A representative set of malformed/undefined inputs (`""`, `"--"`,
  lowercase, wrong length, an unassigned 3-letter combination) all parse to
  `Unknown`, and `Unknown.code() == ""`, `Unknown.decimals() == 2`.
- Each decimal-exception bucket has at least one covering test: 0 decimals
  (e.g. `Jpy`, `Xof`), 1 decimal (`Mro`), 3 decimals (e.g. `Bhd`, `Kwd`,
  `Tnd`), and the two corrected codes explicitly assert 2 decimals (`Eur`,
  `Brl`) with a comment noting they're overrides, not the spec default.

## Codegen Note

The 181 variants, their three match arms (`code`, `description`,
`decimals`), the compact entries list, and the round-trip tests were
generated from a CSV transcription of Appendix B by a one-off Python
script, not typed by hand. The script isn't part of the repository (scratch
tooling only); `src/currency_code.rs` is the source of truth going forward.
