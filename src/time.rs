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

use std::any::Any;
use std::fmt;
use std::time::SystemTime;

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
    /// Format date and time.
    ///
    /// Output will be written to a formatter.
    ///
    /// # Parameters
    ///
    ///  - `time`: The date/time to format.
    ///  - `zone`: The timezone to be displayed. Note that it can be overridden by the format.
    ///  - `fmt`: Format specification.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    // FIXME - need to define suitable generic fail type
    ///
    /// # Panics
    ///
    /// Panics if the `time` exceeds 86,401,000,000,000 ns, the maximum number of seconds in a day
    /// (note this includes provision for a leap second).
    fn format_datetime_to(&self, time: SystemTime, zone: TimeZone,
                          fmt: &str, out: &mut fmt::Formatter) -> fmt::Result;
}

/// Time zone description
pub enum TimeZone {
    /// Universal Time Coordinated.
    /// 
    /// Also called Greenwhich Mean Time in UK, though that is supposed to mean slightly different thing.
    Utc,
    /// User default time zone taken from current locale or system settings.
    /// 
    /// Usually called Local time.
    User,
    /// Special timezone for events that move with timezone.
    /// 
    /// The Systemtime is interpreted as relative to local start of epoch, i.e. as if offset is 0.
    /// The timezone is printed as “Local Time” if requested.
    Unspecified,
    /// Timezone with given offset from Utc, in seconds East of Greenwhich.
    Offset(i32),
    /// Timezone with given name in IANA database (e.g. `"Europe/Brussels"`).
    Named(&'static str),
}
