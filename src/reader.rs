//! Folds BAI2 88 (Continuation) records into the logical line they
//! continue, so every yielded item is ready for `File::push`
//! (`docs/FILE.md`), `Group::push` (`docs/GROUP.md`), or
//! `Account::push` (`docs/ACCOUNT.md`) directly.
//!
//! See `docs/READER.md` for the technical spec.

use std::fmt;
use std::io::{self, BufRead, BufReader, Read};

/// Whether `line` (already trimmed) is an 88 (Continuation) record.
fn is_continuation_line(line: &str) -> bool {
    line == "88" || line.starts_with("88,")
}

/// The payload of an 88 line, i.e. everything after the `88,` prefix
/// (empty if the line was the bare literal `"88"`).
fn continuation_payload(line: &str) -> &str {
    line.strip_prefix("88,").unwrap_or("")
}

/// An iterator that reads physical lines from `R` and yields fully-merged
/// logical BAI2 records, folding any 88 continuations in along the way.
///
/// Assumes one physical record per text line (see `docs/READER.md`'s
/// "Physical records are assumed newline-delimited" note) — a true
/// fixed-width, non-newline-delimited transmission is out of scope.
pub struct BaiReader<R: Read> {
    lines: io::Lines<BufReader<R>>,
    /// One line of lookahead: the next non-blank physical line, already
    /// read and right-trimmed, but not yet folded into anything.
    next_line: Option<String>,
    /// Whether `next_line` reflects the current read position (`false`
    /// right after it's been consumed via `take`).
    primed: bool,
}

/// An error from a `BaiReader`.
#[derive(Debug)]
pub enum BaiReaderError {
    Io(io::Error),
    /// An 88 line appeared with nothing before it to continue.
    DanglingContinuationRecord,
}

impl fmt::Display for BaiReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaiReaderError::Io(e) => write!(f, "bai reader: I/O error: {e}"),
            BaiReaderError::DanglingContinuationRecord => {
                write!(
                    f,
                    "bai reader: a continuation (88) record has nothing to continue"
                )
            }
        }
    }
}

impl std::error::Error for BaiReaderError {}

impl<R: Read> BaiReader<R> {
    /// Wraps any byte source.
    pub fn new(reader: R) -> BaiReader<R> {
        BaiReader {
            lines: BufReader::new(reader).lines(),
            next_line: None,
            primed: false,
        }
    }

    /// Reads physical lines until a non-blank one is found (or EOF),
    /// right-trimming each (dropping a stray `\r` from `\r\n`-terminated
    /// input, and any fixed-width blank padding).
    fn advance_line(&mut self) -> Result<Option<String>, BaiReaderError> {
        loop {
            match self.lines.next() {
                None => return Ok(None),
                Some(Err(e)) => return Err(BaiReaderError::Io(e)),
                Some(Ok(raw)) => {
                    let trimmed = raw.trim_end().to_string();
                    if trimmed.is_empty() {
                        continue;
                    }
                    return Ok(Some(trimmed));
                }
            }
        }
    }

    /// Ensures `next_line` reflects the current read position.
    fn fill_next_line(&mut self) -> Result<(), BaiReaderError> {
        if !self.primed {
            self.next_line = self.advance_line()?;
            self.primed = true;
        }
        Ok(())
    }
}

impl<R: Read> Iterator for BaiReader<R> {
    type Item = Result<String, BaiReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(e) = self.fill_next_line() {
            return Some(Err(e));
        }
        let first = self.next_line.take()?;
        self.primed = false;

        if is_continuation_line(&first) {
            return Some(Err(BaiReaderError::DanglingContinuationRecord));
        }

        let mut buffer = first;
        loop {
            if let Err(e) = self.fill_next_line() {
                return Some(Err(e));
            }
            let continues = self.next_line.as_deref().is_some_and(is_continuation_line);
            if !continues {
                break;
            }
            let continuation = self.next_line.take().expect("just checked it's Some");
            self.primed = false;

            if buffer.ends_with('/') {
                buffer.pop();
                buffer.push(',');
            }
            buffer.push_str(continuation_payload(&continuation));
        }

        Some(Ok(buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn read_all(input: &str) -> Vec<Result<String, BaiReaderError>> {
        BaiReader::new(Cursor::new(input.as_bytes().to_vec())).collect()
    }

    #[test]
    fn a_record_with_no_continuations_round_trips_unchanged() {
        let lines = read_all("01,122099999,123456789,040620,0200,1,,,2/\n");
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0].as_deref().unwrap(),
            "01,122099999,123456789,040620,0200,1,,,2/"
        );
    }

    #[test]
    fn folds_ordinary_field_boundary_continuations_with_inserted_commas() {
        // Spec's own 03/88/88 example (docs/ACCOUNT_IDENTIFIER.md).
        let input = "03,9876543210,,010,-500000,,,100,1000000,,,400,2000000,,,190/\n\
                      88,500000,,,110,1000000,,,072,500000,,,074,500000,,,040/\n\
                      88,-1500000,,/\n";
        let lines = read_all(input);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0].as_deref().unwrap(),
            "03,9876543210,,010,-500000,,,100,1000000,,,400,2000000,,,190,\
             500000,,,110,1000000,,,072,500000,,,074,500000,,,040,-1500000,,/"
        );
    }

    #[test]
    fn folds_mid_text_continuations_with_no_inserted_comma() {
        // Spec's own 16/88/88 example (bai2_doc.pdf, "Sample 88 Record").
        let input = "16,115,10000000,S,5000000,4000000,1000000/\n\
                      88,AX13612,B096132,AMALGAMATED CORP. LOCKBOX\n\
                      88,DEPOSIT-MISC. RECEIVABLES\n";
        let lines = read_all(input);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0].as_deref().unwrap(),
            "16,115,10000000,S,5000000,4000000,1000000,\
             AX13612,B096132,AMALGAMATED CORP. LOCKBOXDEPOSIT-MISC. RECEIVABLES"
        );
    }

    #[test]
    fn strips_trailing_carriage_returns() {
        let lines = read_all("01,A,B,040620,0200,1,,,2/\r\n02,,ORIG,1,040620/\r\n");
        assert_eq!(lines.len(), 2);
        for line in &lines {
            assert!(!line.as_ref().unwrap().contains('\r'));
        }
    }

    #[test]
    fn blank_lines_between_records_are_skipped() {
        let lines = read_all("01,A,B,040620,0200,1,,,2/\n\n\n02,,ORIG,1,040620/\n");
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].as_deref().unwrap(), "01,A,B,040620,0200,1,,,2/");
        assert_eq!(lines[1].as_deref().unwrap(), "02,,ORIG,1,040620/");
    }

    #[test]
    fn a_leading_continuation_with_nothing_to_continue_errors() {
        let mut lines = read_all("88,orphaned text\n01,A,B,040620,0200,1,,,2/\n").into_iter();
        assert!(matches!(
            lines.next(),
            Some(Err(BaiReaderError::DanglingContinuationRecord))
        ));
        assert_eq!(lines.next().unwrap().unwrap(), "01,A,B,040620,0200,1,,,2/");
        assert!(lines.next().is_none());
    }

    #[test]
    fn a_bare_88_with_no_comma_is_still_recognized_as_a_continuation() {
        let lines = read_all("01,A,B,040620,0200,1,,,2/\n88\n");
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].as_deref().unwrap(), "01,A,B,040620,0200,1,,,2,");
    }

    #[test]
    fn io_errors_surface_without_panicking() {
        struct FailingRead;
        impl Read for FailingRead {
            fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("boom"))
            }
        }
        let mut reader = BaiReader::new(FailingRead);
        assert!(matches!(reader.next(), Some(Err(BaiReaderError::Io(_)))));
    }

    #[test]
    fn feeds_a_full_file_end_to_end_into_file_push() {
        let input = "01,S,R,040620,0200,1,,,2/\n\
                      02,,ORIG,1,040620/\n\
                      03,111,,190,100,4,0/\n\
                      49,100,2/\n\
                      98,100,1,4/\n\
                      99,100,1,6/\n";
        let mut file = crate::file::File::new();
        for line in read_all(input) {
            file.push(&line.unwrap()).unwrap();
        }
        assert_eq!(file.groups.len(), 1);
        assert_eq!(file.validate(), vec![]);
    }
}
