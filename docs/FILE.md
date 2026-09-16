# File — Aggregate of Record Types 01, (02 + (03 + 16* + 49)* + 98)*, and 99

## Goal

A struct, `File`, that assembles one complete BAI2 file — one `FileHeader`
(01), zero or more `Group`s (each itself a 02 + account* + 98 sequence),
and one `FileTrailer` (99) — via an incremental `push` method. Same shape
as `Group` (`docs/GROUP.md`) and `Account` (`docs/ACCOUNT.md`) one level
up: `File` : `FileHeader` + `Group` + `FileTrailer` :: `Group` :
`GroupHeader` + `Account` + `GroupTrailer` :: `Account` :
`AccountIdentifier` + `TransactionDetail` + `AccountTrailer`.

## Source

The "FILE LAYOUT" section (`bai2_doc.pdf`, printed pages 8-9), which
defines the required file structure (`01`, then zero or more groups each
shaped `02, (account)*, 98`, then `99`) — plus the 99 record's definition
of File Control Total as the algebraic sum of every group's Group Control
Total (`docs/FILE_TRAILER.md`), which this module's validation checks.

## Scope note: 88 (Continuation) records are still not handled here

Same inherited limitation as `Account` (`docs/ACCOUNT.md`) and `Group`
(`docs/GROUP.md`): **`push` assumes any 88 continuation lines are already
merged into whichever record they continue.** A bare `88` record code is
rejected with `UnmergedContinuationRecord`. Nothing new here — `File`
just carries the same assumption forward unchanged.

## Data Model

```rust
pub struct File {
    pub header: Option<FileHeader>,
    pub groups: Vec<Group>,
    pub trailer: Option<FileTrailer>,
}

pub enum FileError {
    /// A second 01 record was pushed; only one is allowed per file.
    DuplicateFileHeader,
    /// A 99 record was pushed before this file's 01 record.
    FileTrailerBeforeFileHeader,
    /// A 02 record was pushed before this file's 01 record.
    GroupBeforeFileHeader,
    /// A 02 record (opening a new group) or a 99 record (closing the
    /// file) was pushed while the previous group hasn't been closed
    /// with its own 98 record yet.
    PreviousGroupNotClosed,
    /// A 03, 16, 49, or 98 record was pushed with no group currently
    /// open — either none has been started yet, or the last one already
    /// closed. Push a 02 record first.
    NoOpenGroup,
    /// A record was pushed after this file's 99 record already closed it.
    PushAfterFileTrailer,
    /// The record code wasn't "01", "02", "03", "16", "49", "98", or "99".
    UnrecognizedRecordCode(String),
    /// The record code was "88" — see "Scope note" above.
    UnmergedContinuationRecord,
    FileHeader(FileHeaderError),
    FileTrailer(FileTrailerError),
    /// Wraps whatever `Group::push` returned when a 02/03/16/49/98 line
    /// was delegated to the currently-open group.
    Group(GroupError),
}

pub enum FileViolation {
    /// A violation `Group::validate` found in one of this file's groups,
    /// identified by its position in `groups`. Since `GroupViolation`
    /// itself wraps `AccountViolation` (`docs/GROUP.md`), this nests all
    /// the way down: a bad Item Count on some account's 03 record surfaces
    /// here as `Group { group_index, violation: GroupViolation::Account {
    /// account_index, violation: AccountViolation::AccountIdentifier(..) } }`.
    Group { group_index: usize, violation: GroupViolation },
    /// The 99 record's File Control Total doesn't equal the algebraic sum
    /// of every group's Group Control Total.
    ControlTotalMismatch { expected: i64, computed: i64 },
    /// The 99 record's Number of Groups doesn't equal the actual number
    /// of groups pushed.
    GroupCountMismatch { expected: u32, actual: usize },
}
```

## API

- `File::new() -> File` — always succeeds; returns an empty file
  (`header: None, groups: vec![], trailer: None`) ready for `push`. Takes
  no parameters, same reasoning as `Group::new()` — nothing above `File`
  in the hierarchy for it to need supplied.
- `File::push(&mut self, text: &str) -> Result<(), FileError>` — see
  "Order enforcement and delegation" below.
- `File::validate(&self) -> Vec<FileViolation>` — see "File-level
  validation" below.

## Decisions & Edge Cases

- **Order enforcement and delegation follows the exact same recursive
  shape `Group` already established, one level up.** `File` only ever
  special-cases three record codes itself: `01` (sets the header), `02`
  (opens the next `Group`), and `99` (closes the file). Every other
  in-range code — `03`, `16`, `49`, *and* `98` — is delegated uniformly to
  `groups.last_mut()`, exactly the way `Group::push` delegates `16` and
  `49` uniformly to `accounts.last_mut()` (`docs/GROUP.md`). `File`
  doesn't need to know or care that `98` closes a group and `16`/`49`
  don't — `Group::push` already handles that distinction internally, the
  same way `Account::push` already handles `16` vs. `49` without `Group`
  re-implementing it.
- **`groups: Vec<Group>` holds the currently-open group too**, same
  pattern as `Group.accounts` — a `02` line immediately creates a new
  `Group` and appends it; whether the *last* group is "open" or "closed"
  is derived on demand from `groups.last().trailer.is_some()`, not tracked
  in a separate field.
- **A `02` (or a closing `99`) while the previous group is still open is
  one shared error, `PreviousGroupNotClosed`** — same reduction in
  granularity `Group` already made for `PreviousAccountNotClosed`
  (`docs/GROUP.md`): both cases have the identical fix (close the open
  group with a `98` first), so a second named variant wouldn't tell a
  caller anything new.
- **`03`/`16`/`49`/`98` with nowhere to go is one shared error,
  `NoOpenGroup`** — same reasoning as `Group`'s `NoOpenAccount`: "no group
  has been started yet" and "the last group is already closed" both have
  the identical fix (push a `02` first).
- **A new `Group` is constructed with `Group::new()` — no parameters —
  and then immediately has the same `02` line pushed into it** to set its
  header, rather than `File` parsing the `02` line itself and handing
  `Group` an already-built `GroupHeader`. This keeps `File` from
  duplicating any of `GroupHeader`'s parsing/validation logic; `File`'s
  only job is deciding *that* a `02` line should start a new `Group`, not
  *how* to parse one.

## File-level validation

`validate()` checks three things, meaningful to call at any point (not
just once the file is closed):

1. **Every violation `Group::validate` finds, for every group in
   `groups`**, wrapped as `FileViolation::Group { group_index, .. }` —
   which itself may already be a wrapped `GroupViolation::Account { .. }`,
   so a single deeply-nested violation traces all the way from `File` down
   to the specific account (and, for an `AccountIdentifier` business-rule
   violation, the specific `TypeCode`) that caused it.
2. **File Control Total reconciliation**, only if `trailer` is `Some`
   (otherwise skipped, not reported): sum every group's
   `GroupTrailer.group_control_total` (defensively via `Option`, treating
   a hypothetically-still-open group as contributing `0` — each group
   should already be closed by the time `trailer` is set, per "Order
   enforcement" above, same pattern `Group::validate` uses one level
   down). Mismatch → `ControlTotalMismatch`.
3. **Group count reconciliation**, only if `trailer` is `Some`:
   `FileTrailer.number_of_groups` must equal `groups.len()` exactly — no
   ambiguity, same reasoning as `Group`'s `AccountCountMismatch`
   (`docs/GROUP.md`): this counts logical `Group`s, not physical lines.

**Still not validated: `FileTrailer.number_of_records`** — same reasoning
cascading all the way down from `AccountTrailer`: physical record/line
counts, including any 88 continuations, aren't tracked at any level in
this crate yet.

## Testing Expectations

- Pushing `01`, then a full `02`/account(s)/`98` group, then another full
  group, then `99`, succeeds at every step and produces a fully-populated
  `File` with `groups.len() == 2`.
- Pushing `02`, `03`, `16`, `49`, `98`, or `99` before any `01` produces
  `GroupBeforeFileHeader`/`NoOpenGroup` (for `03`/`16`/`49`/`98`)/
  `FileTrailerBeforeFileHeader` respectively.
- Pushing a second `01` produces `DuplicateFileHeader`.
- Pushing a `02` while the previous group is still open (no `98` pushed
  yet) produces `PreviousGroupNotClosed`; pushing `99` in the same
  situation produces the same error.
- Pushing `03`/`16`/`49`/`98` after the currently-open group has already
  been closed (and before a new `02` starts another) produces
  `NoOpenGroup`.
- Pushing anything after `99` produces `PushAfterFileTrailer`.
- Pushing `88` alone produces `UnmergedContinuationRecord`; an
  unrecognized code produces `UnrecognizedRecordCode`.
- A malformed `01`/`99` line, or a malformed `02`/`03`/`16`/`49`/`98` line
  delegated to the open group (and, through it, to the open account),
  produces the corresponding wrapped `FileError` variant with the
  underlying error preserved exactly — including through two levels of
  wrapping (`FileError::Group(GroupError::Account(AccountError::...))`).
- `validate()` on a fully conformant, closed file returns `[]`; on one
  where some account deep inside some group has its own business-rule
  violation, returns it wrapped through both levels
  (`FileViolation::Group { group_index, violation: GroupViolation::Account
  { account_index, violation } }`); on one whose trailer's total or group
  count doesn't match, returns `ControlTotalMismatch`/`GroupCountMismatch`;
  on one still missing its trailer, returns neither of those two checks,
  even if they'd mismatch once a trailer is pushed.

## Codegen Note

No large data table here — hand-written, not generated from a spec
appendix, same as every other record/aggregate module.
