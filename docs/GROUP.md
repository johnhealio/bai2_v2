# Group — Aggregate of Record Types 02, (03 + 16* + 49)*, and 98

## Goal

A struct, `Group`, that assembles one BAI2 group — one `GroupHeader` (02),
zero or more `Account`s (each itself an 03 + 16* + 49 sequence), and one
`GroupTrailer` (98) — via an incremental `push` method, the same shape as
`Account` (`docs/ACCOUNT.md`) one level up: `Group` : `GroupHeader` +
`Account` + `GroupTrailer` :: `Account` : `AccountIdentifier` +
`TransactionDetail` + `AccountTrailer`.

## Source

The "FILE LAYOUT" section (`bai2_doc.pdf`, printed pages 8-9), which
defines the required group structure (`02`, then zero or more accounts
each shaped `03, 16*, 49`, then `98`, with "There must be one 98 record
for each 02 record") — plus the 98 record's definition of Group Control
Total as the algebraic sum of every account's Account Control Total
(`docs/GROUP_TRAILER.md`), which this module's validation checks.

## Scope note: 88 (Continuation) records are still not handled here

Same as `Account` (`docs/ACCOUNT.md`): **`push` assumes any 88
continuation lines are already merged into whichever 02/03/16/49/98
record they continue.** A bare `88` record code is rejected with
`UnmergedContinuationRecord` rather than mishandled. This limitation
propagates unchanged from `Account` — `Group` doesn't add or remove
anything about it, just inherits it.

## Data Model

```rust
pub struct Group {
    pub header: Option<GroupHeader>,
    pub accounts: Vec<Account>,
    pub trailer: Option<GroupTrailer>,
}

pub enum GroupError {
    /// A second 02 record was pushed; only one is allowed per group.
    DuplicateGroupHeader,
    /// A 98 record was pushed before this group's 02 record.
    GroupTrailerBeforeGroupHeader,
    /// A 03 record was pushed before this group's 02 record.
    AccountBeforeGroupHeader,
    /// A 03 record (opening a new account) or a 98 record (closing the
    /// group) was pushed while the previous account hasn't been closed
    /// with its own 49 record yet.
    PreviousAccountNotClosed,
    /// A 16 or 49 record was pushed with no account currently open —
    /// either none has been started yet, or the last one already
    /// closed. Push a 03 record first.
    NoOpenAccount,
    /// A record was pushed after this group's 98 record already closed it.
    PushAfterGroupTrailer,
    /// The record code wasn't "02", "03", "16", "49", or "98".
    UnrecognizedRecordCode(String),
    /// The record code was "88" — see "Scope note" above.
    UnmergedContinuationRecord,
    GroupHeader(GroupHeaderError),
    GroupTrailer(GroupTrailerError),
    /// Wraps whatever `Account::push` returned when a 03/16/49 line was
    /// delegated to the currently-open account.
    Account(AccountError),
}

pub enum GroupViolation {
    /// A violation `Account::validate` found in one of this group's
    /// accounts, identified by its position in `accounts` (not by
    /// account number — an account that failed to parse its own
    /// identifier wouldn't have one to key on).
    Account { account_index: usize, violation: AccountViolation },
    /// The 98 record's Group Control Total doesn't equal the algebraic
    /// sum of every account's Account Control Total.
    ControlTotalMismatch { expected: i64, computed: i64 },
    /// The 98 record's Number of Accounts doesn't equal the actual
    /// number of accounts pushed.
    AccountCountMismatch { expected: u32, actual: usize },
}
```

## API

- `Group::new() -> Group` — always succeeds; returns an empty group
  (`header: None, accounts: vec![], trailer: None`) ready for `push`.
  **Takes no parameters**, unlike `Account::new(group_currency:
  CurrencyCode)` — contrast deliberate: `Account` needed an externally
  supplied currency because nothing below it in the hierarchy produces
  one, but `Group` sits at the level where currency actually originates.
  Once `push` parses a `02` record, `GroupHeader.currency` (which itself
  defaults to a fixed `USD` — `docs/GROUP_HEADER.md`) is exactly the value
  every `Account` created afterward needs, and `Group` already has it.
- `Group::push(&mut self, text: &str) -> Result<(), GroupError>` —
  inspects `text`'s record code and either handles it directly (`02`,
  `98`), or creates/opens the next `Account` (`03`), or delegates to the
  currently-open account (`16`, `49`) — see "Order enforcement and
  delegation" below.
- `Group::validate(&self) -> Vec<GroupViolation>` — see "Group-level
  validation" below.

## Decisions & Edge Cases

- **Order enforcement and delegation: `Group` owns group-level ordering;
  `Account` still owns its own.** `push` handles `02`/`98` itself and
  decides *when* a `03`/`16`/`49` line is even eligible to be attempted
  (is there a header yet? is there an open account, or does one need
  starting?) — but the actual per-record parsing and account-internal
  ordering (e.g. "16 before its own 03") is still `Account::push`'s job,
  wrapped into `GroupError::Account`. `Group` doesn't re-implement
  `Account`'s state machine; it just decides which `Account` (if any) a
  given line should be handed to.
- **`accounts: Vec<Account>` holds the currently-open account too, not
  just closed ones.** A `03` line immediately creates a new `Account` and
  appends it; subsequent `16`/`49` lines mutate `accounts.last_mut()`.
  Whether the *last* account is "open" (no trailer yet) or "closed"
  (has one) is derived from `accounts.last().trailer.is_some()` on demand
  — there's no separate boolean or buffer field tracking this.
- **A `03` (or a closing `98`) while the previous account is still open is
  one shared error, `PreviousAccountNotClosed`**, not two separate
  variants. Both cases have the identical fix (close the open account with
  a `49` first), so — unlike `Account`, which kept `16`-before-`03` and
  `49`-before-`03` as separate errors because those are genuinely
  different fields — collapsing these two at the `Group` level is a
  deliberate reduction in granularity, made because the extra variants
  wouldn't tell a caller anything the shared one doesn't already say.
- **`16`/`49` with nowhere to go is one shared error, `NoOpenAccount`**,
  covering both "no account has been started yet" and "the last account
  is already closed" — same reasoning: the fix is identical (push a `03`
  first), so a caller doesn't need to distinguish which.
- **`98` requires the last account to already be closed, same as a new
  `03` would.** Per spec, a group can't legally end while its last
  account is still open — enforced with the same `PreviousAccountNotClosed`
  check `03` uses, not a new variant.
- **A new `Account` is constructed with `GroupHeader.currency`, not
  anything the caller supplies to `push`.** This is the one piece of state
  `Group::push` computes and threads through on `Account`'s behalf,
  mirroring how `Account::push` itself threads its *own* resolved
  currency down to each `TransactionDetail` (`docs/ACCOUNT.md`).

## Group-level validation

`validate()` checks three things, meaningful to call at any point (not
just once the group is closed):

1. **Every violation `Account::validate` finds, for every account in
   `accounts`**, wrapped as `GroupViolation::Account { account_index, .. }`
   with `account_index` being that account's position in `accounts`.
2. **Group Control Total reconciliation**, only if `trailer` is `Some`
   (otherwise skipped, not reported): sum every account's
   `AccountTrailer.account_control_total` (each account should already be
   closed by the time `trailer` is set — see "Order enforcement" above —
   but this sums defensively via `Option`, treating a hypothetically-still-open
   account as contributing `0`, same pattern `Account::validate` uses for
   a missing `Amount`). Mismatch → `ControlTotalMismatch`.
3. **Account count reconciliation**, only if `trailer` is `Some`:
   `GroupTrailer.number_of_accounts` must equal `accounts.len()` exactly.
   Unlike `AccountTrailer`'s Number of Records (skipped in
   `docs/ACCOUNT_TRAILER.md`/`docs/ACCOUNT.md` because physical line counts
   including 88 continuations aren't tracked), this count has **no such
   ambiguity** — `accounts.len()` is an exact count of logical `Account`s,
   not physical lines, so there's nothing to hedge on. Mismatch →
   `AccountCountMismatch`.

**Still not validated: `GroupTrailer.number_of_records`** (total physical
records in the group) — same reasoning as `AccountTrailer`'s equivalent
field: physical record/line counts, including any 88 continuations,
aren't tracked at this level.

## Testing Expectations

- Pushing `02`, then a full `03`/`16`/`49` account, then another full
  account, then `98`, succeeds at every step and produces a
  fully-populated `Group` with `accounts.len() == 2`.
- Pushing `03`, `16`, `49`, or `98` before any `02` produces
  `AccountBeforeGroupHeader`/`NoOpenAccount`/`NoOpenAccount`/
  `GroupTrailerBeforeGroupHeader` respectively.
- Pushing a second `02` produces `DuplicateGroupHeader`.
- Pushing a `03` while the previous account is still open (no `49` pushed
  yet) produces `PreviousAccountNotClosed`; pushing `98` in the same
  situation produces the same error.
- Pushing `16` or `49` after the currently-open account has already been
  closed (and before a new `03` starts another) produces `NoOpenAccount`.
- Pushing anything after `98` produces `PushAfterGroupTrailer`.
- Pushing `88` alone produces `UnmergedContinuationRecord`; an
  unrecognized code produces `UnrecognizedRecordCode`.
- A malformed `02`/`98` line, or a malformed `03`/`16`/`49` line delegated
  to the open account, produces the corresponding wrapped `GroupError`
  variant with the underlying error preserved exactly.
- Each account created via `push` receives the group header's resolved
  `currency`, confirmed the same way `docs/ACCOUNT.md` confirms it for
  `TransactionDetail`.
- `validate()` on a fully conformant, closed group returns `[]`; on one
  where an account has its own business-rule violation, returns it wrapped
  with the right `account_index`; on one whose trailer's total or account
  count doesn't match, returns `ControlTotalMismatch`/
  `AccountCountMismatch` with both expected and actual/computed values; on
  one still missing its trailer, returns neither of those two checks
  (only wrapped account violations, if any), even if they'd mismatch once
  a trailer is pushed.

## Codegen Note

No large data table here — hand-written, not generated from a spec
appendix, same as `Account`, `AccountIdentifier`, `TransactionDetail`,
`AccountTrailer`, `GroupHeader`, and `GroupTrailer`.
