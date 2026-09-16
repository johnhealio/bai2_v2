//! BAI2 01 (File Header) record.
//!
//! See `docs/FILE_HEADER.md` for the technical spec.

use std::fmt;

use crate::funds_type::{Date, Time, parse_date, parse_time};

/// A parsed BAI2 01 (File Header) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileHeader {
    pub sender_identification: String,
    pub receiver_identification: String,
    pub file_creation_date: Date,
    /// Required, unlike every other Time-shaped field in this crate.
    pub file_creation_time: Time,
    pub file_identification_number: u32,
    pub physical_record_length: Option<u32>,
    pub block_size: Option<u32>,
    pub version_number: u32,
}

/// An error parsing a 01 (File Header) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileHeaderError {
    /// The record's first field isn't the literal `"01"`.
    WrongRecordCode(String),
    /// Sender Identification is required and was blank or entirely absent.
    MissingSenderIdentification,
    /// Sender Identification contained a `/`, which the spec forbids.
    InvalidSenderIdentification(String),
    /// Receiver Identification is required and was blank or entirely absent.
    MissingReceiverIdentification,
    /// Receiver Identification contained a `/`, which the spec forbids.
    InvalidReceiverIdentification(String),
    /// File Creation Date is required and was blank or entirely absent.
    MissingFileCreationDate,
    /// File Creation Date wasn't a valid `YYMMDD` date.
    InvalidFileCreationDate(String),
    /// File Creation Time is required and was blank or entirely absent.
    MissingFileCreationTime,
    /// File Creation Time wasn't a valid military time (or the `9999`
    /// sentinel).
    InvalidFileCreationTime(String),
    /// File Identification Number is required and was blank or entirely
    /// absent.
    MissingFileIdentificationNumber,
    /// File Identification Number wasn't a valid non-negative integer.
    InvalidFileIdentificationNumber(String),
    /// Physical Record Length wasn't a valid non-negative integer.
    InvalidPhysicalRecordLength(String),
    /// Block Size wasn't a valid non-negative integer.
    InvalidBlockSize(String),
    /// Version Number is required and was blank or entirely absent.
    MissingVersionNumber,
    /// Version Number wasn't a valid non-negative integer.
    InvalidVersionNumber(String),
}

impl fmt::Display for FileHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileHeaderError::WrongRecordCode(v) => {
                write!(f, "file header: record code {v:?} is not \"01\"")
            }
            FileHeaderError::MissingSenderIdentification => {
                write!(f, "file header: missing required sender identification")
            }
            FileHeaderError::InvalidSenderIdentification(v) => {
                write!(f, "file header: sender identification {v:?} contains '/'")
            }
            FileHeaderError::MissingReceiverIdentification => {
                write!(f, "file header: missing required receiver identification")
            }
            FileHeaderError::InvalidReceiverIdentification(v) => {
                write!(f, "file header: receiver identification {v:?} contains '/'")
            }
            FileHeaderError::MissingFileCreationDate => {
                write!(f, "file header: missing required file creation date")
            }
            FileHeaderError::InvalidFileCreationDate(v) => {
                write!(f, "file header: invalid file creation date {v:?}")
            }
            FileHeaderError::MissingFileCreationTime => {
                write!(f, "file header: missing required file creation time")
            }
            FileHeaderError::InvalidFileCreationTime(v) => {
                write!(f, "file header: invalid file creation time {v:?}")
            }
            FileHeaderError::MissingFileIdentificationNumber => {
                write!(
                    f,
                    "file header: missing required file identification number"
                )
            }
            FileHeaderError::InvalidFileIdentificationNumber(v) => {
                write!(f, "file header: invalid file identification number {v:?}")
            }
            FileHeaderError::InvalidPhysicalRecordLength(v) => {
                write!(f, "file header: invalid physical record length {v:?}")
            }
            FileHeaderError::InvalidBlockSize(v) => {
                write!(f, "file header: invalid block size {v:?}")
            }
            FileHeaderError::MissingVersionNumber => {
                write!(f, "file header: missing required version number")
            }
            FileHeaderError::InvalidVersionNumber(v) => {
                write!(f, "file header: invalid version number {v:?}")
            }
        }
    }
}

impl std::error::Error for FileHeaderError {}

impl FileHeader {
    /// Parses one already-merged 01 record.
    pub fn new(text: &str) -> Result<FileHeader, FileHeaderError> {
        let text = text.strip_suffix('/').unwrap_or(text);
        let fields: Vec<&str> = text.split(',').collect();

        let record_code = fields.first().copied().unwrap_or("");
        if record_code != "01" {
            return Err(FileHeaderError::WrongRecordCode(record_code.to_string()));
        }

        let sender_str = fields.get(1).copied().unwrap_or("");
        if sender_str.is_empty() {
            return Err(FileHeaderError::MissingSenderIdentification);
        }
        if sender_str.contains('/') {
            return Err(FileHeaderError::InvalidSenderIdentification(
                sender_str.to_string(),
            ));
        }
        let sender_identification = sender_str.to_string();

        let receiver_str = fields.get(2).copied().unwrap_or("");
        if receiver_str.is_empty() {
            return Err(FileHeaderError::MissingReceiverIdentification);
        }
        if receiver_str.contains('/') {
            return Err(FileHeaderError::InvalidReceiverIdentification(
                receiver_str.to_string(),
            ));
        }
        let receiver_identification = receiver_str.to_string();

        let creation_date_str = fields.get(3).copied().unwrap_or("");
        if creation_date_str.is_empty() {
            return Err(FileHeaderError::MissingFileCreationDate);
        }
        let file_creation_date = parse_date(creation_date_str)
            .map_err(|_| FileHeaderError::InvalidFileCreationDate(creation_date_str.to_string()))?;

        let creation_time_str = fields.get(4).copied().unwrap_or("");
        if creation_time_str.is_empty() {
            return Err(FileHeaderError::MissingFileCreationTime);
        }
        let file_creation_time = parse_time(creation_time_str)
            .map_err(|_| FileHeaderError::InvalidFileCreationTime(creation_time_str.to_string()))?;

        let id_number_str = fields.get(5).copied().unwrap_or("");
        if id_number_str.is_empty() {
            return Err(FileHeaderError::MissingFileIdentificationNumber);
        }
        let file_identification_number = id_number_str.parse::<u32>().map_err(|_| {
            FileHeaderError::InvalidFileIdentificationNumber(id_number_str.to_string())
        })?;

        let record_length_str = fields.get(6).copied().unwrap_or("");
        let physical_record_length = if record_length_str.is_empty() {
            None
        } else {
            Some(record_length_str.parse::<u32>().map_err(|_| {
                FileHeaderError::InvalidPhysicalRecordLength(record_length_str.to_string())
            })?)
        };

        let block_size_str = fields.get(7).copied().unwrap_or("");
        let block_size = if block_size_str.is_empty() {
            None
        } else {
            Some(
                block_size_str
                    .parse::<u32>()
                    .map_err(|_| FileHeaderError::InvalidBlockSize(block_size_str.to_string()))?,
            )
        };

        let version_str = fields.get(8).copied().unwrap_or("");
        if version_str.is_empty() {
            return Err(FileHeaderError::MissingVersionNumber);
        }
        let version_number = version_str
            .parse::<u32>()
            .map_err(|_| FileHeaderError::InvalidVersionNumber(version_str.to_string()))?;

        Ok(FileHeader {
            sender_identification,
            receiver_identification,
            file_creation_date,
            file_creation_time,
            file_identification_number,
            physical_record_length,
            block_size,
            version_number,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_spec_sample() {
        let fh = FileHeader::new("01,122099999,123456789,040621,0200,1,55,,2/").unwrap();
        assert_eq!(fh.sender_identification, "122099999");
        assert_eq!(fh.receiver_identification, "123456789");
        assert_eq!(
            fh.file_creation_date,
            Date {
                year: 4,
                month: 6,
                day: 21
            }
        );
        assert_eq!(fh.file_creation_time, Time(200));
        assert_eq!(fh.file_identification_number, 1);
        assert_eq!(fh.physical_record_length, Some(55));
        assert_eq!(fh.block_size, None);
        assert_eq!(fh.version_number, 2);
    }

    #[test]
    fn physical_record_length_and_block_size_default_when_blank() {
        let fh = FileHeader::new("01,122099999,123456789,040621,0200,1,,,2/").unwrap();
        assert_eq!(fh.physical_record_length, None);
        assert_eq!(fh.block_size, None);
    }

    #[test]
    fn file_creation_time_accepts_9999_sentinel_and_2400_rejects_2401() {
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,9999,1,,,2/")
                .unwrap()
                .file_creation_time,
            Time(9999)
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,2400,1,,,2/")
                .unwrap()
                .file_creation_time,
            Time(2400)
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,2401,1,,,2/"),
            Err(FileHeaderError::InvalidFileCreationTime("2401".to_string()))
        );
    }

    #[test]
    fn missing_required_fields_error() {
        assert_eq!(
            FileHeader::new(",123456789,040621,0200,1,,,2/"),
            Err(FileHeaderError::WrongRecordCode("".to_string()))
        );
        assert_eq!(
            FileHeader::new("01,,123456789,040621,0200,1,,,2/"),
            Err(FileHeaderError::MissingSenderIdentification)
        );
        assert_eq!(
            FileHeader::new("01,122099999,,040621,0200,1,,,2/"),
            Err(FileHeaderError::MissingReceiverIdentification)
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,,0200,1,,,2/"),
            Err(FileHeaderError::MissingFileCreationDate)
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,,1,,,2/"),
            Err(FileHeaderError::MissingFileCreationTime)
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,0200,,,,2/"),
            Err(FileHeaderError::MissingFileIdentificationNumber)
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,0200,1,,,/"),
            Err(FileHeaderError::MissingVersionNumber)
        );
    }

    #[test]
    fn version_number_is_parsed_not_validated_against_two() {
        let fh = FileHeader::new("01,122099999,123456789,040621,0200,1,,,3/").unwrap();
        assert_eq!(fh.version_number, 3);
    }

    #[test]
    fn invalid_numeric_fields_error() {
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,0200,abc,,,2/"),
            Err(FileHeaderError::InvalidFileIdentificationNumber(
                "abc".to_string()
            ))
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,0200,1,abc,,2/"),
            Err(FileHeaderError::InvalidPhysicalRecordLength(
                "abc".to_string()
            ))
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,0200,1,,abc,2/"),
            Err(FileHeaderError::InvalidBlockSize("abc".to_string()))
        );
        assert_eq!(
            FileHeader::new("01,122099999,123456789,040621,0200,1,,,abc/"),
            Err(FileHeaderError::InvalidVersionNumber("abc".to_string()))
        );
    }

    #[test]
    fn slash_in_identification_fields_errors() {
        assert_eq!(
            FileHeader::new("01,SEND/ER,123456789,040621,0200,1,,,2/"),
            Err(FileHeaderError::InvalidSenderIdentification(
                "SEND/ER".to_string()
            ))
        );
        assert_eq!(
            FileHeader::new("01,122099999,REC/EIVER,040621,0200,1,,,2/"),
            Err(FileHeaderError::InvalidReceiverIdentification(
                "REC/EIVER".to_string()
            ))
        );
    }
}
