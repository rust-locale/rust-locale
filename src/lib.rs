#![crate_name = "locale"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![allow(deprecated)] // FIXME: otherwise warns about self-deprecation

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

#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate locale_config;

use std::fmt::Display;
use std::io::Result;

// TODO: Wrap instead of plain re-export so we can maintain better compatibility.
pub use locale_config::{LanguageRange,Locale};

pub mod facet;

// ---------------------- DEPRECATED CODE BELOW -----------------------------
// The below code is to be deleted in next release.

#[deprecated(since="0.3.0", note="Facets are now independent; see facet module")]
pub trait LocaleFactory {
    /// Get implementation of the Numeric locale category.
    fn get_numeric(&mut self) -> Option<Box<Numeric>> { None }

    /// Get implementation of the Time locale category.
    fn get_time(&mut self) -> Option<Box<Time>> { None }
}

#[deprecated(since="0.3.0", note="Facets are now independent; see facet module")]
#[derive(Debug, Clone)]
pub struct CompositeLocaleFactory<First: LocaleFactory, Second: LocaleFactory> {
    first: First,
    second: Second,
}

impl<F: LocaleFactory, S: LocaleFactory> CompositeLocaleFactory<F, S> {
    pub fn new(first: F, second: S) -> Self {
        CompositeLocaleFactory::<F, S> {
            first: first, second: second
        }
    }
}

#[deprecated(since="0.3.0", note="Facets are now independent; see facet module")]
impl<F: LocaleFactory, S: LocaleFactory> LocaleFactory for CompositeLocaleFactory<F, S> {
    // XXX: Make a macro for this
    fn get_numeric(&mut self) -> Option<Box<Numeric>> {
        if let Some(v) = self.first.get_numeric() {
            Some(v)
        } else {
            self.second.get_numeric()
        }
    }

    fn get_time(&mut self) -> Option<Box<Time>> {
        if let Some(v) = self.first.get_time() {
            Some(v)
        } else {
            self.second.get_time()
        }
    }
}

#[deprecated(since="0.3.0", note="Facets are now independent; see facet module")]
#[derive(Debug, Clone, Default)]
pub struct InvariantLocaleFactory;

impl InvariantLocaleFactory {
    /// Constructs invariant locale factory.
    ///
    /// The signature is just so that it matches the other locale factories so the classes can be
    /// substituted depending on target operating system and the code using them does not have to
    /// care.
    #[allow(unused_variables)]
    pub fn new(locale: &str) -> Result<Self> {
        Ok(InvariantLocaleFactory)
    }
}

#[deprecated(since="0.3.0", note="Facets are now independent; see facet module")]
impl LocaleFactory for InvariantLocaleFactory {
    // NOTE: Yep, it's empty. This just returns nothing and the Locale constructor will take care
    // of the actual defaults.
}

// Deprecated!
#[cfg(target_os = "linux")]
mod linux;

#[deprecated(since="0.3.0", note="Locale factories are deprecated; use facets directly")]
#[cfg(target_os = "linux")]
pub use linux::LibCLocaleFactory as SystemLocaleFactory;

// Deprecated!
// FIXME: #[cfg(target_os = "macos")], but for the moment I need to test whether it compiles, don't
// have MacOS box nor cross-compiler and it does not actually contain anything system-specific yet
mod macos;

#[deprecated(since="0.3.0", note="Locale factories are deprecated; use facets directly")]
#[cfg(target_os = "macos")]
pub use macos::MacOSLocaleFactory as SystemLocaleFactory;

#[deprecated(since="0.3.0", note="Locale factories are deprecated; use facets directly")]
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub use InvariantLocaleFactory as SystemLocaleFactory;

#[deprecated(since="0.3.0", note="Locale factories are deprecated; use facets directly")]
pub fn user_locale_factory() -> SystemLocaleFactory {
    // FIXME: Error handling? Constructing locale with "" should never fail as far as I can tell.
    SystemLocaleFactory::new("").unwrap()
}

// ---- locale facets ----


// ---- numeric stuff ----

#[deprecated(since="0.3.0", note="Replaced with numeric::Numeric facet")]
#[derive(Debug, Clone)]
pub struct Numeric {
    /// The punctuation that separates the decimal part of a non-integer number. Usually a decimal
    /// point or a decimal comma.
    pub decimal_sep: String,

    /// The punctuation that separates groups of digits in long numbers.
    pub thousands_sep: String,
}

impl Numeric {
    pub fn load_user_locale() -> Result<Numeric> {
        if let Ok(mut factory) = SystemLocaleFactory::new("") {
            if let Some(numeric) = factory.get_numeric() {
                return Ok(*numeric);
            }
        }
        Ok(Numeric::english())
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

    pub fn format_int<I: Display>(&self, input: I) -> String {
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

    pub fn format_float<F: Display>(&self, input: F, decimal_places: usize) -> String {
        format!("{:.*}", decimal_places, input).replace(".", &self.decimal_sep)
    }
}

// ---- time stuff ---

#[deprecated(since="0.3.0", note="Replaced with time::Time facet")]
#[derive(Debug, Clone)]
pub struct Time {
    month_names: Vec<String>,
    long_month_names: Vec<String>,
    day_names: Vec<String>,
    long_day_names: Vec<String>,
}

impl Time {
    pub fn load_user_locale() -> Result<Time> {
        if let Ok(mut factory) = SystemLocaleFactory::new("") {
            if let Some(time) = factory.get_time() {
                return Ok(*time);
            }
        }
        Ok(Time::english())
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
}
