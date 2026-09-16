# Reader — Folding 88 (Continuation) Records into Logical Lines

## Goal

An iterator, wrapping any `std::io::Read` (a file, a socket, an in-memory
buffer, ...), that reads a raw BAI2 transmission line by line, folds any
88 (Continuation) records into the logical record they continue per the
spec's exact merge rule, and yields fully-formed logical record strings
— each one ready to hand directly to `File::push` (`docs/FILE.md`),
`Group::push` (`docs/GROUP.md`), or `Account::push` (`docs/ACCOUNT.md`)
without any further assembly.

This is the piece every aggregate module (`Account`, `Group`, `File`) has
deferred so far — each one's docs explicitly assume 88-merging is already
done by the caller. This phase builds that caller.

## Source

The "88 – CONTINUATION RECORD" section (`bai2_doc.pdf`, printed pages
20-21) — specifically its delimiter rules, which turn out to make the
merge fully mechanical rather than something requiring per-record-type
knowledge:

> If the preceding physical record does not end within a text field, that
> record should end with a delimiter slash "/", even if the following
> Continuation (88) includes or begins with text. If the preceding
> physical record ends within text that is to be continued, no delimiter
> should be placed at the end of that record.
>
> If a record is to be continued by a type 88 Continuation record, the
> last field before the continuation is delimited by a slash "/", not by
> a comma and a slash ",/". The slash replaces the comma that ordinarily
> delimits that field.

Combined with "A field may not be split across physical records unless
that field is a text field" (same section), this means: **a physical
line's trailing `/` — present or absent — is a fully reliable, mechanical
signal for how to fold the next line in, with no need to parse or
understand the record's fields at all.**

## The Merge Algorithm

Given a buffered logical record so far, and the next physical line:

1. If the next physical line doesn't start with `88,`, the buffer is a
   complete logical record — stop folding, emit it, and start a new
   buffer from this line.
2. If it does start with `88,`, strip that prefix, then:
   - If the buffer currently **ends with `/`**: this is an ordinary
     field boundary. Strip the trailing `/`, append a `,`, then append
     the rest of the continuation line.
   - If the buffer currently **doesn't end with `/`**: the previous line
     ended mid-text-field. Append the rest of the continuation line
     directly — no comma inserted.
3. Repeat from step 1 (an 88 record may itself be continued by another
   88, per spec: "A Continuation (88) record may follow any type of
   record, including another 'Continuation' (88)").

The emitted string may or may not end in `/` (depending on whether its
final physical line was itself mid-text or not) — this is deliberately
left as-is, not normalized, because every leaf `::new` function in this
crate already does `text.strip_suffix('/').unwrap_or(text)` as its own
first step (see e.g. `docs/ACCOUNT_TRAILER.md`). The reader doesn't need
to duplicate that.

## Data Model

```rust
pub struct BaiReader<R: std::io::Read> {
    // wraps io::Lines<io::BufReader<R>> or equivalent, with one line of
    // lookahead buffered internally so `next()` can tell whether the
    // record it's about to emit is actually complete (see "Lookahead"
    // below)
}

pub enum BaiReaderError {
    Io(std::io::Error),
    /// An `88` line appeared with nothing before it to continue — either
    /// the first line of the input, or one following a record that was
    /// already emitted.
    DanglingContinuationRecord,
}

impl<R: std::io::Read> Iterator for BaiReader<R> {
    type Item = Result<String, BaiReaderError>;
    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

## API

- `BaiReader::new(reader: R) -> BaiReader<R>` where `R: std::io::Read` —
  wraps any byte source. Takes no other parameters; this module has no
  dependency on anything else in the crate (it doesn't parse field
  content, just folds physical lines).
- `impl Iterator for BaiReader<R>`, `Item = Result<String, BaiReaderError>`
  — each `Some(Ok(line))` is one fully-merged logical record, ready for
  `File::push`/`Group::push`/`Account::push`. `None` means clean EOF with
  no pending dangling continuation.

## Decisions & Edge Cases

- **Physical records are assumed newline-delimited in the byte stream.**
  The spec itself is transport-agnostic about how physical records are
  separated ("The specifications do not replace, define, restrict,
  supersede, or alter data communication or telecommunication protocols
  used by senders and receivers of these files") and describes an
  alternative where Physical Record Length (from the 01 record) defines
  fixed-width chunks with no delimiter at all. This module assumes the
  overwhelmingly common real-world case — one physical record per text
  line — via `BufRead::lines()` (or equivalent), not fixed-width byte
  chunking. A true fixed-width-with-no-newlines transmission is out of
  scope; if that ever comes up in practice, it needs a different
  low-level line-splitting strategy feeding the same fold algorithm.
- **Trailing `\r` and trailing blank padding are trimmed from each raw
  physical line before folding.** Two concrete failure modes if this
  isn't done: a Windows-originated (`\r\n`) file leaves a stray `\r` on
  every line from `BufRead::lines()` (which only strips `\n`), silently
  corrupting whatever the last field on that line was (e.g. a Version
  Number field ending up as `"2\r"`, which fails to parse as `u32`); and
  a fixed-length-record file's blank padding after the logical record's
  last significant character would otherwise get folded in as part of
  the record's actual content. The spec itself describes such padding as
  "meaningless and should be disregarded" (`docs/FILE_HEADER.md`'s
  Physical Record Length note), so trimming it is exactly the documented
  behavior, not an assumption layered on top.
- **A dangling `88` (nothing to continue) is a defined error, not silently
  dropped or merged into the wrong place — but it can only actually arise
  at the very start of the input, or from consecutive leading `88` lines,
  never right after a fully-emitted record.** Any `88` immediately
  following a record is, by construction, already folded into that
  record before `next()` ever returns it — the fold loop only stops (and
  hands back a completed buffer) once it peeks a line that *isn't* an
  `88`, so whatever's left buffered for the next call is guaranteed not
  to be one either. Each dangling `88` produces its own `Err` and
  iteration continues past it (it doesn't end the stream) — a caller
  processing results needs to keep pulling items after an `Err`, not
  assume the first one is fatal.
- **Blank/empty physical lines are skipped, not treated as a malformed
  record.** A stray trailing blank line at EOF (very common in
  text files) has no record code to dispatch on and no reasonable
  interpretation as part of the previous or next record — skipping it
  silently is more useful than erroring on what's almost always
  incidental whitespace, not a data problem.
- **The reader has one line of lookahead, not zero.** `next()` can't
  know a buffered record is complete until it has peeked at the
  following physical line to check whether *that* one starts with `88,`.
  This means the iterator's internal state, after returning one item, may
  already contain the start of the next one (or nothing, at EOF) — a
  standard "peekable lines" pattern, but worth being explicit about since
  it means `next()`'s cost isn't uniform (the call that finally confirms
  a record is complete does an extra read).
- **This module produces lines; it does not drive `File::push` itself.**
  Deliberately scoped to exactly the iterator, per the phase's own
  framing — wiring `BaiReader` output into a loop that builds a `File` is
  the caller's few lines of code, not something this module wraps in a
  convenience function. If that convenience turns out to be wanted later,
  it's a thin addition on top of this, not a redesign.

## Testing Expectations

- A record with no continuations at all (e.g. a bare `01,...,2/` line)
  round-trips as a single emitted item, unchanged (modulo trailing
  whitespace/`\r` trimming).
- The spec's own 88 examples fold correctly:
  - `16,115,10000000,S,5000000,4000000,1000000/` followed by
    `88,AX13612,B096132,AMALGAMATED CORP. LOCKBOX` and
    `88,DEPOSIT-MISC. RECEIVABLES` (mid-text, chained across *two*
    continuations) folds into one line with the two continuation
    payloads concatenated directly onto the text with no extra commas.
  - `03,9876543210,,010,-500000,,,100,1000000,,,400,2000000,,,190/`
    followed by `88,500000,,,110,1000000,,,072,500000,,,074,500000,,,040/`
    and `88,-1500000,,/` (ordinary field boundary, chained across two
    continuations) folds into one line with a single `,` inserted at each
    fold point, matching `docs/ACCOUNT_IDENTIFIER.md`'s worked example of
    this exact input.
- A trailing `\r` on every physical line (simulating a `\r\n`-terminated
  file) doesn't leak into any emitted line.
- A blank line between two records is skipped, not emitted as an empty
  string and not treated as an error.
- An `88` line as the very first line of the input produces
  `DanglingContinuationRecord`, and iteration continues afterward rather
  than stopping — a well-formed record later in the same input still
  gets yielded normally on a subsequent `next()` call. A bare `"88"`
  line with no comma/payload is still recognized as a continuation (of
  whatever preceded it), not treated as an unrecognized record code.
- An underlying I/O error (e.g. from a reader that fails mid-stream)
  surfaces as `BaiReaderError::Io`, not a panic.
- A full multi-record, multi-continuation input, once fully folded and
  fed line-by-line into a fresh `File` via repeated `push` calls,
  produces the same `File` as manually constructing the equivalent
  already-merged strings by hand (i.e. this module's output is a drop-in
  replacement for the manual merging every other test in this crate does
  today).

## Codegen Note

No large data table here — hand-written, not generated from a spec
appendix, same as every other module except `TypeCode`/`CurrencyCode`.
