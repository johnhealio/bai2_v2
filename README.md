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
| 4 | `TransactionDetail` — record type 16 | Not started |

This is a work in progress: no top-level file parser exists yet. See
`CLAUDE.md` for the current phase plan and `docs/` for each module's
technical spec.

## Usage

```rust
use bai2::{CurrencyCode, FundsType, TypeCode};

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

The crate has zero runtime dependencies.

## Project layout

- `src/` — one module per phase (`type_code.rs`, `funds_type.rs`,
  `currency_code.rs`, ...).
- `docs/` — one spec file per module, written to be self-contained enough
  to regenerate that module from the doc plus the BAI2 spec PDF alone.
- `CLAUDE.md` — phase plan and conventions for ongoing development.
