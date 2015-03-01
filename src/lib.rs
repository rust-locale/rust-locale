#![crate_name = "locale"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(core, env, path, io, std_misc)]

//! Localisation is hard.
//!
//! Getting your program to work well in multiple languages is a world fraught with edge-cases,
//! minor grammatical errors, and most importantly, subtle things that don't map over well that you
//! have absolutely no idea are different in other cultures.
//!
//! Many people are aware of the simpler ones, such as whether to use decimal points or decimal
//! commas, or that the names of the months are different in other languages. But there are also
//! different ways to format dates and times, or variations on what day the week begins. It's
//! perfectly possible to write your program unaware of how these things have to be changed at all,
//! and that's why it's so hard.

use std::old_io::fs::PathExtensions;
use std::old_io::{IoResult, File, BufferedReader};
use std::env::var;
use std::num::{Int, Float};
use std::fmt::Display;

/// The directory inside which locale files are found.
///
/// For example, the set of Korean files will be in
/// `/usr/share/locale/ko`.
static LOCALE_DIR: &'static str = "/usr/share/locale";

#[derive(Debug, Clone)]
enum LocaleType {
    Numeric, Time,
}

fn find_locale_path(locale_type: LocaleType) -> Option<Path> {
    if let Ok(numeric_path) = var("LC_ALL") {
        let path = Path::new(numeric_path);
        if path.exists() {
            return Some(path);
        }
    }

    let file_name = match locale_type {
        LocaleType::Numeric => "LC_NUMERIC",
        LocaleType::Time    => "LC_TIME",
    };

    if let Ok(numeric_path) = var(file_name) {
        let path = Path::new(numeric_path);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(lang) = var("LANG") {
        let path = Path::new(LOCALE_DIR).join(Path::new(lang)).join(Path::new(file_name));

        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Information on how to format numbers.
#[derive(Debug, Clone)]
pub struct Numeric {
    /// The punctuation that separates the decimal part of a non-integer number. Usually a decimal
    /// point or a decimal comma.
    pub decimal_sep: String,

    /// The punctuation that separates groups of digits in long numbers.
    pub thousands_sep: String,
}

impl Numeric {
    pub fn load_user_locale() -> IoResult<Numeric> {
        let path = find_locale_path(LocaleType::Numeric);

        if let Some(path) = path {
            let mut file = BufferedReader::new(File::open(&path));
            let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

            Ok(Numeric {
                decimal_sep: lines[0].trim().to_string(),
                thousands_sep: lines[1].trim().to_string(),
            })
        }
        else {
            return Ok(Numeric::english());
        }
    }

    pub fn english() -> Numeric {
        Numeric::new(".", ",")
    }

    pub fn new(decimal_sep: &str, thousands_sep: &str) -> Numeric {
        Numeric {
            decimal_sep: decimal_sep.to_string(),
            thousands_sep: thousands_sep.to_string(),
        }
    }

    pub fn format_int<I: Int + Display>(&self, input: I) -> String {
        let s = input.to_string();
        let mut buf = String::new();

        for (i, c) in s.chars().enumerate() {
            buf.push(c);
            if (s.len() - i - 1) % 3 == 0 && i != s.len() - 1 {
                buf.push_str(&self.thousands_sep[..]);
            }
        }

        buf
    }

    pub fn format_float<F: Float>(&self, input: F, decimal_places: usize) -> String {
        use std::num::strconv;
        strconv::float_to_str_common(input, 10, false, strconv::SignFormat::SignNone, strconv::SignificantDigits::DigExact(decimal_places), strconv::ExponentFormat::ExpNone, false).0.replace(".", self.decimal_sep.as_slice())
    }
}

// ---- time stuff ---

#[derive(Debug, Clone)]
pub struct Time {
    month_names: Vec<String>,
    long_month_names: Vec<String>,
    day_names: Vec<String>,
    long_day_names: Vec<String>,
}

impl Time {
    pub fn load_user_locale() -> IoResult<Time> {
        let path = find_locale_path(LocaleType::Time);

        if let Some(path) = path {
            let mut file = BufferedReader::new(File::open(&path));
            let mut iter = file.lines().map(|x| x.unwrap().trim().to_string());

            let month_names      = iter.by_ref().take(12).collect();
            let long_month_names = iter.by_ref().take(12).collect();
            let day_names        = iter.by_ref().take(7).collect();
            let long_day_names   = iter.by_ref().take(7).collect();

            Ok(Time {
                month_names:      month_names,
                long_month_names: long_month_names,
                day_names:        day_names,
                long_day_names:   long_day_names,
            })
        }
        else {
            return Ok(Time::english());
        }
    }

    pub fn english() -> Time {
        Time {
            month_names: vec![
                "Jan".to_string(),  "Feb".to_string(),  "Mar".to_string(),
                "Apr".to_string(),  "May".to_string(),  "Jun".to_string(),
                "Jul".to_string(),  "Aug".to_string(),  "Sep".to_string(),
                "Oct".to_string(),  "Nov".to_string(),  "Dec".to_string(),
            ],
            long_month_names: vec![
                "January".to_string(),    "February".to_string(),
                "March".to_string(),      "April".to_string(),
                "May".to_string(),        "June".to_string(),
                "July".to_string(),       "August".to_string(),
                "September".to_string(),  "October".to_string(),
                "November".to_string(),   "December".to_string(),
            ],
            day_names: vec![
                "Sun".to_string(),
                "Mon".to_string(),  "Tue".to_string(),  "Wed".to_string(),
                "Thu".to_string(),  "Fri".to_string(),  "Sat".to_string(),
            ],
            long_day_names: vec![
                "Sunday".to_string(),
                "Monday".to_string(),    "Tuesday".to_string(),  "Wednesday".to_string(),
                "Thursday".to_string(),  "Friday".to_string(),   "Saturday".to_string(),
            ],
        }
    }

    pub fn long_month_name(&self, months_from_january: usize) -> String {
        self.long_month_names[months_from_january].clone()
    }

    pub fn short_month_name(&self, months_from_january: usize) -> String {
        self.month_names[months_from_january].clone()
    }

    pub fn long_day_name(&self, days_from_sunday: usize) -> String {
        self.day_names[days_from_sunday].clone()
    }

    pub fn short_day_name(&self, days_from_sunday: usize) -> String {
        self.day_names[days_from_sunday].clone()
    }

}

// ---- tests ----

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn thousands_separator() {
        let numeric_options = Numeric::new("/", "=");
        assert_eq!("1=234=567".to_string(), numeric_options.format_int(1234567))
    }

    #[test]
    fn thousands_separator_2() {
        let numeric_options = Numeric::new("/", "=");
        assert_eq!("123=456".to_string(), numeric_options.format_int(123456))
    }

    #[test]
    fn thousands_separator_3() {
        let numeric_options = Numeric::new("/", "=");
        assert_eq!("12=345=678".to_string(), numeric_options.format_int(12345678))
    }
}
