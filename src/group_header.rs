//! BAI2 02 (Group Header) record.
//!
//! See `docs/GROUP_HEADER.md` for the technical spec.

use std::fmt;

use crate::currency_code::CurrencyCode;
use crate::funds_type::{Date, Time, parse_date, parse_time};

/// How data in this group of accounts is to be processed. This module
/// only parses the code — the spec's "GROUP STATUS" Data Element goes on
/// to define update/delete/correct processing semantics for each value,
/// none of which is implemented here; acting on the value is the
/// caller's concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupStatus {
    /// Most transmissions; contains an 03 record for each account.
    Update,
    Deletion,
    Correction,
    TestOnly,
}

/// Distinguishes same-day data from previous-day data, and interim data
/// from final data. "For identification only and does not affect
/// processing."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsOfDateModifier {
    InterimPreviousDay,
    FinalPreviousDay,
    InterimSameDay,
    FinalSameDay,
}

/// A parsed BAI2 02 (Group Header) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupHeader {
    pub ultimate_receiver_identification: Option<String>,
    pub originator_identification: String,
    pub group_status: GroupStatus,
    pub as_of_date: Date,
    pub as_of_time: Option<Time>,
    /// This record's own Currency Code field, or `CurrencyCode::Usd` if
    /// that field was blank — the spec's default here is a fixed value,
    /// not derived from anything else (contrast
    /// `AccountIdentifier.currency`, whose default is *this* value —
    /// see `docs/ACCOUNT_IDENTIFIER.md`).
    pub currency: CurrencyCode,
    pub as_of_date_modifier: Option<AsOfDateModifier>,
}

/// An error parsing a 02 (Group Header) record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupHeaderError {
    /// The record's first field isn't the literal `"02"`.
    WrongRecordCode(String),
    /// Ultimate Receiver Identification contained a `/`, which the spec
    /// forbids.
    InvalidUltimateReceiverIdentification(String),
    /// Originator Identification is required and was blank or entirely
    /// absent.
    MissingOriginatorIdentification,
    /// Originator Identification contained a `/`, which the spec forbids.
    InvalidOriginatorIdentification(String),
    /// Group Status is required and was blank or entirely absent.
    MissingGroupStatus,
    /// Group Status wasn't one of `1`-`4`.
    InvalidGroupStatus(String),
    /// As-of-Date is required and was blank or entirely absent.
    MissingAsOfDate,
    /// As-of-Date wasn't a valid `YYMMDD` date.
    InvalidAsOfDate(String),
    /// As-of-Time wasn't a valid military time (or the `9999` sentinel).
    InvalidAsOfTime(String),
    /// As-of-Date Modifier wasn't one of `1`-`4`.
    InvalidAsOfDateModifier(String),
}

impl fmt::Display for GroupHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupHeaderError::WrongRecordCode(v) => {
                write!(f, "group header: record code {v:?} is not \"02\"")
            }
            GroupHeaderError::InvalidUltimateReceiverIdentification(v) => write!(
                f,
                "group header: ultimate receiver identification {v:?} contains '/'"
            ),
            GroupHeaderError::MissingOriginatorIdentification => {
                write!(
                    f,
                    "group header: missing required originator identification"
                )
            }
            GroupHeaderError::InvalidOriginatorIdentification(v) => {
                write!(
                    f,
                    "group header: originator identification {v:?} contains '/'"
                )
            }
            GroupHeaderError::MissingGroupStatus => {
                write!(f, "group header: missing required group status")
            }
            GroupHeaderError::InvalidGroupStatus(v) => {
                write!(f, "group header: invalid group status {v:?}")
            }
            GroupHeaderError::MissingAsOfDate => {
                write!(f, "group header: missing required as-of-date")
            }
            GroupHeaderError::InvalidAsOfDate(v) => {
                write!(f, "group header: invalid as-of-date {v:?}")
            }
            GroupHeaderError::InvalidAsOfTime(v) => {
                write!(f, "group header: invalid as-of-time {v:?}")
            }
            GroupHeaderError::InvalidAsOfDateModifier(v) => {
                write!(f, "group header: invalid as-of-date modifier {v:?}")
            }
        }
    }
}

impl std::error::Error for GroupHeaderError {}

impl GroupHeader {
    /// Parses one 02 record line.
    pub fn new(text: &str) -> Result<GroupHeader, GroupHeaderError> {
        let text = text.strip_suffix('/').unwrap_or(text);
        let fields: Vec<&str> = text.split(',').collect();

        let record_code = fields.first().copied().unwrap_or("");
        if record_code != "02" {
            return Err(GroupHeaderError::WrongRecordCode(record_code.to_string()));
        }

        let ultimate_receiver_str = fields.get(1).copied().unwrap_or("");
        let ultimate_receiver_identification = if ultimate_receiver_str.is_empty() {
            None
        } else if ultimate_receiver_str.contains('/') {
            return Err(GroupHeaderError::InvalidUltimateReceiverIdentification(
                ultimate_receiver_str.to_string(),
            ));
        } else {
            Some(ultimate_receiver_str.to_string())
        };

        let originator_str = fields.get(2).copied().unwrap_or("");
        if originator_str.is_empty() {
            return Err(GroupHeaderError::MissingOriginatorIdentification);
        }
        if originator_str.contains('/') {
            return Err(GroupHeaderError::InvalidOriginatorIdentification(
                originator_str.to_string(),
            ));
        }
        let originator_identification = originator_str.to_string();

        let group_status_str = fields.get(3).copied().unwrap_or("");
        if group_status_str.is_empty() {
            return Err(GroupHeaderError::MissingGroupStatus);
        }
        let group_status = match group_status_str {
            "1" => GroupStatus::Update,
            "2" => GroupStatus::Deletion,
            "3" => GroupStatus::Correction,
            "4" => GroupStatus::TestOnly,
            other => return Err(GroupHeaderError::InvalidGroupStatus(other.to_string())),
        };

        let as_of_date_str = fields.get(4).copied().unwrap_or("");
        if as_of_date_str.is_empty() {
            return Err(GroupHeaderError::MissingAsOfDate);
        }
        let as_of_date = parse_date(as_of_date_str)
            .map_err(|_| GroupHeaderError::InvalidAsOfDate(as_of_date_str.to_string()))?;

        let as_of_time_str = fields.get(5).copied().unwrap_or("");
        let as_of_time = if as_of_time_str.is_empty() {
            None
        } else {
            Some(
                parse_time(as_of_time_str)
                    .map_err(|_| GroupHeaderError::InvalidAsOfTime(as_of_time_str.to_string()))?,
            )
        };

        let currency_str = fields.get(6).copied().unwrap_or("");
        let currency = if currency_str.is_empty() {
            CurrencyCode::Usd
        } else {
            CurrencyCode::from(currency_str)
        };

        let as_of_date_modifier_str = fields.get(7).copied().unwrap_or("");
        let as_of_date_modifier = if as_of_date_modifier_str.is_empty() {
            None
        } else {
            Some(match as_of_date_modifier_str {
                "1" => AsOfDateModifier::InterimPreviousDay,
                "2" => AsOfDateModifier::FinalPreviousDay,
                "3" => AsOfDateModifier::InterimSameDay,
                "4" => AsOfDateModifier::FinalSameDay,
                other => return Err(GroupHeaderError::InvalidAsOfDateModifier(other.to_string())),
            })
        };

        Ok(GroupHeader {
            ultimate_receiver_identification,
            originator_identification,
            group_status,
            as_of_date,
            as_of_time,
            currency,
            as_of_date_modifier,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_spec_sample() {
        let gh = GroupHeader::new("02,031001234,122099999,1,040620,2359,,2/").unwrap();
        assert_eq!(
            gh.ultimate_receiver_identification,
            Some("031001234".to_string())
        );
        assert_eq!(gh.originator_identification, "122099999");
        assert_eq!(gh.group_status, GroupStatus::Update);
        assert_eq!(
            gh.as_of_date,
            Date {
                year: 4,
                month: 6,
                day: 20
            }
        );
        assert_eq!(gh.as_of_time, Some(Time(2359)));
        assert_eq!(gh.currency, CurrencyCode::Usd);
        assert_eq!(
            gh.as_of_date_modifier,
            Some(AsOfDateModifier::FinalPreviousDay)
        );
    }

    #[test]
    fn optional_fields_default_when_blank_via_commas() {
        let gh = GroupHeader::new("02,,ORIG123,1,040620,,,/").unwrap();
        assert_eq!(gh.ultimate_receiver_identification, None);
        assert_eq!(gh.as_of_time, None);
        assert_eq!(gh.currency, CurrencyCode::Usd);
        assert_eq!(gh.as_of_date_modifier, None);
    }

    #[test]
    fn optional_trailing_fields_default_when_entirely_truncated() {
        let gh = GroupHeader::new("02,,ORIG123,1,040620").unwrap();
        assert_eq!(gh.as_of_time, None);
        assert_eq!(gh.currency, CurrencyCode::Usd);
        assert_eq!(gh.as_of_date_modifier, None);
    }

    #[test]
    fn explicit_currency_overrides_default() {
        let gh = GroupHeader::new("02,,ORIG123,1,040620,,JPY,2/").unwrap();
        assert_eq!(gh.currency, CurrencyCode::Jpy);
    }

    #[test]
    fn as_of_time_accepts_9999_sentinel_and_2400_rejects_2401() {
        assert_eq!(
            GroupHeader::new("02,,ORIG123,1,040620,9999,,/")
                .unwrap()
                .as_of_time,
            Some(Time(9999))
        );
        assert_eq!(
            GroupHeader::new("02,,ORIG123,1,040620,2400,,/")
                .unwrap()
                .as_of_time,
            Some(Time(2400))
        );
        assert_eq!(
            GroupHeader::new("02,,ORIG123,1,040620,2401,,/"),
            Err(GroupHeaderError::InvalidAsOfTime("2401".to_string()))
        );
    }

    #[test]
    fn missing_group_status_and_as_of_date_error() {
        assert_eq!(
            GroupHeader::new("02,,ORIG123,,040620/"),
            Err(GroupHeaderError::MissingGroupStatus)
        );
        assert_eq!(
            GroupHeader::new("02,,ORIG123,1,/"),
            Err(GroupHeaderError::MissingAsOfDate)
        );
    }

    #[test]
    fn out_of_range_group_status_and_malformed_as_of_date_error() {
        assert_eq!(
            GroupHeader::new("02,,ORIG123,5,040620/"),
            Err(GroupHeaderError::InvalidGroupStatus("5".to_string()))
        );
        assert_eq!(
            GroupHeader::new("02,,ORIG123,1,birthday/"),
            Err(GroupHeaderError::InvalidAsOfDate("birthday".to_string()))
        );
    }

    #[test]
    fn out_of_range_as_of_date_modifier_errors_distinct_from_absent() {
        assert_eq!(
            GroupHeader::new("02,,ORIG123,1,040620,,,5/"),
            Err(GroupHeaderError::InvalidAsOfDateModifier("5".to_string()))
        );
        assert_eq!(
            GroupHeader::new("02,,ORIG123,1,040620/")
                .unwrap()
                .as_of_date_modifier,
            None
        );
    }

    #[test]
    fn wrong_record_code_errors() {
        assert_eq!(
            GroupHeader::new("03,,ORIG123,1,040620/"),
            Err(GroupHeaderError::WrongRecordCode("03".to_string()))
        );
    }

    #[test]
    fn slash_in_identification_fields_errors() {
        assert_eq!(
            GroupHeader::new("02,REC/123,ORIG123,1,040620/"),
            Err(GroupHeaderError::InvalidUltimateReceiverIdentification(
                "REC/123".to_string()
            ))
        );
        assert_eq!(
            GroupHeader::new("02,,ORIG/123,1,040620/"),
            Err(GroupHeaderError::InvalidOriginatorIdentification(
                "ORIG/123".to_string()
            ))
        );
    }

    #[test]
    fn missing_originator_identification_errors() {
        assert_eq!(
            GroupHeader::new("02,,,1,040620/"),
            Err(GroupHeaderError::MissingOriginatorIdentification)
        );
    }
}
