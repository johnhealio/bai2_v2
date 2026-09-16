//! An assembled BAI2 file: record type 01 (File Header), zero or more
//! `Group`s (each themselves a 02 + account* + 98 sequence), and record
//! type 99 (File Trailer).
//!
//! See `docs/FILE.md` for the technical spec.

use std::fmt;

use crate::file_header::{FileHeader, FileHeaderError};
use crate::file_trailer::{FileTrailer, FileTrailerError};
use crate::group::{Group, GroupError, GroupViolation};

/// One BAI2 file: a `FileHeader` (01), zero or more `Group`s, and a
/// `FileTrailer` (99), assembled incrementally via `push` in that order
/// — the same shape as `Group` one level up.
///
/// `push` assumes any 88 (Continuation) records have already been merged
/// into the text of whichever record they continue — see `docs/FILE.md`'s
/// "Scope note" (inherited unchanged from `docs/ACCOUNT.md`/`docs/GROUP.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub header: Option<FileHeader>,
    pub groups: Vec<Group>,
    pub trailer: Option<FileTrailer>,
}

/// An error from `File::push`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileError {
    /// A second 01 record was pushed; only one is allowed per file.
    DuplicateFileHeader,
    /// A 99 record was pushed before this file's 01 record.
    FileTrailerBeforeFileHeader,
    /// A 02 record was pushed before this file's 01 record.
    GroupBeforeFileHeader,
    /// A 02 record (opening a new group) or a 99 record (closing the
    /// file) was pushed while the previous group hasn't been closed with
    /// its own 98 record yet.
    PreviousGroupNotClosed,
    /// A 03, 16, 49, or 98 record was pushed with no group currently
    /// open — either none has been started yet, or the last one already
    /// closed.
    NoOpenGroup,
    /// A record was pushed after this file's 99 record already closed it.
    PushAfterFileTrailer,
    /// The record code wasn't "01", "02", "03", "16", "49", "98", or "99".
    UnrecognizedRecordCode(String),
    /// The record code was "88" — continuation merging isn't handled by
    /// this module; the caller must fold it into the preceding record's
    /// text before calling `push`.
    UnmergedContinuationRecord,
    FileHeader(FileHeaderError),
    FileTrailer(FileTrailerError),
    /// Wraps whatever `Group::push` returned when a 02/03/16/49/98 line
    /// was delegated to the currently-open group.
    Group(GroupError),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::DuplicateFileHeader => {
                write!(f, "file: a second file header (01) was pushed")
            }
            FileError::FileTrailerBeforeFileHeader => {
                write!(
                    f,
                    "file: file trailer (99) pushed before a file header (01)"
                )
            }
            FileError::GroupBeforeFileHeader => {
                write!(
                    f,
                    "file: group header (02) pushed before a file header (01)"
                )
            }
            FileError::PreviousGroupNotClosed => {
                write!(
                    f,
                    "file: pushed before the previous group was closed with a 98 record"
                )
            }
            FileError::NoOpenGroup => {
                write!(
                    f,
                    "file: no group is currently open; push a 02 record first"
                )
            }
            FileError::PushAfterFileTrailer => {
                write!(f, "file: record pushed after this file's trailer (99)")
            }
            FileError::UnrecognizedRecordCode(v) => {
                write!(f, "file: unrecognized record code {v:?}")
            }
            FileError::UnmergedContinuationRecord => write!(
                f,
                "file: a continuation (88) record was pushed on its own; merge it into the preceding record's text first"
            ),
            FileError::FileHeader(e) => write!(f, "file: {e}"),
            FileError::FileTrailer(e) => write!(f, "file: {e}"),
            FileError::Group(e) => write!(f, "file: {e}"),
        }
    }
}

impl std::error::Error for FileError {}

/// A violation `File::validate` can find, beyond what `Group::validate`
/// already checks on its own for each group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileViolation {
    /// A violation found in one of this file's groups, identified by its
    /// position in `groups`. `GroupViolation` itself wraps
    /// `AccountViolation` (`docs/GROUP.md`), so this nests all the way
    /// down to the specific account and, where relevant, `TypeCode` that
    /// caused it.
    Group {
        group_index: usize,
        violation: GroupViolation,
    },
    /// The 99 record's File Control Total doesn't equal the algebraic
    /// sum of every group's Group Control Total.
    ControlTotalMismatch { expected: i64, computed: i64 },
    /// The 99 record's Number of Groups doesn't equal the actual number
    /// of groups pushed.
    GroupCountMismatch { expected: u32, actual: usize },
}

impl fmt::Display for FileViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileViolation::Group {
                group_index,
                violation,
            } => {
                write!(f, "file: group {group_index}: {violation}")
            }
            FileViolation::ControlTotalMismatch { expected, computed } => write!(
                f,
                "file: control total {expected} doesn't match computed sum {computed}"
            ),
            FileViolation::GroupCountMismatch { expected, actual } => write!(
                f,
                "file: number of groups {expected} doesn't match actual count {actual}"
            ),
        }
    }
}

/// Whether the last group in `groups`, if any, is still open (no trailer
/// yet).
fn last_group_is_open(groups: &[Group]) -> bool {
    matches!(groups.last(), Some(group) if group.trailer.is_none())
}

impl File {
    /// Returns an empty file ready for `push`. Takes no parameters, same
    /// reasoning as `Group::new()` — nothing above `File` in the
    /// hierarchy for it to need supplied.
    pub fn new() -> File {
        File {
            header: None,
            groups: Vec::new(),
            trailer: None,
        }
    }

    /// Parses one already-merged record's text and adds it to the file.
    /// Only `01` (sets the header), `02` (opens the next `Group`), and
    /// `99` (closes the file) are handled directly — `03`, `16`, `49`,
    /// and `98` are all delegated uniformly to the currently-open group,
    /// which already knows how to route each of those internally.
    pub fn push(&mut self, text: &str) -> Result<(), FileError> {
        let record_code = text.split(',').next().unwrap_or("");
        match record_code {
            "01" => {
                if self.trailer.is_some() {
                    return Err(FileError::PushAfterFileTrailer);
                }
                if self.header.is_some() {
                    return Err(FileError::DuplicateFileHeader);
                }
                self.header = Some(FileHeader::new(text).map_err(FileError::FileHeader)?);
                Ok(())
            }
            "02" => {
                if self.trailer.is_some() {
                    return Err(FileError::PushAfterFileTrailer);
                }
                if self.header.is_none() {
                    return Err(FileError::GroupBeforeFileHeader);
                }
                if last_group_is_open(&self.groups) {
                    return Err(FileError::PreviousGroupNotClosed);
                }
                let mut group = Group::new();
                group.push(text).map_err(FileError::Group)?;
                self.groups.push(group);
                Ok(())
            }
            "03" | "16" | "49" | "98" => {
                if self.trailer.is_some() {
                    return Err(FileError::PushAfterFileTrailer);
                }
                if !last_group_is_open(&self.groups) {
                    return Err(FileError::NoOpenGroup);
                }
                let group = self
                    .groups
                    .last_mut()
                    .expect("just checked an open group exists");
                group.push(text).map_err(FileError::Group)?;
                Ok(())
            }
            "99" => {
                if self.trailer.is_some() {
                    return Err(FileError::PushAfterFileTrailer);
                }
                if self.header.is_none() {
                    return Err(FileError::FileTrailerBeforeFileHeader);
                }
                if last_group_is_open(&self.groups) {
                    return Err(FileError::PreviousGroupNotClosed);
                }
                self.trailer = Some(FileTrailer::new(text).map_err(FileError::FileTrailer)?);
                Ok(())
            }
            "88" => Err(FileError::UnmergedContinuationRecord),
            other => Err(FileError::UnrecognizedRecordCode(other.to_string())),
        }
    }

    /// Checks `Group::validate`'s rules on every group in `groups`, plus
    /// reconciles the 99 record's File Control Total and Number of
    /// Groups against what was actually pushed (only if `trailer` is
    /// present — otherwise both checks are simply skipped, not reported
    /// as violations).
    pub fn validate(&self) -> Vec<FileViolation> {
        let mut violations = Vec::new();

        for (group_index, group) in self.groups.iter().enumerate() {
            violations.extend(
                group
                    .validate()
                    .into_iter()
                    .map(|violation| FileViolation::Group {
                        group_index,
                        violation,
                    }),
            );
        }

        if let Some(trailer) = &self.trailer {
            let computed: i64 = self
                .groups
                .iter()
                .filter_map(|group| group.trailer.map(|t| t.group_control_total))
                .sum();
            if computed != trailer.file_control_total {
                violations.push(FileViolation::ControlTotalMismatch {
                    expected: trailer.file_control_total,
                    computed,
                });
            }

            let actual = self.groups.len();
            if trailer.number_of_groups as usize != actual {
                violations.push(FileViolation::GroupCountMismatch {
                    expected: trailer.number_of_groups,
                    actual,
                });
            }
        }

        violations
    }
}

impl Default for File {
    fn default() -> File {
        File::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::{AccountError, AccountViolation};
    use crate::account_identifier::AccountIdentifierViolation;
    use crate::account_trailer::AccountTrailerError;
    use crate::type_code::TypeCode;

    fn push_full_group(file: &mut File, originator: &str, account_number: &str, total: i64) {
        file.push(&format!("02,,{originator},1,040620/")).unwrap();
        file.push(&format!("03,{account_number},,190,{total},4,0/"))
            .unwrap();
        file.push(&format!("49,{total},2/")).unwrap();
        file.push(&format!("98,{total},1,4/")).unwrap();
    }

    #[test]
    fn pushes_header_then_groups_then_trailer_in_order() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        push_full_group(&mut file, "ORIG1", "111", 100);
        push_full_group(&mut file, "ORIG2", "222", 200);
        file.push("99,300,2,10/").unwrap();

        assert_eq!(
            file.header.as_ref().unwrap().sender_identification,
            "122099999"
        );
        assert_eq!(file.groups.len(), 2);
        assert_eq!(file.trailer.unwrap().file_control_total, 300);
    }

    #[test]
    fn group_before_file_header_errors() {
        let mut file = File::new();
        assert_eq!(
            file.push("02,,ORIG,1,040620/"),
            Err(FileError::GroupBeforeFileHeader)
        );
    }

    #[test]
    fn no_open_group_before_any_group_and_after_group_closed() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        assert_eq!(file.push("03,111,,010,100/"), Err(FileError::NoOpenGroup));
        assert_eq!(file.push("98,100,1,4/"), Err(FileError::NoOpenGroup));

        push_full_group(&mut file, "ORIG1", "111", 100);
        assert_eq!(file.push("03,222,,010,100/"), Err(FileError::NoOpenGroup));
    }

    #[test]
    fn file_trailer_before_file_header_errors() {
        let mut file = File::new();
        assert_eq!(
            file.push("99,100,1,4/"),
            Err(FileError::FileTrailerBeforeFileHeader)
        );
    }

    #[test]
    fn duplicate_file_header_errors() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        assert_eq!(
            file.push("01,999999999,888888888,040620,0200,2,,,2/"),
            Err(FileError::DuplicateFileHeader)
        );
    }

    #[test]
    fn previous_group_not_closed_blocks_new_group_and_file_trailer() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        file.push("02,,ORIG1,1,040620/").unwrap();
        assert_eq!(
            file.push("02,,ORIG2,1,040620/"),
            Err(FileError::PreviousGroupNotClosed)
        );
        assert_eq!(
            file.push("99,100,1,4/"),
            Err(FileError::PreviousGroupNotClosed)
        );
    }

    #[test]
    fn push_after_file_trailer_errors() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        push_full_group(&mut file, "ORIG1", "111", 100);
        file.push("99,100,1,4/").unwrap();
        assert_eq!(
            file.push("02,,ORIG2,1,040620/"),
            Err(FileError::PushAfterFileTrailer)
        );
        assert_eq!(
            file.push("99,100,1,4/"),
            Err(FileError::PushAfterFileTrailer)
        );
    }

    #[test]
    fn unrecognized_and_unmerged_continuation_record_codes_error() {
        let mut file = File::new();
        assert_eq!(
            file.push("77,text/"),
            Err(FileError::UnrecognizedRecordCode("77".to_string()))
        );
        assert_eq!(
            file.push("88,text"),
            Err(FileError::UnmergedContinuationRecord)
        );
    }

    #[test]
    fn malformed_lines_bubble_up_through_two_levels_of_wrapping() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        file.push("02,,ORIG1,1,040620/").unwrap();
        file.push("03,111,,010,100/").unwrap();
        assert_eq!(
            file.push("49,abc,2/"),
            Err(FileError::Group(GroupError::Account(
                AccountError::AccountTrailer(AccountTrailerError::InvalidAccountControlTotal(
                    "abc".to_string()
                ))
            )))
        );
    }

    #[test]
    fn validate_returns_empty_for_conformant_closed_file() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        push_full_group(&mut file, "ORIG1", "111", 100);
        push_full_group(&mut file, "ORIG2", "222", 200);
        file.push("99,300,2,10/").unwrap();
        assert_eq!(file.validate(), vec![]);
    }

    #[test]
    fn validate_wraps_nested_account_violations() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        file.push("02,,ORIG1,1,040620/").unwrap();
        // 010 is Status-level; a present Item Count violates the spec.
        file.push("03,111,,010,100,4,0/").unwrap();
        file.push("49,100,2/").unwrap();
        assert_eq!(
            file.validate(),
            vec![FileViolation::Group {
                group_index: 0,
                violation: GroupViolation::Account {
                    account_index: 0,
                    violation: AccountViolation::AccountIdentifier(
                        AccountIdentifierViolation::ItemCountOnStatusTypeCode {
                            type_code: TypeCode::from("010"),
                        }
                    ),
                },
            }]
        );
    }

    #[test]
    fn validate_flags_control_total_and_group_count_mismatches() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        push_full_group(&mut file, "ORIG1", "111", 100);
        file.push("99,999,5,4/").unwrap();
        assert_eq!(
            file.validate(),
            vec![
                FileViolation::ControlTotalMismatch {
                    expected: 999,
                    computed: 100
                },
                FileViolation::GroupCountMismatch {
                    expected: 5,
                    actual: 1
                },
            ]
        );
    }

    #[test]
    fn validate_skips_trailer_checks_without_a_trailer() {
        let mut file = File::new();
        file.push("01,122099999,123456789,040620,0200,1,,,2/")
            .unwrap();
        push_full_group(&mut file, "ORIG1", "111", 100);
        assert_eq!(file.validate(), vec![]);
    }
}
