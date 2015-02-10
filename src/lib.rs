#![crate_name = "locale"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(core, env, path, io, std_misc)]

use std::old_io::fs::PathExtensions;
use std::old_io::{IoResult, File, BufferedReader};
use std::env::var_string;
use std::num::{Int, Float};
use std::fmt::Display;

enum LocaleType {
    Numeric, Time,
}

impl LocaleType {
    fn file_name(&self) -> &'static str {
        match *self {
            LocaleType::Numeric => "LC_NUMERIC",
            LocaleType::Time    => "LC_TIME",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Numeric {
    decimal_sep: String,
    thousands_sep: String,
}

fn find_locale_path(locale_type: LocaleType) -> Option<Path> {
    if let Ok(numeric_path) = var_string("LC_ALL") {
        let path = Path::new(numeric_path);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(numeric_path) = var_string(locale_type.file_name()) {
        let path = Path::new(numeric_path);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(lang) = var_string("LANG") {
        let path = Path::new("/usr/share/locale").join(Path::new(lang)).join(Path::new(locale_type.file_name()));
        if path.exists() {
            return Some(path);
        }
    }

    None
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
            return Ok(Numeric::default());
        }
    }

    pub fn default() -> Numeric {
        Numeric::new(".", " ")
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
                buf.push_str(&self.thousands_sep[]);
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
            return Ok(Time::default());
        }
    }

    pub fn default() -> Time {
        Time {
            month_names: vec![],
            long_month_names: vec![],
            day_names: vec![],
            long_day_names: vec![],
        }
    }

    pub fn long_month_name(&self, months_from_january: usize) -> String {
        self.month_names[months_from_january].clone()
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
