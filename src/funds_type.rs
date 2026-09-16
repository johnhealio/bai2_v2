//! BAI2 "Funds Type" composite field (03 and 16 records).
//!
//! See `docs/FUND_TYPE.md` for the technical spec.

use std::fmt;

/// A YYMMDD date as it appears in a BAI2 file. The two-digit year is
/// transcribed as-is; the spec defines no century-inference rule, so callers
/// that need a real calendar date must supply their own windowing policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: u8,
    pub month: u8,
    pub day: u8,
}

/// A military-format time of day (`0000`-`2400`), as it appears in a BAI2
/// file. `9999` is accepted as the spec's documented nonstandard end-of-day
/// sentinel; stored as the raw four digits rather than hour/minute since
/// `99:99` doesn't decompose sensibly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time(pub u16);

/// The value date/time for Funds Type `V` (value dated).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueDate {
    pub date: Date,
    /// Optional; `None` if defaulted by adjacent delimiters.
    pub time: Option<Time>,
}

/// The three fixed availability buckets for Funds Type `S` (distributed
/// availability, fixed form). Amounts share the currency code and implied
/// decimal of the "Amount" field they describe, and may be signed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DistributedAvailability {
    pub immediate: i64,
    pub one_day: i64,
    pub two_or_more_days: i64,
}

/// One (days, amount) pair for Funds Type `D` (distributed availability,
/// itemized form).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DistributionDay {
    pub days: u32,
    pub amount: i64,
}

/// The availability of an Amount reported in a 03 or 16 record.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FundsType {
    /// Unknown (default) — `Z`, or the field defaulted by adjacent delimiters.
    Unknown,
    /// Immediate availability — `0`.
    Immediate,
    /// One day availability — `1`.
    OneDay,
    /// Two-or-more days availability — `2`.
    TwoOrMoreDays,
    /// Value dated — `V`.
    ValueDated(ValueDate),
    /// Distributed availability, fixed three-bucket form — `S`.
    Distributed(DistributedAvailability),
    /// Distributed availability, itemized (days, amount) form — `D`.
    DistributedDays(Vec<DistributionDay>),
}

/// An error parsing a Funds Type composite field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FundsTypeError {
    /// No fields were given at all.
    Empty,
    /// The leading code character isn't one of `Z 0 1 2 V S D` (or blank).
    UnknownCode(String),
    /// A required sub-field for the given code was missing.
    MissingField { code: char, field: &'static str },
    /// A sub-field that should have been a 6-digit YYMMDD date wasn't.
    InvalidDate(String),
    /// A sub-field that should have been a 4-digit military time wasn't.
    InvalidTime(String),
    /// A sub-field that should have been an integer wasn't.
    InvalidInteger { field: &'static str, value: String },
    /// Funds Type `D`'s distribution count didn't match the number of
    /// (days, amount) pairs actually present.
    DistributionCountMismatch { expected: u32, found: usize },
}

impl fmt::Display for FundsTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FundsTypeError::Empty => write!(f, "funds type: no fields given"),
            FundsTypeError::UnknownCode(c) => write!(f, "funds type: unknown code {c:?}"),
            FundsTypeError::MissingField { code, field } => {
                write!(f, "funds type {code}: missing {field} field")
            }
            FundsTypeError::InvalidDate(v) => write!(f, "funds type: invalid date {v:?}"),
            FundsTypeError::InvalidTime(v) => write!(f, "funds type: invalid time {v:?}"),
            FundsTypeError::InvalidInteger { field, value } => {
                write!(f, "funds type: invalid {field} {value:?}")
            }
            FundsTypeError::DistributionCountMismatch { expected, found } => write!(
                f,
                "funds type D: distribution count {expected} doesn't match {found} (days, amount) pairs found"
            ),
        }
    }
}

impl std::error::Error for FundsTypeError {}

fn parse_date(s: &str) -> Result<Date, FundsTypeError> {
    if s.len() != 6 || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(FundsTypeError::InvalidDate(s.to_string()));
    }
    let year: u8 = s[0..2].parse().unwrap();
    let month: u8 = s[2..4].parse().unwrap();
    let day: u8 = s[4..6].parse().unwrap();
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(FundsTypeError::InvalidDate(s.to_string()));
    }
    Ok(Date { year, month, day })
}

fn parse_time(s: &str) -> Result<Time, FundsTypeError> {
    if s.len() != 4 || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(FundsTypeError::InvalidTime(s.to_string()));
    }
    let raw: u16 = s.parse().unwrap();
    if raw > 2400 && raw != 9999 {
        return Err(FundsTypeError::InvalidTime(s.to_string()));
    }
    Ok(Time(raw))
}

fn parse_amount(code: char, field: &'static str, s: Option<&&str>) -> Result<i64, FundsTypeError> {
    let s = s.ok_or(FundsTypeError::MissingField { code, field })?;
    s.parse().map_err(|_| FundsTypeError::InvalidInteger {
        field,
        value: (*s).to_string(),
    })
}

fn parse_count(code: char, field: &'static str, s: Option<&&str>) -> Result<u32, FundsTypeError> {
    let s = s.ok_or(FundsTypeError::MissingField { code, field })?;
    s.parse().map_err(|_| FundsTypeError::InvalidInteger {
        field,
        value: (*s).to_string(),
    })
}

impl FundsType {
    /// Parses a Funds Type composite field from its already comma-split
    /// sub-fields, starting at the code character (`fields[0]`).
    ///
    /// Returns the parsed value and the number of leading entries in
    /// `fields` it consumed, so a record parser can slice past them to reach
    /// whatever field follows the Funds Type composite.
    pub fn parse(fields: &[&str]) -> Result<(FundsType, usize), FundsTypeError> {
        let code = *fields.first().ok_or(FundsTypeError::Empty)?;
        match code {
            "" | "Z" => Ok((FundsType::Unknown, 1)),
            "0" => Ok((FundsType::Immediate, 1)),
            "1" => Ok((FundsType::OneDay, 1)),
            "2" => Ok((FundsType::TwoOrMoreDays, 1)),
            "V" => {
                let date_str = fields.get(1).ok_or(FundsTypeError::MissingField {
                    code: 'V',
                    field: "value date",
                })?;
                let date = parse_date(date_str)?;
                let time = match fields.get(2) {
                    None => None,
                    Some(&"") => None,
                    Some(t) => Some(parse_time(t)?),
                };
                Ok((FundsType::ValueDated(ValueDate { date, time }), 3))
            }
            "S" => {
                let immediate = parse_amount('S', "immediate availability", fields.get(1))?;
                let one_day = parse_amount('S', "one-day availability", fields.get(2))?;
                let two_or_more_days =
                    parse_amount('S', "two-or-more days availability", fields.get(3))?;
                Ok((
                    FundsType::Distributed(DistributedAvailability {
                        immediate,
                        one_day,
                        two_or_more_days,
                    }),
                    4,
                ))
            }
            "D" => {
                let count = parse_count('D', "number of distributions", fields.get(1))?;
                let mut days = Vec::with_capacity(count as usize);
                let mut idx = 2;
                for _ in 0..count {
                    let d = parse_count('D', "availability in days", fields.get(idx))?;
                    let amount = parse_amount('D', "availability amount", fields.get(idx + 1))?;
                    days.push(DistributionDay { days: d, amount });
                    idx += 2;
                }
                if days.len() != count as usize {
                    return Err(FundsTypeError::DistributionCountMismatch {
                        expected: count,
                        found: days.len(),
                    });
                }
                Ok((FundsType::DistributedDays(days), idx))
            }
            other => Err(FundsTypeError::UnknownCode(other.to_string())),
        }
    }

    /// The single-character code this variant serializes as (`Z 0 1 2 V S D`).
    pub fn code(&self) -> &'static str {
        match self {
            FundsType::Unknown => "Z",
            FundsType::Immediate => "0",
            FundsType::OneDay => "1",
            FundsType::TwoOrMoreDays => "2",
            FundsType::ValueDated(_) => "V",
            FundsType::Distributed(_) => "S",
            FundsType::DistributedDays(_) => "D",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_codes() {
        assert_eq!(FundsType::parse(&["Z"]), Ok((FundsType::Unknown, 1)));
        assert_eq!(FundsType::parse(&[""]), Ok((FundsType::Unknown, 1)));
        assert_eq!(FundsType::parse(&["0"]), Ok((FundsType::Immediate, 1)));
        assert_eq!(FundsType::parse(&["1"]), Ok((FundsType::OneDay, 1)));
        assert_eq!(FundsType::parse(&["2"]), Ok((FundsType::TwoOrMoreDays, 1)));
    }

    #[test]
    fn parses_value_dated_with_time() {
        let (ft, consumed) = FundsType::parse(&["V", "040701", "1300"]).unwrap();
        assert_eq!(consumed, 3);
        assert_eq!(
            ft,
            FundsType::ValueDated(ValueDate {
                date: Date {
                    year: 4,
                    month: 7,
                    day: 1
                },
                time: Some(Time(1300)),
            })
        );
    }

    #[test]
    fn parses_value_dated_with_defaulted_time() {
        let (ft, consumed) = FundsType::parse(&["V", "040701", ""]).unwrap();
        assert_eq!(consumed, 3);
        assert_eq!(
            ft,
            FundsType::ValueDated(ValueDate {
                date: Date {
                    year: 4,
                    month: 7,
                    day: 1
                },
                time: None,
            })
        );
    }

    #[test]
    fn parses_value_dated_with_missing_time_field() {
        let (ft, consumed) = FundsType::parse(&["V", "040701"]).unwrap();
        assert_eq!(consumed, 3);
        assert_eq!(
            ft,
            FundsType::ValueDated(ValueDate {
                date: Date {
                    year: 4,
                    month: 7,
                    day: 1
                },
                time: None,
            })
        );
    }

    #[test]
    fn rejects_end_of_day_9999_time_but_accepts_2400() {
        assert!(FundsType::parse(&["V", "040701", "9999"]).is_ok());
        assert!(FundsType::parse(&["V", "040701", "2400"]).is_ok());
        assert!(FundsType::parse(&["V", "040701", "2401"]).is_err());
    }

    #[test]
    fn parses_distributed_s() {
        let (ft, consumed) = FundsType::parse(&["S", "150000", "100000", "90000"]).unwrap();
        assert_eq!(consumed, 4);
        assert_eq!(
            ft,
            FundsType::Distributed(DistributedAvailability {
                immediate: 150000,
                one_day: 100000,
                two_or_more_days: 90000,
            })
        );
    }

    #[test]
    fn parses_distributed_s_negative_amount() {
        let (ft, _) = FundsType::parse(&["S", "-150000", "100000", "90000"]).unwrap();
        assert_eq!(
            ft,
            FundsType::Distributed(DistributedAvailability {
                immediate: -150000,
                one_day: 100000,
                two_or_more_days: 90000,
            })
        );
    }

    #[test]
    fn parses_distributed_d() {
        let (ft, consumed) =
            FundsType::parse(&["D", "3", "0", "150000", "1", "100000", "2", "90000"]).unwrap();
        assert_eq!(consumed, 8);
        assert_eq!(
            ft,
            FundsType::DistributedDays(vec![
                DistributionDay {
                    days: 0,
                    amount: 150000
                },
                DistributionDay {
                    days: 1,
                    amount: 100000
                },
                DistributionDay {
                    days: 2,
                    amount: 90000
                },
            ])
        );
    }

    #[test]
    fn distributed_d_with_zero_distributions() {
        let (ft, consumed) = FundsType::parse(&["D", "0"]).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(ft, FundsType::DistributedDays(vec![]));
    }

    #[test]
    fn distributed_d_stops_at_first_extra_field() {
        // Trailing fields beyond the declared count belong to whatever
        // follows the composite, not to this Funds Type.
        let (_, consumed) = FundsType::parse(&["D", "1", "0", "150000", "16", "text"]).unwrap();
        assert_eq!(consumed, 4);
    }

    #[test]
    fn s_missing_field_errors() {
        assert_eq!(
            FundsType::parse(&["S", "150000"]),
            Err(FundsTypeError::MissingField {
                code: 'S',
                field: "one-day availability"
            })
        );
    }

    #[test]
    fn d_short_of_declared_count_errors() {
        assert_eq!(
            FundsType::parse(&["D", "2", "0", "150000"]),
            Err(FundsTypeError::MissingField {
                code: 'D',
                field: "availability in days"
            })
        );
    }

    #[test]
    fn unknown_code_errors() {
        assert_eq!(
            FundsType::parse(&["X"]),
            Err(FundsTypeError::UnknownCode("X".to_string()))
        );
    }

    #[test]
    fn empty_input_errors() {
        assert_eq!(FundsType::parse(&[]), Err(FundsTypeError::Empty));
    }

    #[test]
    fn code_round_trips() {
        assert_eq!(FundsType::Unknown.code(), "Z");
        assert_eq!(FundsType::Immediate.code(), "0");
        assert_eq!(FundsType::OneDay.code(), "1");
        assert_eq!(FundsType::TwoOrMoreDays.code(), "2");
    }
}
