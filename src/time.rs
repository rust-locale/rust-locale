//! Time category.
//!
//! Handling of local variations in how dates and times are written and read.
//!
//! # Supported features
//!
//! The default implementation currently only supports formatting dates and times in absolute
//! notation using Gregorian calendar rules.
//!
//! # Format specifications
//!
//! Supported are CLDR style patterns including skeletons.
//!
//! ## Examples
// FIXME FIXME FIXME DOCTESTS!

use chrono;
use nom;
use std::any::Any;
use std::fmt;

pub use chrono::Weekday;

/// Interface for formatting dates and times.
///
/// Currently only Gregorian calendar is supported.
///
/// # Default implementation
///
/// All methods have default implementation suitable for the invariant locale.
///
/// # TODO
///
/// - Parsing.
/// - Other calendars.
pub trait Time : Any + Send + Sync {
    /// Format datetime.
    ///
    /// Output will be written to a formatter.
    ///
    /// # Parameters
    ///
    ///  - `date`: The date to format, as number of days since 1970-01-01.
    ///  - `time`: The time to format, as number of nanoseconds since *local* midnight.
    ///  - `zone`: TODO: The timezone to be displayed.
    ///  - `fmt`: Format specification.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` on invalid pattern or if the underlying formatter returns it during
    /// formatting.
    ///
    /// # Panics
    ///
    /// Panics if the `time` exceeds 86,401,000,000,000 ns, the maximum number of seconds in a day
    /// (note this includes provision for a leap second).
    fn format_datetime_to(&self, date: Option<&i32>, time: Option<&u64>,
                          /* FIXME: zone */ fmt: &str, out: &mut fmt::Formatter) -> fmt::Result {
        // FIXME FIXME FIXME FILLME IN
        Err(fmt::Error)
    }

    /// Format datetime.
    ///
    /// Output will be written to a formatter.
    ///
    /// # Parameters
    ///
    ///  - `time`: The timestamp to format.
    ///  - `zone`: TODO: The timezone.
    ///  - `fmt`: Format specification.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` on invalid pattern or if the underlying formatter returns it during
    /// formatting.
    ///
    /// # Notes
    ///
    /// The `SystemTime` stores UTC, so the value will be adjusted according to zone specified.
    /// Therefore unlike `format_datetime_to`, the timezone matters here.
    ///
    /// `None` timezone is calculated as bias of 0 and printed as ‘local time’.
    fn format_timestamp_to(&self, time: ::std::time::SystemTime, /* FIXME: zone */ fmt: &str, &mut fmt::Formatter) -> fmt::Result {
        // FIXME FIXME FIXME FILLME IN
        Err(fmt::Error)
    }

    /// Get date format.
    ///
    /// This should return format matching the CLDR definition of date/time format elements.
    ///
    /// # Parameters
    ///
    ///  - `fmt`: User-specified format string.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` on invalid format.
    fn get_date_format<'a>(&'a self, fmt: &'a str) -> Result<&'a str, fmt::Error> { Ok(fmt) }

    /// Get time format.
    ///
    /// This should return format matching the CLDR definition of date/time format elements.
    ///
    /// # Parameters
    ///
    ///  - `fmt`: User-specified format string.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` on invalid format.
    fn get_time_format<'a>(&'a self, fmt: &'a str) -> Result<&'a str, fmt::Error> { Ok(fmt) }

    /// Get date+time format.
    ///
    /// This should return format matching the CLDR definition of date/time format elements.
    ///
    /// # Parameters
    ///
    ///  - `fmt`: User-specified format string.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` on invalid format.
    fn get_datetime_format<'a>(&'a self, fmt: &'a str) -> Result<&'a str, fmt::Error> { Ok(fmt) }

    /// Get day on which weeks start.
    ///
    /// # Notes
    ///
    /// Default implementation returns `Weekday::Mon` as that conforms to ISO8601.
    fn get_week_start(&self) -> Weekday { Weekday::Mon }

    /// Print month name.
    ///
    /// # Parameters
    ///
    ///  - `w`: Format variant and width.
    ///  - `c`: Calendar. Default only supports Gregorian.
    ///  - `n`: The month number. January = 1.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` if provided input is not supported or the formatter fails.
    ///
    /// # Notes
    ///
    /// Default implementation just prints the English name for Gregorian calendar and errors
    /// otherwise.
    fn fmt_month(&self, w: FormatWidth, c: Calendar, n: u32, out: &mut fmt::Formatter) -> fmt::Result {
        const MONTHS: [&'static str; 12] = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December",
        ];
        if c != Calendar::Gregorian { return Err(fmt::Error); }
        match w {
            FormatWidth::FormatWide|FormatWidth::StandAloneWide =>
                out.write_str(MONTHS[n as usize - 1]),
            FormatWidth::FormatAbbr|FormatWidth::StandAloneAbbr|
            FormatWidth::FormatShort|FormatWidth::StandAloneShort =>
                out.write_str(&MONTHS[n as usize - 1][0..3]),
            FormatWidth::FormatNarrow|FormatWidth::StandAloneNarrow =>
                out.write_str(&MONTHS[n as usize - 1][0..1]),
        }
    }

    /// Print day of week name.
    ///
    /// # Parameters
    ///
    ///  - `w`: Format variant and width.
    ///  - `c`: Calendar. Default only supports Gregorian.
    ///  - `n`: The day number. Sunday = 0.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` if provided input is not supported or the formatter fails.
    ///
    /// # Notes
    ///
    /// Default implementation just prints the English name for Gregorian calendar and errors
    /// otherwise.
    fn fmt_day(&self, w: FormatWidth, c: Calendar, d: Weekday, out: &mut fmt::Formatter) -> fmt::Result {
        const DAYS: [&'static str; 7] = [
            "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
        ];
        if c != Calendar::Gregorian { return Err(fmt::Error); }
        let n = d.num_days_from_sunday();
        match w {
            FormatWidth::FormatWide|FormatWidth::StandAloneWide =>
                out.write_str(DAYS[n as usize]),
            FormatWidth::FormatAbbr|FormatWidth::StandAloneAbbr|
            FormatWidth::FormatShort|FormatWidth::StandAloneShort =>
                out.write_str(&DAYS[n as usize][0..3]),
            FormatWidth::FormatNarrow|FormatWidth::StandAloneNarrow =>
                out.write_str(&DAYS[n as usize][0..1]),
        }
    }

    /// Print quarter name.
    ///
    /// # Parameters
    ///
    ///  - `w`: Format variant and width.
    ///  - `c`: Calendar. Default only supports Gregorian.
    ///  - `n`: The quarter number. 1 to 4.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` if provided input is not supported or the formatter fails.
    ///
    /// # Notes
    ///
    /// Default implementation just prints Q with the number.
    fn fmt_quarter(&self, w: FormatWidth, c: Calendar, n: u32, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_fmt(format_args!("Q{}", n))
    }

    /// Print day period name.
    ///
    /// # Parameters
    ///
    ///  - `w`: Format variant and width.
    ///  - `c`: Calendar. Default only supports Gregorian.
    ///  - `p`: The day period.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` if provided input is not supported or the formatter fails.
    ///
    /// # Notes
    ///
    /// The default implementation simply prints AM or PM.
    fn fmt_period(&self, w: FormatWidth, c: Calendar, p: DayPeriod, out: &mut fmt::Formatter) -> fmt::Result {
        match p {
            DayPeriod::AM|DayPeriod::Midnight =>
                out.write_str("AM"),
            DayPeriod::PM|DayPeriod::Noon =>
                out.write_str("PM"),
        }
    }

    /// Print era name.
    ///
    /// # Parameters
    ///
    ///  - `w`: Format variant and width.
    ///  - `c`: Calendar. Default only supports Gregorian.
    ///  - `n`: The era. 0 = BCE, 1 = CE.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` if provided input is not supported or the formatter fails.
    ///
    /// # Notes
    ///
    /// The default implementation simply prints BCE or CE for Gregorian calendar and errors
    /// otherwise.
    fn fmt_era(&self, w: FormatWidth, c: Calendar, n: u8, out: &mut fmt::Formatter) -> fmt::Result {
        if c != Calendar::Gregorian { return Err(fmt::Error); }
        match n {
            0 => out.write_str("BCE"),
            1 => out.write_str("CE"),
            _ => Err(fmt::Error),
        }
    }

    /// Print a number.
    ///
    /// # Parameters
    ///
    ///  - `n`: The number to format.
    ///  - `digits`: The number of digits.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` if provided input is not supported or the formatter fails.
    ///
    /// # Notes
    ///
    /// The default implementation formats with non-locale-aware standard formatter, so the actual
    /// implementations need to redirect this to appropriate Numeric instead.
    // TODO: Width as number in Numeric to avoid stringification and re-parse of the width.
    fn fmt_number(&self, n: &fmt::Display, digits: usize, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_fmt(format_args!("{1:0$}", digits, n))
    }
}

/// Calendar type.
///
/// For the moment, only Gregorian proleptic calendar is supported. More calendar types will be
/// added in future, so don't rely on the enum being complete.
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash,PartialOrd,Ord)]
pub enum Calendar {
    /// Gregorian proleptic calendar.
    ///
    /// Interprets dates as if Gregorian calendar has been defined from the dawn of time.
    Gregorian,
    /// Extension placeholder; do not match!
    __MoreCalendars,
}

/// Format style and width.
///
/// CLDR defines several variants of names of days, months, quarters and eras:
///
///  - Format variants are in case appropriate for complete date.
///  - Stand-alone variants are in nominative as appropriate e.g. for labeling calendar columns.
///
/// These are times three or four widths:
///
///  - wide, or full, is the full name,
///  - abbr is the usual abbreviation,
///  - short is shorter abbreviation if it makes sense and
///  - narrow is single-letter (non-unique) indicators only for days of week for use as e.g. column
///    labels in compact table.
///
/// This enum is used to request corresponding format from `Time::fmt_month`, `Time::fmt_day`,
/// `Time::fmt_quarter`, `Time::fmt_period` and `Time::fmt_era`.
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash,PartialOrd,Ord)]
pub enum FormatWidth {
    FormatAbbr,
    FormatWide,
    FormatNarrow,
    FormatShort,
    StandAloneAbbr,
    StandAloneWide,
    StandAloneNarrow,
    StandAloneShort,
}

#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash,PartialOrd,Ord)]
pub enum DayPeriod {
    AM,
    PM,
    Midnight,
    Noon,
}

// ------ pattern grammars (private) -------------------------------------------------------------

fn do_format_date<'a, F: Time>(input: &'a str, facet: &F, date: &chrono::NaiveDate, out: &mut fmt::Formatter) ->
    nom::IResult<&'a str, fmt::Result>
{
    use chrono::Datelike;
    alt_complete!(
        input,
        map!(
            is_a_s!("G"),
            |s: &str| {
                let w = match s.len() {
                    1...3 => FormatWidth::FormatAbbr,
                    4 => FormatWidth::FormatWide,
                    5 => FormatWidth::FormatNarrow,
                    _ => return Err(fmt::Error),
                };
                facet.fmt_era(w, Calendar::Gregorian,
                              if date.year_ce().0 { 1 } else { 0 }, out)
            })
        | map!(
            is_a_s!("y"),
            |s: &str| {
                let y = date.year_ce().1;
                let yy = if s.len() == 2 { y % 100 } else { y };
                facet.fmt_number(&yy, s.len(), out)
            })
        | map!(
            is_a_s!("Y"),
            |s: &str| {
                let y = date.isoweekdate().0;
                let yy = if s.len() == 2 { y % 100 } else { y };
                facet.fmt_number(&yy, s.len(), out)
            })
        | map!(
            is_a_s!("u"),
            |s: &str| {
                facet.fmt_number(&date.year(), s.len(), out)
            })
        // TODO: 'U' - Cyclic year name.
        // TODO: 'r' - Related Gregorian year (numeric).
        | map!(
            is_a_s!("qQ"),
            |s: &str| {
                let q = date.month0() / 3 + 1;
                if s.len() <= 2 {
                    facet.fmt_number(&q, s.len(), out)
                } else {
                    facet.fmt_quarter(match s {
                        "QQQ" => FormatWidth::FormatAbbr,
                        "QQQQ" => FormatWidth::FormatWide,
                        "QQQQQ" => FormatWidth::FormatNarrow,
                        "qqq" => FormatWidth::StandAloneAbbr,
                        "qqqq" => FormatWidth::StandAloneWide,
                        "qqqqq" => FormatWidth::StandAloneNarrow,
                        _ => return Err(fmt::Error),
                    }, Calendar::Gregorian, q, out)
                }
            })
        | map!(
            is_a_s!("ML"),
            |s: &str| {
                if s.len() <= 2 {
                    facet.fmt_number(&date.month(), s.len(), out)
                } else {
                    facet.fmt_month(match s {
                        "MMM" => FormatWidth::FormatAbbr,
                        "MMMM" => FormatWidth::FormatWide,
                        "MMMMM" => FormatWidth::FormatNarrow,
                        "LLL" => FormatWidth::StandAloneAbbr,
                        "LLLL" => FormatWidth::StandAloneWide,
                        "LLLLL" => FormatWidth::StandAloneNarrow,
                        _ => return Err(fmt::Error),
                    }, Calendar::Gregorian, date.month(), out)
                }
            })
        | map!(
            is_a_s!("w"),
            |s: &str| {
                facet.fmt_number(&date.isoweekdate().1, s.len(), out)
            })
        // TODO: 'W' - Week of Month (numeric)
        | map!(
            is_a_s!("d"),
            |s: &str| {
                facet.fmt_number(&date.day(), s.len(), out)
            })
        | map!(
            is_a_s!("D"),
            |s: &str| {
                facet.fmt_number(&date.ordinal(), s.len(), out)
            })
        // TODO: 'F' - Day of Week in Month (numeric).
        // TODO: 'g' - Modified Julian day (numeric).
        | map!(
            is_a_s!("Eec"),
            |s: &str| {
                if s == "e" || s == "ee" || s == "c" || s == "cc" {
                    let n = (date.weekday().num_days_from_sunday()
                             + 7 - facet.get_week_start().num_days_from_sunday()) % 7 + 1;
                    facet.fmt_number(&n, s.len(), out)
                } else {
                    facet.fmt_day(match s {
                        "E"|"EE"|"EEE"|"eee" => FormatWidth::FormatAbbr,
                        "EEEE"|"eeee" => FormatWidth::FormatWide,
                        "EEEEE"|"eeeee" => FormatWidth::FormatNarrow,
                        "EEEEEE"|"eeeeee" => FormatWidth::FormatShort,
                        "ccc" => FormatWidth::StandAloneAbbr,
                        "cccc" => FormatWidth::StandAloneWide,
                        "ccccc" => FormatWidth::StandAloneNarrow,
                        "cccccc" => FormatWidth::StandAloneShort,
                        _ => return Err(fmt::Error),
                    }, Calendar::Gregorian, date.weekday(), out)
                }
            })
    )
}

fn do_format_time<'a, F: Time>(input: &'a str, facet: &F, time: &chrono::NaiveTime, out: &mut fmt::Formatter) ->
    nom::IResult<&'a str, fmt::Result>
{
    use chrono::Timelike;
    alt_complete!(
        input,
        map!(
            is_a_s!("a"),
            |s: &str| {
                facet.fmt_period(match s {
                    "a"|"aa"|"aaa" => FormatWidth::FormatAbbr,
                    "aaaa" => FormatWidth::FormatWide,
                    "aaaaa" => FormatWidth::FormatNarrow,
                    _ => return Err(fmt::Error),
                },
                Calendar::Gregorian,
                if time.hour() < 12 { DayPeriod::AM } else { DayPeriod::PM }, out)
            })
        | map!(
            is_a_s!("b"),
            |s: &str| {
                facet.fmt_period(match s {
                    "b"|"bb"|"bbb" => FormatWidth::FormatAbbr,
                    "bbbb" => FormatWidth::FormatWide,
                    "bbbbb" => FormatWidth::FormatNarrow,
                    _ => return Err(fmt::Error),
                },
                Calendar::Gregorian,
                if time.hour() == 0 && time.minute() == 0 && time.second() == 0 { DayPeriod::Midnight }
                else if time.hour() == 12 && time.minute() == 0 && time.second() == 0 { DayPeriod::Noon }
                else if time.hour() < 12 { DayPeriod::AM }
                else { DayPeriod::PM }, out)
            })
        // TODO: 'B' - flexible day periods
        | map!(
            is_a_s!("h"), // h12
            |s: &str| {
                facet.fmt_number(&time.hour12().1, s.len(), out)
            })
        | map!(
            is_a_s!("H"), // h23
            |s: &str| {
                facet.fmt_number(&time.hour(), s.len(), out)
            })
        | map!(
            is_a_s!("K"), // h11
            |s: &str| {
                facet.fmt_number(&(time.hour() % 12), s.len(), out)
            })
        | map!(
            is_a_s!("k"), // h24
            |s: &str| {
                let h = time.hour();
                facet.fmt_number(&(if h == 0 { 24 } else { h }), s.len(), out)
            })
        | map!(
            is_a_s!("m"),
            |s: &str| {
                facet.fmt_number(&time.minute(), s.len(), out)
            })
        | map!(
            is_a_s!("s"),
            |s: &str| {
                // NOTE: leap second is represented as 59 s + >=1e9 ns.
                let sec = time.second() + if time.nanosecond() >= 1000000000 { 1 } else { 0 };
                facet.fmt_number(&sec, s.len(), out)
            })
        // XXX The specification of S is somewhat insane; I would prefer specifying fractional
        // number of seconds with precision to get the correct separator from corresponding
        // Numeric.
        | map!(
            is_a_s!("S"),
            |s: &str| {
                let ns = time.nanosecond() % 1000000000; // Leap second was added to seconds
                let fs = if s.len() < 9 { ns / 10u32.pow(s.len() as u32) }
                    else if s.len() == 9 { ns }
                    else { ns * 10u32.pow(s.len() as u32) };
                facet.fmt_number(&fs, s.len(), out)
            })
        // TODO: 'A' - ms in a day
    )
}

// TODO: Time zones - 'z', 'Z', 'O', 'v', 'V', 'X', 'x'

fn do_format_other<'a>(input: &'a str, out: &mut fmt::Formatter) ->
    nom::IResult<&'a str, fmt::Result>
{
    alt_complete!(
        input,
        map!(
            is_not_s!("'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrtsuvwxyz"),
            |s: &str| {
                out.write_str(s)
            })
        | map!(
            tag_s!("''"),
            |s: &str| {
                out.write_str(s)
            })
        | map!(
            delimited!(
                tag_s!("'"),
                many1!(
                    alt_complete!(
                        is_not_s!("'")
                        | tag_s!("''"))),
                tag_s!("'")),
            |mut sv: Vec<&str>| {
                sv.drain(..)
                    .map(|s| out.write_str(if s == "''" { "'" } else { s }))
                    .collect::<Result<Vec<()>, _>>()
                    .map(|_| ())
            })
    )
}
