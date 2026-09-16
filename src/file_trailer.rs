//! BAI2 99 (File Trailer) record.
//!
//! See `docs/FILE_TRAILER.md` for the technical spec.

use std::fmt;

/// A parsed BAI2 99 (File Trailer) record.
///
/// All three fields are genuinely mandatory — the spec states "All
/// fields are required" outright, so a blank field is a parse error, not
/// a default. Structurally identical in shape to `GroupTrailer`
/// (`docs/GROUP_TRAILER.md`), just one level up the hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTrailer {
    /// Algebraic sum of every 98 record's Group Control Total in this
    /// file.
    pub file_control_total: i64,
    /// Count of 02 records in this file.
    pub number_of_groups: u32,
    /// Total count of every record of every code in the file, including
    /// this 99 record.
    pub number_of_records: u32,
}

/// An error parsing a 99 (File Trailer) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileTrailerError {
    /// The record's first field isn't the literal `"99"`.
    WrongRecordCode(String),
    /// File Control Total is required and was blank or entirely absent.
    MissingFileControlTotal,
    /// File Control Total wasn't a valid signed integer.
    InvalidFileControlTotal(String),
    /// Number of Groups is required and was blank or entirely absent.
    MissingNumberOfGroups,
    /// Number of Groups wasn't a valid non-negative integer.
    InvalidNumberOfGroups(String),
    /// Number of Records is required and was blank or entirely absent.
    MissingNumberOfRecords,
    /// Number of Records wasn't a valid non-negative integer.
    InvalidNumberOfRecords(String),
}

impl fmt::Display for FileTrailerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileTrailerError::WrongRecordCode(v) => {
                write!(f, "file trailer: record code {v:?} is not \"99\"")
            }
            FileTrailerError::MissingFileControlTotal => {
                write!(f, "file trailer: missing required file control total")
            }
            FileTrailerError::InvalidFileControlTotal(v) => {
                write!(f, "file trailer: invalid file control total {v:?}")
            }
            FileTrailerError::MissingNumberOfGroups => {
                write!(f, "file trailer: missing required number of groups")
            }
            FileTrailerError::InvalidNumberOfGroups(v) => {
                write!(f, "file trailer: invalid number of groups {v:?}")
            }
            FileTrailerError::MissingNumberOfRecords => {
                write!(f, "file trailer: missing required number of records")
            }
            FileTrailerError::InvalidNumberOfRecords(v) => {
                write!(f, "file trailer: invalid number of records {v:?}")
            }
        }
    }
}

impl std::error::Error for FileTrailerError {}

impl FileTrailer {
    /// Parses one 99 record line.
    pub fn new(line: &str) -> Result<FileTrailer, FileTrailerError> {
        let line = line.strip_suffix('/').unwrap_or(line);
        let fields: Vec<&str> = line.split(',').collect();

        let record_code = fields.first().copied().unwrap_or("");
        if record_code != "99" {
            return Err(FileTrailerError::WrongRecordCode(record_code.to_string()));
        }

        let total_str = fields.get(1).copied().unwrap_or("");
        if total_str.is_empty() {
            return Err(FileTrailerError::MissingFileControlTotal);
        }
        let file_control_total = total_str
            .parse::<i64>()
            .map_err(|_| FileTrailerError::InvalidFileControlTotal(total_str.to_string()))?;

        let groups_str = fields.get(2).copied().unwrap_or("");
        if groups_str.is_empty() {
            return Err(FileTrailerError::MissingNumberOfGroups);
        }
        let number_of_groups = groups_str
            .parse::<u32>()
            .map_err(|_| FileTrailerError::InvalidNumberOfGroups(groups_str.to_string()))?;

        let records_str = fields.get(3).copied().unwrap_or("");
        if records_str.is_empty() {
            return Err(FileTrailerError::MissingNumberOfRecords);
        }
        let number_of_records = records_str
            .parse::<u32>()
            .map_err(|_| FileTrailerError::InvalidNumberOfRecords(records_str.to_string()))?;

        Ok(FileTrailer {
            file_control_total,
            number_of_groups,
            number_of_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_spec_sample() {
        assert_eq!(
            FileTrailer::new("99,1215450000,4,36/"),
            Ok(FileTrailer {
                file_control_total: 1_215_450_000,
                number_of_groups: 4,
                number_of_records: 36,
            })
        );
    }

    #[test]
    fn negative_and_explicit_positive_signs_parse_correctly() {
        assert_eq!(
            FileTrailer::new("99,-1215450000,4,36/")
                .unwrap()
                .file_control_total,
            -1_215_450_000
        );
        assert_eq!(
            FileTrailer::new("99,+1215450000,4,36/")
                .unwrap()
                .file_control_total,
            1_215_450_000
        );
    }

    #[test]
    fn extra_trailing_fields_are_ignored() {
        assert_eq!(
            FileTrailer::new("99,1215450000,4,36,ignored,fields/"),
            Ok(FileTrailer {
                file_control_total: 1_215_450_000,
                number_of_groups: 4,
                number_of_records: 36,
            })
        );
    }

    #[test]
    fn wrong_record_code_errors() {
        assert_eq!(
            FileTrailer::new("98,1215450000,4,36/"),
            Err(FileTrailerError::WrongRecordCode("98".to_string()))
        );
    }

    #[test]
    fn missing_fields_error() {
        assert_eq!(
            FileTrailer::new("99,,4,36/"),
            Err(FileTrailerError::MissingFileControlTotal)
        );
        assert_eq!(
            FileTrailer::new("99,1215450000,,36/"),
            Err(FileTrailerError::MissingNumberOfGroups)
        );
        assert_eq!(
            FileTrailer::new("99,1215450000,4,/"),
            Err(FileTrailerError::MissingNumberOfRecords)
        );
        assert_eq!(
            FileTrailer::new("99"),
            Err(FileTrailerError::MissingFileControlTotal)
        );
    }

    #[test]
    fn invalid_fields_error() {
        assert_eq!(
            FileTrailer::new("99,abc,4,36/"),
            Err(FileTrailerError::InvalidFileControlTotal("abc".to_string()))
        );
        assert_eq!(
            FileTrailer::new("99,1215450000,abc,36/"),
            Err(FileTrailerError::InvalidNumberOfGroups("abc".to_string()))
        );
        assert_eq!(
            FileTrailer::new("99,1215450000,4,abc/"),
            Err(FileTrailerError::InvalidNumberOfRecords("abc".to_string()))
        );
    }

    #[test]
    fn negative_counts_error() {
        assert_eq!(
            FileTrailer::new("99,1215450000,-4,36/"),
            Err(FileTrailerError::InvalidNumberOfGroups("-4".to_string()))
        );
        assert_eq!(
            FileTrailer::new("99,1215450000,4,-36/"),
            Err(FileTrailerError::InvalidNumberOfRecords("-36".to_string()))
        );
    }
}
