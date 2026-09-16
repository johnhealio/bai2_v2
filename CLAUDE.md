## Primary Goal
Build a Rust library (crate `bai2`) to parse a BAI2-formatted file.

File spec: https://cdn.bai.org/migrated-from-www/docs/default-source/libraries/site-general-downloads/cash_management_2005.pdf

Github repo: git@github.com:johnhealio/bai2_v2.git

## How this project is organized

Work proceeds one phase at a time, in the order listed below. Each phase has
a spec file in `docs/` named after the module it covers
(`docs/TYPE_CODE.md`, `docs/FUND_TYPE.md`, etc.) — read that file before
touching the corresponding phase. A phase's doc is written to be
self-contained enough that its module could be regenerated from the doc
plus the spec PDF alone, without needing to re-derive earlier design
decisions from scratch or from git history.

Only work on the current phase — the first one below not yet marked done.
Don't start a later phase early, and don't revisit a completed phase's
design unless the user explicitly asks; treat its `docs/*.md` as settled.

## Phases

| Phase | Module | Status |
|---|---|---|
| 1 | Initialize Rust Project and TYPE_CODE | Done — `src/type_code.rs`, `docs/TYPE_CODE.md` |
| 2 | FUND_TYPE | Done — `src/funds_type.rs`, `docs/FUND_TYPE.md` |
| 3 | CURRENCY_CODE | Done — `src/currency_code.rs`, `docs/CURRENCY_CODE.md` |
| 4 | TRANSACTION_DETAIL (record type 16) | Not started — spec in `docs/TRANSACTION_DETAIL.md` |

## Conventions established so far

These apply to every phase, not just the ones already done — follow them
without being asked again unless a specific doc says otherwise:

- **Hardcode spec data in source, not at runtime.** Enums like `TypeCode`
  and `CurrencyCode` have every defined value written out as a literal
  variant. Nothing is loaded from a CSV/JSON/etc. file at runtime, and the
  project has zero runtime dependencies — don't add a crate to solve a
  problem a hand-rolled type can solve just as well at this scale (see
  `docs/FUND_TYPE.md`'s "no date/time crate" note for the reasoning and its
  limits).
- **Large lookup tables must be genuinely O(1), and verified with
  `cargo clippy`, not just `cargo test`.** A big `match` on `&str` is a
  comparison chain, not a lookup — see the "Lookup Algorithm" section in
  `docs/TYPE_CODE.md` and `docs/CURRENCY_CODE.md` for the two variants of
  this pattern used so far (dense literal table vs. compact entries list +
  const-evaluated build loop, depending on how sparse the code space is).
  Run `cargo clippy --all-targets` on any new generated table before
  considering it done — it's what caught that `CurrencyCode`'s table
  needed `static` instead of `const`.
- **An `Unknown` fallback vs. a `Result`/error type is a real design
  choice, not a coin flip.** Use `Unknown` (infallible parsing) when the
  code space is large/sparse/evolving and "not defined (yet)" is itself a
  meaningful value (`TypeCode`, `CurrencyCode`). Use an error type when the
  code space is small and closed, so anything outside it is actually
  malformed input (`FundsType`). Say which one applies and why in the
  phase's doc.
- **Every hardcoded/generated table gets exhaustive round-trip tests**
  (`from(code) == Variant` and `Variant.code() == code` for every defined
  value), plus explicit tests for the malformed/undefined cases. One-off
  Python (or similar) codegen scripts used to transcribe a spec appendix
  into Rust are scratch tooling — never commit them; the generated
  `src/*.rs` file is the source of truth from the moment it's written, and
  gets hand-edited for any future correction.
- **When the spec document itself is internally inconsistent or stale in a
  way that has real correctness consequences** (e.g. Appendix B's decimals
  table marking EUR/BRL as 0-decimal via what are clearly uncorrected
  pre-Euro entries), stop and ask before either transcribing the error
  silently or silently "fixing" it — this is exactly the kind of decision
  that's the user's to make, and it gets a brief, focused question, not an
  essay. Purely mechanical choices (variant naming/disambiguation rules,
  what to do with an orphaned code that appears in one appendix table but
  not another) don't need to be asked about — decide and document the
  reasoning in the phase's doc.
- **A phase's doc gets updated to reflect the actual implementation**,
  including anything decided only in conversation (a tradeoff worked out
  interactively, an edge case discovered while testing) — the doc is the
  durable record, not the chat.
