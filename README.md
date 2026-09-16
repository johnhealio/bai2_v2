# bai2

A Rust library for parsing [BAI2](https://cdn.bai.org/migrated-from-www/docs/default-source/libraries/site-general-downloads/cash_management_2005.pdf)-formatted
cash management balance reporting files.

BAI2 is a bank-to-corporate file format for transmitting account balance
and transaction detail information. This crate hardcodes the format's
defined code tables as Rust enums and provides typed parsers for its
composite fields and records, built incrementally, phase by phase, straight
from the BAI Cash Management Balance Reporting Specifications, Version 2.

## Status

| Phase | Module | Status |
|---|---|---|
| 1 | `TypeCode` — every Uniform BAI Balance Reporting Type Code (Appendix A) | Done |
| 2 | `FundsType` — the composite Funds Type field (03/16 records) | Done |
| 3 | `CurrencyCode` — every ISO 4217 currency code (Appendix B) | Done |
| 4 | `TransactionDetail` — record type 16 | Done |
| 5 | `AccountIdentifier` — record type 03, incl. 88 continuations | Done |
| 6 | `AccountTrailer` — record type 49 | Done |
| 7 | `Account` — aggregates 03 + 16* + 49 | Done |
| 8 | `GroupHeader` — record type 02 | Done |
| 9 | `GroupTrailer` — record type 98 | Done |
| 10 | `Group` — aggregates 02 + (03 + 16* + 49)* + 98 | Done |
| 11 | `FileHeader` — record type 01 | Done |
| 12 | `FileTrailer` — record type 99 | Done |
| 13 | `File` — aggregates 01 + (02 + (03 + 16* + 49)* + 98)* + 99 | Done |
| 14 | `BaiReader` — folds 88 continuation records into logical lines | Done |

`BaiReader` plus `File` together give an end-to-end path from a raw BAI2
transmission (anything implementing `std::io::Read`, 88 continuations and
all) to a fully validated `File`: wrap the source in a `BaiReader`, `push`
each yielded line into a `File`, then call `validate()`. See
`docs/READER.md` for exactly how continuation folding works, and its one
real scope boundary (assumes one physical record per text line — a raw
fixed-width, non-newline-delimited transmission isn't handled).
See `CLAUDE.md` for the current phase plan and
`docs/` for each module's
technical spec.

## Usage

```rust
use bai2::{BaiReader, CurrencyCode, File, FundsType, TypeCode};

// A 3-digit BAI2 type code.
let code = TypeCode::from("010");
assert_eq!(code, TypeCode::OpeningLedger);
assert_eq!(code.code(), "010");
assert_eq!(code.description(), "Opening Ledger");

// A 3-letter ISO 4217 currency code, and its implied decimal places.
let currency = CurrencyCode::from("JPY");
assert_eq!(currency.decimals(), 0); // Yen has no minor unit

// A Funds Type composite field, already comma-split.
let (funds_type, consumed) = FundsType::parse(&["V", "040701", "1300"]).unwrap();
assert_eq!(consumed, 3);

// A full file, read from anything implementing `std::io::Read`, with
// any 88 continuation records folded in automatically.
let raw = "01,S,R,040620,0200,1,,,2/\n\
           02,,ORIG,1,040620/\n\
           03,111,,190,100,4,0/\n\
           49,100,2/\n\
           98,100,1,4/\n\
           99,100,1,6/\n";
let mut file = File::new();
for line in BaiReader::new(raw.as_bytes()) {
    file.push(&line.unwrap()).unwrap();
}
assert_eq!(file.groups.len(), 1);
assert_eq!(file.validate(), vec![]); // every control total reconciles
```

Every code table (`TypeCode`, `CurrencyCode`) is hardcoded from its
respective spec appendix — nothing is loaded from a file at runtime — and
exposes an infallible `From<&str>` with an `Unknown` fallback for anything
not defined. `FundsType` parses a small closed set of composite-field codes
and returns a `Result` for anything malformed. See each module's doc in
`docs/` for the reasoning behind that split, and for every other
non-obvious parsing decision (known BAI2 spec inconsistencies and how each
was resolved, why certain lookups are structured the way they are, etc.).

## Building and testing

```sh
cargo build
cargo test
cargo clippy --all-targets
```

The crate has zero runtime dependencies. `cargo test` runs both the
per-module unit tests and `tests/bank_style_files.rs`, a set of
integration tests against large, multi-group/multi-account files built
to exercise real-world variations (heavy 88 continuation usage,
multi-currency files, currency overrides, zero-account groups, bare
account-only-for-detail records) — constructed from the spec and
commonly documented implementer conventions, not literal production
output from any institution.

## Project layout

- `src/` — one module per phase (`type_code.rs`, `funds_type.rs`,
  `currency_code.rs`, ...).
- `docs/` — one spec file per module, written to be self-contained enough
  to regenerate that module from the doc plus the BAI2 spec PDF alone.
- `CLAUDE.md` — phase plan and conventions for ongoing development.
