//! ISO8601 is a parser library for the
//! [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format
//! and partially RFC3339.
//!
//! Validity of a given date is not guaranteed, this parser will happily parse
//! `"2015-02-29"` as a valid date,
//! even though 2015 was no leap year.
//!
//! # Example
//!
//! ```rust
//! let datetime = iso8601::datetime("2015-06-26T16:43:23+0200").unwrap();
//! ```

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate nom;
use nom::IResult::*;

#[macro_use]
mod helper;
pub mod parsers;

/// A date, can hold three different formats.
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub enum Date {
    /// consists of year, month and day of month
    YMD {
        year: i32,
        month: u32,
        day: u32,
    },
    /// consists of year, week and day of week
    Week {
        year: i32,
        ww: u32,
        d: u32,
    },
    /// consists of year and day of year
    Ordinal {
        year: i32,
        ddd: u32,
    },
}

/// A time object
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct Time {
    /// a 24th of a day
    pub hour: u32,
    /// 60 discrete parts of an hour
    pub minute: u32,
    /// a minute are 60 of these
    pub second: u32,
    /// everything after a `.`
    pub millisecond: u32,
    /// depends on where you're at
    pub tz_offset_hours: i32,
    pub tz_offset_minutes: i32,
}

/// Compound struct, hold Date and Time
///
/// duh!
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl Time {
    pub fn set_tz(&self, tzo: (i32, i32)) -> Time {
        let mut t = self.clone();
        t.tz_offset_hours = tzo.0;
        t.tz_offset_hours = tzo.1;
        t
    }
}


/// Parses a date string.
///
/// A string can have one of the following formats:
///
/// * `2015-11-02` or `20151102`
/// * `2015-W45-01` or `2015W451`
/// * `2015-306` or `2015306`
///
/// ## Example
///
/// ```rust
/// let date = iso8601::date("2015-11-02").unwrap();
/// ```
pub fn date(string: &str) -> Result<Date, String> {
    if let Done(_, parsed) = parsers::parse_date(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Parser Error: {}", string))
    }
}


/// Parses a time string.
///
/// A string can have one of the following formats:
///
/// * `07:35:[00][.123]` or `0735[00][.123]`
/// * `07:35:[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)0000)]`
///
/// ## Example
///
/// ```rust
/// let time = iso8601::time("21:56:42").unwrap();
/// ```
pub fn time(string: &str) -> Result<Time, String> {
    if let Done(_, parsed) = parsers::parse_time(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Parser Error: {}", string))
    }
}


/// This parses a datetime string.
///
/// A datetime string is a combination of the valid formats for the date and time.
/// separated by a literal `T`.
/// See the respective functions for the correct format.
///
/// ## Example
///
/// ```rust
/// let dt = iso8601::datetime("2015-11-03T21:56").unwrap();
/// ```
pub fn datetime(string: &str) -> Result<DateTime, String> {
    if let Done(_left_overs, parsed) = parsers::parse_datetime(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Parser Error: {}", string))
    }
}
