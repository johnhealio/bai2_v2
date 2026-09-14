# Type Codes

## Primary Goal
Build an Enum to hold the type code data defined in the bai2_doc.pdf file


All the type codes included the definition should be included in the code and not loaded from a file or other sources.
An additional variant, Unknown" should be included when the code provide is not defined.
Implement a "from" function using a &str as input
Include sub enum for "Transaction", NA, DB, and CR
Include sub enum for "Level", Status, Summary, Detail

Update this file with technical specs used to build the enum

## Technical Spec

Implemented in `src/type_code.rs`.

### Source

Every type code is transcribed from Appendix A ("Uniform BAI Balance Reporting
Type Codes and Type Code Ranges") of the BAI Cash Management Balance Reporting
Specifications, Version 2 (`bai2_doc.pdf`, printed pages 46-68). That appendix
lists 469 defined type codes, each with a 3-digit code, a `Transaction`
(`NA`/`DB`/`CR`), a `Level` (`Status`/`Summary`/`Detail`), and a description.
All 469 are hard-coded as enum variants — nothing is loaded from a file at
runtime.

### Types

- `Transaction` — `Na`, `Db`, `Cr`.
- `Level` — `Status`, `Summary`, `Detail`.
- `TypeCode` — one variant per defined 3-digit code, plus `Unknown` for any
  code string not present in Appendix A.

### `TypeCode` API

- `TypeCode::from(code: &str) -> TypeCode` (via `impl From<&str> for TypeCode`)
  — maps a 3-digit code string to its variant, or `TypeCode::Unknown` if the
  code isn't defined. Implemented as a single array index (`O(1)`, no string
  comparisons): the input is validated as exactly 3 ASCII digits, parsed to a
  `0..=999` index, and looked up in a static `[TypeCode; 1000]` table, rather
  than matched against 469 string literals.
- `code(self) -> &'static str` — the original 3-digit code string (`""` for
  `Unknown`).
- `description(self) -> &'static str` — the description from Appendix A.
- `transaction(self) -> Option<Transaction>` — `None` for `Unknown`.
- `level(self) -> Option<Level>` — `None` for `Unknown`.

### Variant naming

Enum variant identifiers are derived from each code's description, converted
to PascalCase (e.g. code `010` "Opening Ledger" → `OpeningLedger`). A small
number of codes across the Debit/Credit ranges (001-099 vs. 400-699, etc.)
share an identical description for their credit and debit counterparts (e.g.
"Overdraft" appears as both a `CR` detail code and a `DB` detail code); these
are disambiguated by appending the transaction (`Credit`/`Debit`/`Na`), then
the level, and finally the numeric code itself, in that order, only as far as
needed to make each identifier unique. Descriptions beginning with a digit
(e.g. "0-Day Float") are prefixed with `N` since Rust identifiers cannot start
with a digit.

### Codegen

The 469 variants, and their four `match` arms plus exhaustive round-trip
tests, were generated from a CSV transcription of Appendix A by a one-off
Python script rather than typed by hand, to guarantee every code in the
appendix is present and that `code()` / `from()` stay in lockstep. The script
itself is not part of the repository (scratch tooling only) — `src/type_code.rs`
is the source of truth going forward and should be hand-edited for any future
corrections.
