//! Locale implementation using GNU libc

use ::std::borrow::Cow;
use ::std::ffi::{CStr,CString};
use ::std::io::{Error,Result};
use ::std::sync::Arc;
use super::{LocaleFactory,Numeric,Time};

pub mod ffi;
pub mod langinfo;

/// Wrapper for libc's locale_t.
#[derive(Debug)]
pub struct CLocale {
    c_locale: ffi::locale_t,
}

impl CLocale {
    /// Constructs new complete locale.
    ///
    /// Constructs `CLocale` with all categories from locale `locale`. See
    /// [`newlocale`](http://man7.org/linux/man-pages/man3/newlocale.3.html).
    pub fn new(locale: &str) -> Result<Self> {
        let cloc = try!(CString::new(locale));
        let res = unsafe { ffi::newlocale(ffi::LC_ALL_MASK, cloc.as_ptr(), ::std::ptr::null_mut()) };
        if res.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(CLocale { c_locale: res, })
        }
    }

    /// Constructs new complete locale.
    ///
    /// Constructs `CLocale` with specified categories from locale `locale` and the rest
    /// from `from`. `from` is destroyed in the process. See
    /// [`newlocale`(3)](http://man7.org/linux/man-pages/man3/newlocale.3.html).
    pub fn new_from(mask: ::libc::c_int, locale: &str, mut from: Self) -> Result<CLocale> {
        let cloc = try!(CString::new(locale));
        let res = unsafe { ffi::newlocale(mask, cloc.as_ptr(), from.c_locale) };
        // XXX: Is there better way to skip Drop then zeroing+check? And the associated need to
        // have the field mut though it's otherwise not needed and not desired?
        from.c_locale = ::std::ptr::null_mut();
        if res.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(CLocale { c_locale: res, })
        }
    }

    /// Returns locale ID that is in use for given category.
    /// 
    /// As indicated by `locale_t::names[category]`.
    pub fn name<'a>(&'a self, category: ::libc::c_int) -> Cow<'a, str> {
        assert!(category >= 0 && category <= 12);
        unsafe {
            let ptr = (*self.c_locale).__names[category as usize];
            if ptr.is_null() {
                return Cow::Borrowed("C");
            }
            let cres: &'a CStr = CStr::from_ptr(ptr);
            return String::from_utf8_lossy(cres.to_bytes());
        }
    }
}

impl Drop for CLocale {
    fn drop(&mut self) {
        if !self.c_locale.is_null() {
            unsafe { ffi::freelocale(self.c_locale) };
        }
    }
}

impl Clone for CLocale {
    fn clone(&self) -> Self {
        CLocale {
            c_locale: unsafe { ffi::duplocale(self.c_locale) },
        }
    }
}

#[derive(Debug)]
pub struct IConv {
    iconv: ffi::iconv_t,
}

/// Wrapper for iconv.
///
/// See [`iconv`(3)](http://man7.org/linux/man-pages/man3/iconv.3.html).
/// 
/// On Linux this is part of standard C library and should always be able to convert any charset
/// that the locale component presents, so we can conveniently use it for translating that to the
/// Rust's internal utf-8 encoding there.
impl IConv {
    /// Construct iconv converter.
    ///
    /// See [`iconv_open`(3)](http://man7.org/linux/man-pages/man3/iconv_open.3.html).
    pub fn new(to: &str, from: &str) -> Result<Self> {
        let cto = try!(::std::ffi::CString::new(to));
        let cfrom = try!(::std::ffi::CString::new(from));
        let res = unsafe { ffi::iconv_open(cto.as_ptr(), cfrom.as_ptr()) };
        if res.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(IConv { iconv: res, })
        }
    }

    /// Convert data with iconv
    ///
    /// See [`iconv`(3)](http://man7.org/linux/man-pages/man3/iconv.3.html). The parameters are
    ///
    ///  1. `src`: The input buffer.
    ///  2. `dst`: The output buffer.
    ///
    /// Return values are:
    ///
    ///  1. Result of `iconv`. If -1, the reason can be read from `errno` (unfortunately
    ///     `::std::io::Error::last_os_error()` does not seem to be able to distinguish them at the
    ///     moment).
    ///  2. Number of bytes processed from `src`.
    ///  3. Number of bytes written to `dst`.
    ///
    /// The C interface returns the remaining buffers instead, but that is actually hard to work
    /// with in Rust.
    pub fn convert(&self, src: &[u8], dst: &mut [u8]) -> (isize, usize, usize) {
        let mut inptr: *const ::libc::c_char = src.as_ptr() as *const ::libc::c_char;
        let mut insize: ::libc::size_t = src.len() as ::libc::size_t;
        let mut outptr: *mut ::libc::c_char = dst.as_ptr() as *mut ::libc::c_char;
        let mut outsize: ::libc::size_t = dst.len() as ::libc::size_t;
        // XXX: Do we need error handling? We don't expect errors and can't do much about them here.
        let res = unsafe {
            ffi::iconv(self.iconv,
                &mut inptr, &mut insize,
                &mut outptr, &mut outsize)
        };
        (res as isize, src.len() - (insize as usize), dst.len() - (outsize as usize))
    }
}

impl Drop for IConv {
    fn drop(&mut self) {
        if !self.iconv.is_null() {
            unsafe { ffi::iconv_close(self.iconv); }
        }
    }
}

// FIXME FIXME FIXME #[derive(Clone)]
#[derive(Debug)]
pub struct LibCLocaleFactory {
    locale: Arc<CLocale>,
    iconv: [Option<Arc<IConv>>; 12],
}

impl LibCLocaleFactory {
    fn codeset_index(item: langinfo::CodesetItems) -> usize {
        match item {
            langinfo::_NL_COLLATE_CODESET => 0,
            langinfo::_NL_CTYPE_CODESET_NAME => 1,
            langinfo::_NL_MONETARY_CODESET => 2,
            langinfo::_NL_NUMERIC_CODESET => 3,
            langinfo::_NL_TIME_CODESET => 4,
            langinfo::_NL_MESSAGES_CODESET => 5,
            langinfo::_NL_PAPER_CODESET => 6,
            langinfo::_NL_NAME_CODESET => 7,
            langinfo::_NL_ADDRESS_CODESET => 8,
            langinfo::_NL_TELEPHONE_CODESET => 9,
            langinfo::_NL_MEASUREMENT_CODESET => 10,
            langinfo::_NL_IDENTIFICATION_CODESET => 11,
        }
    }

    // TODO TODO: Could also try overriding all components to their corresponding UTF-8 variants,
    // though that's quite a bit more work.
    pub fn new_from_c_locale(c_locale: CLocale) -> Self {
        fn get_iconv(codeset: langinfo::CodesetItems, locale: &CLocale) -> Option<Arc<IConv>> {
            let cs = unsafe {
                ::std::str::from_utf8_unchecked(
                    ::std::ffi::CStr::from_ptr(
                        ffi::nl_langinfo_l(codeset as ::libc::c_uint, locale.c_locale)).to_bytes())
            };
            if cs != "UTF-8" {
                if let Ok(i) = IConv::new("UTF-8", cs) {
                    return Some(Arc::new(i));
                }
            }
            return None;
        }
        return LibCLocaleFactory{
            iconv: [
                get_iconv(langinfo::_NL_COLLATE_CODESET, &c_locale),
                get_iconv(langinfo::_NL_CTYPE_CODESET_NAME, &c_locale),
                get_iconv(langinfo::_NL_MONETARY_CODESET, &c_locale),
                get_iconv(langinfo::_NL_NUMERIC_CODESET, &c_locale),
                get_iconv(langinfo::_NL_TIME_CODESET, &c_locale),
                get_iconv(langinfo::_NL_MESSAGES_CODESET, &c_locale),
                get_iconv(langinfo::_NL_PAPER_CODESET, &c_locale),
                get_iconv(langinfo::_NL_NAME_CODESET, &c_locale),
                get_iconv(langinfo::_NL_ADDRESS_CODESET, &c_locale),
                get_iconv(langinfo::_NL_TELEPHONE_CODESET, &c_locale),
                get_iconv(langinfo::_NL_MEASUREMENT_CODESET, &c_locale),
                get_iconv(langinfo::_NL_IDENTIFICATION_CODESET, &c_locale),
            ],
            locale: Arc::new(c_locale),
        };
    }

    pub fn new(locale: &str) -> Result<Self> {
        let loc = try!(CLocale::new(locale));

        return Ok(LibCLocaleFactory::new_from_c_locale(loc));
    }

    pub fn langinfo<'a, I>(&'a self, item: I) -> I::Type
        where I: langinfo::LanginfoItem<'a>
    {
        let mut conv = None;
        if let Some(cs) = I::needs_iconv() {
            if let Some(ref iconv) = self.iconv[LibCLocaleFactory::codeset_index(cs)] {
                conv = Some(&**iconv);
            }
        }
        unsafe {
            item.decode(ffi::nl_langinfo_l(item.to_ffi(), self.locale.c_locale), conv)
        }
    }
}

impl LocaleFactory for LibCLocaleFactory {
    fn get_numeric(&mut self) -> Option<Box<Numeric>> {
        return Some(
            Box::new(
                Numeric::new(
                    &self.langinfo(langinfo::RADIXCHAR),
                    &self.langinfo(langinfo::THOUSEP))));
    }

    fn get_time(&mut self) -> Option<Box<Time>> {
        return Some(
            Box::new(
                Time {
                    month_names: vec![
                        self.langinfo(langinfo::ABMON_1).into_owned(),
                        self.langinfo(langinfo::ABMON_2).into_owned(),
                        self.langinfo(langinfo::ABMON_3).into_owned(),
                        self.langinfo(langinfo::ABMON_4).into_owned(),
                        self.langinfo(langinfo::ABMON_5).into_owned(),
                        self.langinfo(langinfo::ABMON_6).into_owned(),
                        self.langinfo(langinfo::ABMON_7).into_owned(),
                        self.langinfo(langinfo::ABMON_8).into_owned(),
                        self.langinfo(langinfo::ABMON_9).into_owned(),
                        self.langinfo(langinfo::ABMON_10).into_owned(),
                        self.langinfo(langinfo::ABMON_11).into_owned(),
                        self.langinfo(langinfo::ABMON_12).into_owned(),
                    ],
                    long_month_names: vec![
                        self.langinfo(langinfo::MON_1).into_owned(),
                        self.langinfo(langinfo::MON_2).into_owned(),
                        self.langinfo(langinfo::MON_3).into_owned(),
                        self.langinfo(langinfo::MON_4).into_owned(),
                        self.langinfo(langinfo::MON_5).into_owned(),
                        self.langinfo(langinfo::MON_6).into_owned(),
                        self.langinfo(langinfo::MON_7).into_owned(),
                        self.langinfo(langinfo::MON_8).into_owned(),
                        self.langinfo(langinfo::MON_9).into_owned(),
                        self.langinfo(langinfo::MON_10).into_owned(),
                        self.langinfo(langinfo::MON_11).into_owned(),
                        self.langinfo(langinfo::MON_12).into_owned(),
                    ],
                    day_names: vec![
                        self.langinfo(langinfo::ABDAY_1).into_owned(),
                        self.langinfo(langinfo::ABDAY_2).into_owned(),
                        self.langinfo(langinfo::ABDAY_3).into_owned(),
                        self.langinfo(langinfo::ABDAY_4).into_owned(),
                        self.langinfo(langinfo::ABDAY_5).into_owned(),
                        self.langinfo(langinfo::ABDAY_6).into_owned(),
                        self.langinfo(langinfo::ABDAY_7).into_owned(),
                    ],
                    long_day_names: vec![
                        self.langinfo(langinfo::DAY_1).into_owned(),
                        self.langinfo(langinfo::DAY_2).into_owned(),
                        self.langinfo(langinfo::DAY_3).into_owned(),
                        self.langinfo(langinfo::DAY_4).into_owned(),
                        self.langinfo(langinfo::DAY_5).into_owned(),
                        self.langinfo(langinfo::DAY_6).into_owned(),
                        self.langinfo(langinfo::DAY_7).into_owned(),
                    ],
                }));
    }
}

#[cfg(test)]
mod test {
    use ::std::ffi::CStr;
    use super::*;

    fn has_locale(locale: &str) -> bool {
        CLocale::new(locale).is_ok()
    }

    #[test]
    fn c_locale() {
        if has_locale("C.UTF-8") {
            let l = LibCLocaleFactory::new("C.UTF-8").unwrap();
            assert_eq!("UTF-8", l.langinfo(langinfo::CODESET));
        } else {
            println!("Skipped!");
        }
    }

    #[test]
    fn en_locale() {
        if has_locale("en_GB") {
            let l = LibCLocaleFactory::new("en_GB").unwrap();
            assert_eq!("ISO-8859-1", l.langinfo(langinfo::CODESET));
        } else {
            println!("Skipped!");
        }
    }

    #[test]
    fn bad_locale() {
        let l = LibCLocaleFactory::new("wrong");
        assert!(l.is_err());
    }

    #[test]
    fn mixed_locale() {
        fn langinfo(loc: &CLocale, item: ::libc::c_uint) -> &str {
            let res = unsafe { CStr::from_ptr(ffi::nl_langinfo_l(item, loc.c_locale)) };
            ::std::str::from_utf8(res.to_bytes()).unwrap()
        }

        if let Ok(l) = CLocale::new("cs_CZ") {
            // only test if the host has these locales (travis boxen don't)
            assert_eq!(",", langinfo(&l, ffi::RADIXCHAR));
            assert_eq!("Po", langinfo(&l, ffi::ABDAY_2));
            if let Ok(m) = CLocale::new_from(ffi::LC_NUMERIC_MASK, "en_GB", l) {
                assert_eq!(".", langinfo(&m, ffi::RADIXCHAR));
                assert_eq!("Po", langinfo(&m, ffi::ABDAY_2));
                if let Ok(n) = CLocale::new_from(ffi::LC_TIME_MASK, "de_DE", m.clone()) {
                    assert_eq!(".", langinfo(&n, ffi::RADIXCHAR));
                    assert_eq!("Mi", langinfo(&n, ffi::ABDAY_4));
                    assert_eq!(".", langinfo(&m, ffi::RADIXCHAR));
                    assert_eq!("Po", langinfo(&m, ffi::ABDAY_2));
                    assert_eq!("cs_CZ", n.name(ffi::LC_CTYPE));
                    assert_eq!("en_GB", n.name(ffi::LC_NUMERIC));
                    assert_eq!("de_DE", n.name(ffi::LC_TIME));
                }
            }
        }
    }

    #[test]
    fn locale_with_convert() {
        if let Ok(lf) = LibCLocaleFactory::new("cs_CZ") {
            // only test if the host has cs_CZ (non-unicode) locale (travis boxen don't)
            assert_eq!("ISO-8859-2", lf.langinfo(langinfo::CODESET));
            assert_eq!("Út", lf.langinfo(langinfo::ABDAY_3));
        }
    }
}
