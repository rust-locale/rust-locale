//! Locale implementation for MacOS X

use std::borrow::ToOwned;
use std::env::var;
use std::fs::{metadata, File};
use std::io::{BufRead, Error, Result, BufReader};
use std::path::{Path, PathBuf};

use super::{LocaleFactory, Numeric, Time};

/// The directory inside which locale files are found.
///
/// For example, the set of Korean files will be in
/// `/usr/share/locale/ko`.
static LOCALE_DIR: &'static str = "/usr/share/locale";

#[derive(Debug, Clone)]
enum LocaleType {
    Numeric, Time,
}

fn find_user_locale_path(file_name: &str) -> Option<PathBuf> {
    let locale_dir = Path::new(LOCALE_DIR);

    if let Ok(specific_path) = var(file_name) {
        let path = locale_dir.join(Path::new(&specific_path)).join(Path::new(file_name));

        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(all_path) = var("LC_ALL") {
        let path = locale_dir.join(Path::new(&all_path)).join(Path::new(file_name));

        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(lang) = var("LANG") {
        let path = locale_dir.join(Path::new(&lang)).join(Path::new(file_name));

        if path.exists() {
            return Some(path);
        }
    }

    None
}

fn find_locale_path(locale_type: LocaleType, locale_name: &str) -> Option<PathBuf> {
    let file_name = match locale_type {
        LocaleType::Numeric => "LC_NUMERIC",
        LocaleType::Time    => "LC_TIME",
    };

    if locale_name == "" {
        return find_user_locale_path(&file_name);
    } else {
        let locale_dir = Path::new(LOCALE_DIR);
        let path = locale_dir.join(locale_name).join(file_name);

        if path.exists() {
            return Some(path);
        }
    }

    None
}

fn load_numeric(locale: &str) -> Result<Numeric> {
    let path = find_locale_path(LocaleType::Numeric, locale);

    if let Some(path) = path {
        let file = BufReader::new(try!(File::open(&path)));
        let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

        Ok(Numeric {
            decimal_sep: lines[0].trim().to_string(),
            thousands_sep: lines[1].trim().to_string(),
        })
    }
    else {
        return Err(Error::last_os_error());
    }
}

fn load_time(locale: &str) -> Result<Time> {
    let path = find_locale_path(LocaleType::Time, locale);

    if let Some(path) = path {
        let file = BufReader::new(try!(File::open(&path)));
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
        return Err(Error::last_os_error());
    }
}

pub struct MacOSLocaleFactory {
    locale: String,
}

impl MacOSLocaleFactory {
    // TODO: Should really check whether the locale exists.
    pub fn new(locale: &str) -> Result<Self> {
        Ok(MacOSLocaleFactory {
            locale: locale.to_owned()
        })
    }
}

impl LocaleFactory for MacOSLocaleFactory {
    fn get_numeric(&mut self) -> Option<Box<Numeric>> {
        if let Ok(numeric) = load_numeric(&self.locale) {
            Some(Box::new(numeric))
        } else {
            None
        }
    }

    fn get_time(&mut self) -> Option<Box<Time>> {
        if let Ok(time) = load_time(&self.locale) {
            Some(Box::new(time))
        } else {
            None
        }
    }
}

// ---- PathExt replacement ----

pub trait PathExt {
    fn exists(&self) -> bool;
}

impl PathExt for Path {
    fn exists(&self) -> bool { metadata(self).is_ok() }
}
