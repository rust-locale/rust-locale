#![crate_name = "locale"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(core, env, path, io, std_misc)]

use std::old_io::fs::PathExtensions;
use std::old_io::{IoResult, File, BufferedReader};
use std::env::var_string;
use std::num::{Int, Float};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Numeric {
    decimal_sep: String,
    thousands_sep: String,
}

fn find_numeric_locale_path() -> Option<Path> {
    if let Ok(numeric_path) = var_string("LC_ALL") {
        let path = Path::new(numeric_path);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(numeric_path) = var_string("LC_NUMERIC") {
        let path = Path::new(numeric_path);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(lang) = var_string("LANG") {
        let path = Path::new("/usr/share/locale").join(Path::new(lang)).join(Path::new("LC_NUMERIC"));
        if path.exists() {
            return Some(path);
        }
    }

    None
}

impl Numeric {
    pub fn load_user_locale() -> IoResult<Numeric> {
        let path = find_numeric_locale_path();

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

    pub fn format_float<F: Float>(&self, input: F, decimal_places: usize, use_thousand_separators: bool) -> String {
        use std::num::strconv;
        strconv::float_to_str_common(input, 10, false, strconv::SignFormat::SignNone, strconv::SignificantDigits::DigExact(decimal_places), strconv::ExponentFormat::ExpNone, false).0.replace(".", self.decimal_sep.as_slice())
    }
}

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
