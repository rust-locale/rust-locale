//! Items for use with nl_langinfo_l and associated properties.
#![allow(non_camel_case_types)]

use ::std::borrow::Cow;
use ::std::ffi::CStr;
use ::std::fmt::Debug;
use ::std::mem::transmute_copy;
use super::ffi;
use super::IConv;

pub trait LanginfoItem<'a> : Copy + Sized {
    type Type : Debug;
    fn needs_iconv() -> Option<CodesetItems>;
    unsafe fn decode(&self, *const ::libc::c_char, Option<&IConv>) -> Self::Type;
    fn to_ffi(self) -> ffi::nl_item;
}

unsafe fn decode_string<'a>(ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
    if ptr.is_null() {
        return Cow::Borrowed("");
    }
    let cres: &'a CStr = CStr::from_ptr(ptr);
    if let Some(iconv) = iconv {
        let capacity = 4 * cres.to_bytes().len();
        let mut buf = Vec::with_capacity(capacity);
        buf.set_len(capacity);
        let conv = iconv.convert(cres.to_bytes(), &mut buf);
        buf.set_len(conv.2);
        if let Ok(s) = String::from_utf8(buf) {
            return Cow::Owned(s);
        }
    }
    return String::from_utf8_lossy(cres.to_bytes());
}

unsafe fn decode_strings<'a>(mut ptr: *const ::libc::c_char, iconv: Option<&IConv>, max: usize) -> Vec<Cow<'a, str>> {
    let mut res = Vec::with_capacity(max);
    while max > 0 && !ptr.is_null() && *ptr != 0 {
        let len = CStr::from_ptr(ptr).to_bytes_with_nul().len();
        let s = decode_string(ptr, iconv);
        ptr = ptr.offset(len as isize);
        res.push(s);
    }
    return res;
}

unsafe fn decode_bytes<'a>(ptr: *const ::libc::c_char) -> &'a [i8] {
    if ptr.is_null() {
        &[]
    } else {
        ::std::slice::from_raw_parts(ptr as *const i8, ::libc::strlen(ptr) as usize)
    }
}

/// `nl_langinfo` items that return charset names
#[derive(Copy, Clone, Debug)]
pub enum CodesetItems {
    _NL_COLLATE_CODESET = ffi::_NL_COLLATE_CODESET as isize,
    _NL_CTYPE_CODESET_NAME = ffi::_NL_CTYPE_CODESET_NAME as isize,
    _NL_MONETARY_CODESET = ffi::_NL_MONETARY_CODESET as isize,
    _NL_NUMERIC_CODESET = ffi::_NL_NUMERIC_CODESET as isize,
    _NL_TIME_CODESET = ffi::_NL_TIME_CODESET as isize,
    _NL_MESSAGES_CODESET = ffi::_NL_MESSAGES_CODESET as isize,
    _NL_PAPER_CODESET = ffi::_NL_PAPER_CODESET as isize,
    _NL_NAME_CODESET = ffi::_NL_NAME_CODESET as isize,
    _NL_ADDRESS_CODESET = ffi::_NL_ADDRESS_CODESET as isize,
    _NL_TELEPHONE_CODESET = ffi::_NL_TELEPHONE_CODESET as isize,
    _NL_MEASUREMENT_CODESET = ffi::_NL_MEASUREMENT_CODESET as isize,
    _NL_IDENTIFICATION_CODESET = ffi::_NL_IDENTIFICATION_CODESET as isize,
}

impl<'a> LanginfoItem<'a> for CodesetItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { None }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, _: Option<&IConv>) -> Cow<'a, str> {
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes())
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::CodesetItems::*;
pub use self::CodesetItems::_NL_CTYPE_CODESET_NAME as CODESET;

/// `nl_langinfo` items in `LC_COLLATE` category that have string values
///
/// The decoding function uses appropriate 
#[derive(Copy, Clone, Debug)]
pub enum CollateStringItems {
    _NL_COLLATE_RULESETS = ffi::_NL_COLLATE_RULESETS as isize,
}

impl<'a> LanginfoItem<'a> for CollateStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_COLLATE_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::CollateStringItems::*;

/// `nl_langinfo` items in `LC_CTYPE` category that have string values
#[derive(Copy, Clone, Debug)]
pub enum CTypeStringItems {
    _NL_CTYPE_INDIGITS0_MB = ffi::_NL_CTYPE_INDIGITS0_MB as isize,
    _NL_CTYPE_INDIGITS1_MB = ffi::_NL_CTYPE_INDIGITS1_MB as isize,
    _NL_CTYPE_INDIGITS2_MB = ffi::_NL_CTYPE_INDIGITS2_MB as isize,
    _NL_CTYPE_INDIGITS3_MB = ffi::_NL_CTYPE_INDIGITS3_MB as isize,
    _NL_CTYPE_INDIGITS4_MB = ffi::_NL_CTYPE_INDIGITS4_MB as isize,
    _NL_CTYPE_INDIGITS5_MB = ffi::_NL_CTYPE_INDIGITS5_MB as isize,
    _NL_CTYPE_INDIGITS6_MB = ffi::_NL_CTYPE_INDIGITS6_MB as isize,
    _NL_CTYPE_INDIGITS7_MB = ffi::_NL_CTYPE_INDIGITS7_MB as isize,
    _NL_CTYPE_INDIGITS8_MB = ffi::_NL_CTYPE_INDIGITS8_MB as isize,
    _NL_CTYPE_INDIGITS9_MB = ffi::_NL_CTYPE_INDIGITS9_MB as isize,
    _NL_CTYPE_OUTDIGIT0_MB = ffi::_NL_CTYPE_OUTDIGIT0_MB as isize,
    _NL_CTYPE_OUTDIGIT1_MB = ffi::_NL_CTYPE_OUTDIGIT1_MB as isize,
    _NL_CTYPE_OUTDIGIT2_MB = ffi::_NL_CTYPE_OUTDIGIT2_MB as isize,
    _NL_CTYPE_OUTDIGIT3_MB = ffi::_NL_CTYPE_OUTDIGIT3_MB as isize,
    _NL_CTYPE_OUTDIGIT4_MB = ffi::_NL_CTYPE_OUTDIGIT4_MB as isize,
    _NL_CTYPE_OUTDIGIT5_MB = ffi::_NL_CTYPE_OUTDIGIT5_MB as isize,
    _NL_CTYPE_OUTDIGIT6_MB = ffi::_NL_CTYPE_OUTDIGIT6_MB as isize,
    _NL_CTYPE_OUTDIGIT7_MB = ffi::_NL_CTYPE_OUTDIGIT7_MB as isize,
    _NL_CTYPE_OUTDIGIT8_MB = ffi::_NL_CTYPE_OUTDIGIT8_MB as isize,
    _NL_CTYPE_OUTDIGIT9_MB = ffi::_NL_CTYPE_OUTDIGIT9_MB as isize,
    _NL_CTYPE_TRANSLIT_IGNORE = ffi::_NL_CTYPE_TRANSLIT_IGNORE as isize,
}

impl<'a> LanginfoItem<'a> for CTypeStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_CTYPE_CODESET_NAME) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::CTypeStringItems::*;

#[derive(Copy, Clone, Debug)]
pub enum CTypeStringListItems {
    _NL_CTYPE_CLASS_NAMES = ffi::_NL_CTYPE_CLASS_NAMES as isize,
    _NL_CTYPE_MAP_NAMES = ffi::_NL_CTYPE_MAP_NAMES as isize,
}

impl<'a> LanginfoItem<'a> for CTypeStringListItems {
    type Type = Vec<Cow<'a, str>>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_CTYPE_CODESET_NAME) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Vec<Cow<'a, str>> {
        decode_strings(ptr, iconv, 32)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::CTypeStringListItems::*;

#[derive(Copy, Clone, Debug)]
pub enum MonetaryStringItems {
    INT_CURR_SYMBOL = ffi::__INT_CURR_SYMBOL as isize,
    CURRENCY_SYMBOL = ffi::__CURRENCY_SYMBOL as isize,
    MON_DECIMAL_POINT = ffi::__MON_DECIMAL_POINT as isize,
    MON_THOUSANDS_SEP = ffi::__MON_THOUSANDS_SEP as isize,
    POSITIVE_SIGN = ffi::__POSITIVE_SIGN as isize,
    NEGATIVE_SIGN = ffi::__NEGATIVE_SIGN as isize,
    _NL_MONETARY_CRNCYSTR = ffi::_NL_MONETARY_CRNCYSTR as isize,
    _NL_MONETARY_DUO_INT_CURR_SYMBOL = ffi::_NL_MONETARY_DUO_INT_CURR_SYMBOL as isize,
    _NL_MONETARY_DUO_CURRENCY_SYMBOL = ffi::_NL_MONETARY_DUO_CURRENCY_SYMBOL as isize,
}

impl<'a> LanginfoItem<'a> for MonetaryStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_MONETARY_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::MonetaryStringItems::*;

#[derive(Copy, Clone, Debug)]
pub enum NumericStringItems {
    __DECIMAL_POINT = ffi::__DECIMAL_POINT as isize,
    __THOUSANDS_SEP = ffi::__THOUSANDS_SEP as isize,
}

impl<'a> LanginfoItem<'a> for NumericStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_NUMERIC_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::NumericStringItems::*;

pub use self::NumericStringItems::__DECIMAL_POINT as RADIXCHAR;
pub use self::NumericStringItems::__THOUSANDS_SEP as THOUSEP;

#[derive(Copy, Clone, Debug)]
pub enum TimeStringItems {
    ABDAY_1 = ffi::ABDAY_1 as isize,
    ABDAY_2 = ffi::ABDAY_2 as isize,
    ABDAY_3 = ffi::ABDAY_3 as isize,
    ABDAY_4 = ffi::ABDAY_4 as isize,
    ABDAY_5 = ffi::ABDAY_5 as isize,
    ABDAY_6 = ffi::ABDAY_6 as isize,
    ABDAY_7 = ffi::ABDAY_7 as isize,
    DAY_1 = ffi::DAY_1 as isize,
    DAY_2 = ffi::DAY_2 as isize,
    DAY_3 = ffi::DAY_3 as isize,
    DAY_4 = ffi::DAY_4 as isize,
    DAY_5 = ffi::DAY_5 as isize,
    DAY_6 = ffi::DAY_6 as isize,
    DAY_7 = ffi::DAY_7 as isize,
    ABMON_1 = ffi::ABMON_1 as isize,
    ABMON_2 = ffi::ABMON_2 as isize,
    ABMON_3 = ffi::ABMON_3 as isize,
    ABMON_4 = ffi::ABMON_4 as isize,
    ABMON_5 = ffi::ABMON_5 as isize,
    ABMON_6 = ffi::ABMON_6 as isize,
    ABMON_7 = ffi::ABMON_7 as isize,
    ABMON_8 = ffi::ABMON_8 as isize,
    ABMON_9 = ffi::ABMON_9 as isize,
    ABMON_10 = ffi::ABMON_10 as isize,
    ABMON_11 = ffi::ABMON_11 as isize,
    ABMON_12 = ffi::ABMON_12 as isize,
    MON_1 = ffi::MON_1 as isize,
    MON_2 = ffi::MON_2 as isize,
    MON_3 = ffi::MON_3 as isize,
    MON_4 = ffi::MON_4 as isize,
    MON_5 = ffi::MON_5 as isize,
    MON_6 = ffi::MON_6 as isize,
    MON_7 = ffi::MON_7 as isize,
    MON_8 = ffi::MON_8 as isize,
    MON_9 = ffi::MON_9 as isize,
    MON_10 = ffi::MON_10 as isize,
    MON_11 = ffi::MON_11 as isize,
    MON_12 = ffi::MON_12 as isize,
    AM_STR = ffi::AM_STR as isize,
    PM_STR = ffi::PM_STR as isize,
    D_T_FMT = ffi::D_T_FMT as isize,
    D_FMT = ffi::D_FMT as isize,
    T_FMT = ffi::T_FMT as isize,
    T_FMT_AMPM = ffi::T_FMT_AMPM as isize,
    ERA_YEAR = ffi::__ERA_YEAR as isize,
    ERA_D_FMT = ffi::ERA_D_FMT as isize,
    ERA_D_T_FMT = ffi::ERA_D_T_FMT as isize,
    ERA_T_FMT = ffi::ERA_T_FMT as isize,
    _NL_TIME_ERA_ENTRIES = ffi::_NL_TIME_ERA_ENTRIES as isize,
    _NL_TIME_TIMEZONE = ffi::_NL_TIME_TIMEZONE as isize,
    _DATE_FMT = ffi::_DATE_FMT as isize,
}

impl<'a> LanginfoItem<'a> for TimeStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_TIME_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::TimeStringItems::*;

#[derive(Copy, Clone, Debug)]
pub enum TimeStringListItems {
    ERA = ffi::ERA as isize,
    ALT_DIGITS = ffi::ALT_DIGITS as isize,
}

impl<'a> LanginfoItem<'a> for TimeStringListItems {
    type Type = Vec<Cow<'a, str>>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_TIME_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Vec<Cow<'a, str>> {
        decode_strings(ptr, iconv, 100)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::TimeStringListItems::*;

#[derive(Copy, Clone, Debug)]
pub enum MessagesStringItems {
    YESEXPR = ffi::__YESEXPR as isize,
    NOEXPR = ffi::__NOEXPR as isize,
    YESSTR = ffi::__YESSTR as isize,
    NOSTR = ffi::__NOSTR as isize,
}

impl<'a> LanginfoItem<'a> for MessagesStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_MESSAGES_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::MessagesStringItems::*;

#[derive(Copy, Clone, Debug)]
pub enum NameStringItems {
    _NL_NAME_NAME_FMT = ffi::_NL_NAME_NAME_FMT as isize,
    _NL_NAME_NAME_GEN = ffi::_NL_NAME_NAME_GEN as isize,
    _NL_NAME_NAME_MR = ffi::_NL_NAME_NAME_MR as isize,
    _NL_NAME_NAME_MRS = ffi::_NL_NAME_NAME_MRS as isize,
    _NL_NAME_NAME_MISS = ffi::_NL_NAME_NAME_MISS as isize,
    _NL_NAME_NAME_MS = ffi::_NL_NAME_NAME_MS as isize,
}

impl<'a> LanginfoItem<'a> for NameStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_NAME_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::NameStringItems::*;

#[derive(Copy, Clone, Debug)]
pub enum AddressStringItems {
    _NL_ADDRESS_POSTAL_FMT = ffi::_NL_ADDRESS_POSTAL_FMT as isize,
    _NL_ADDRESS_COUNTRY_NAME = ffi::_NL_ADDRESS_COUNTRY_NAME as isize,
    _NL_ADDRESS_COUNTRY_POST = ffi::_NL_ADDRESS_COUNTRY_POST as isize,
    _NL_ADDRESS_COUNTRY_AB2 = ffi::_NL_ADDRESS_COUNTRY_AB2 as isize,
    _NL_ADDRESS_COUNTRY_AB3 = ffi::_NL_ADDRESS_COUNTRY_AB3 as isize,
    _NL_ADDRESS_COUNTRY_CAR = ffi::_NL_ADDRESS_COUNTRY_CAR as isize,
    _NL_ADDRESS_COUNTRY_ISBN = ffi::_NL_ADDRESS_COUNTRY_ISBN as isize,
    _NL_ADDRESS_LANG_NAME = ffi::_NL_ADDRESS_LANG_NAME as isize,
    _NL_ADDRESS_LANG_AB = ffi::_NL_ADDRESS_LANG_AB as isize,
    _NL_ADDRESS_LANG_TERM = ffi::_NL_ADDRESS_LANG_TERM as isize,
    _NL_ADDRESS_LANG_LIB = ffi::_NL_ADDRESS_LANG_LIB as isize,
}

impl<'a> LanginfoItem<'a> for AddressStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_ADDRESS_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::AddressStringItems::*;

#[derive(Copy, Clone, Debug)]
pub enum TelephoneStringItems {
    _NL_TELEPHONE_TEL_INT_FMT = ffi::_NL_TELEPHONE_TEL_INT_FMT as isize,
    _NL_TELEPHONE_TEL_DOM_FMT = ffi::_NL_TELEPHONE_TEL_DOM_FMT as isize,
    _NL_TELEPHONE_INT_SELECT = ffi::_NL_TELEPHONE_INT_SELECT as isize,
    _NL_TELEPHONE_INT_PREFIX = ffi::_NL_TELEPHONE_INT_PREFIX as isize,
}

impl<'a> LanginfoItem<'a> for TelephoneStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_TELEPHONE_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::TelephoneStringItems::*;

#[derive(Copy, Clone, Debug)]
pub enum ByteItems {
    // Monetary
    __INT_FRAC_DIGITS = ffi::__INT_FRAC_DIGITS as isize,
    __FRAC_DIGITS = ffi::__FRAC_DIGITS as isize,
    __P_CS_PRECEDES = ffi::__P_CS_PRECEDES as isize,
    __P_SEP_BY_SPACE = ffi::__P_SEP_BY_SPACE as isize,
    __N_CS_PRECEDES = ffi::__N_CS_PRECEDES as isize,
    __N_SEP_BY_SPACE = ffi::__N_SEP_BY_SPACE as isize,
    __P_SIGN_POSN = ffi::__P_SIGN_POSN as isize,
    __N_SIGN_POSN = ffi::__N_SIGN_POSN as isize,
    __INT_P_CS_PRECEDES = ffi::__INT_P_CS_PRECEDES as isize,
    __INT_P_SEP_BY_SPACE = ffi::__INT_P_SEP_BY_SPACE as isize,
    __INT_N_CS_PRECEDES = ffi::__INT_N_CS_PRECEDES as isize,
    __INT_N_SEP_BY_SPACE = ffi::__INT_N_SEP_BY_SPACE as isize,
    __INT_P_SIGN_POSN = ffi::__INT_P_SIGN_POSN as isize,
    __INT_N_SIGN_POSN = ffi::__INT_N_SIGN_POSN as isize,
    _NL_MONETARY_DUO_INT_FRAC_DIGITS = ffi::_NL_MONETARY_DUO_INT_FRAC_DIGITS as isize,
    _NL_MONETARY_DUO_FRAC_DIGITS = ffi::_NL_MONETARY_DUO_FRAC_DIGITS as isize,
    _NL_MONETARY_DUO_P_CS_PRECEDES = ffi::_NL_MONETARY_DUO_P_CS_PRECEDES as isize,
    _NL_MONETARY_DUO_P_SEP_BY_SPACE = ffi::_NL_MONETARY_DUO_P_SEP_BY_SPACE as isize,
    _NL_MONETARY_DUO_N_CS_PRECEDES = ffi::_NL_MONETARY_DUO_N_CS_PRECEDES as isize,
    _NL_MONETARY_DUO_N_SEP_BY_SPACE = ffi::_NL_MONETARY_DUO_N_SEP_BY_SPACE as isize,
    _NL_MONETARY_DUO_INT_P_CS_PRECEDES = ffi::_NL_MONETARY_DUO_INT_P_CS_PRECEDES as isize,
    _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE = ffi::_NL_MONETARY_DUO_INT_P_SEP_BY_SPACE as isize,
    _NL_MONETARY_DUO_INT_N_CS_PRECEDES = ffi::_NL_MONETARY_DUO_INT_N_CS_PRECEDES as isize,
    _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE = ffi::_NL_MONETARY_DUO_INT_N_SEP_BY_SPACE as isize,
    _NL_MONETARY_DUO_P_SIGN_POSN = ffi::_NL_MONETARY_DUO_P_SIGN_POSN as isize,
    _NL_MONETARY_DUO_N_SIGN_POSN = ffi::_NL_MONETARY_DUO_N_SIGN_POSN as isize,
    _NL_MONETARY_DUO_INT_P_SIGN_POSN = ffi::_NL_MONETARY_DUO_INT_P_SIGN_POSN as isize,
    _NL_MONETARY_DUO_INT_N_SIGN_POSN = ffi::_NL_MONETARY_DUO_INT_N_SIGN_POSN as isize,
    // Time
    _NL_TIME_WEEK_NDAYS = ffi::_NL_TIME_WEEK_NDAYS as isize,
    _NL_TIME_WEEK_1STWEEK = ffi::_NL_TIME_WEEK_1STWEEK as isize,
    _NL_TIME_FIRST_WEEKDAY = ffi::_NL_TIME_FIRST_WEEKDAY as isize,
    _NL_TIME_FIRST_WORKDAY = ffi::_NL_TIME_FIRST_WORKDAY as isize,
    _NL_TIME_CAL_DIRECTION = ffi::_NL_TIME_CAL_DIRECTION as isize,
    // Measurement
    _NL_MEASUREMENT_MEASUREMENT = ffi::_NL_MEASUREMENT_MEASUREMENT as isize,
}

impl<'a> LanginfoItem<'a> for ByteItems {
    type Type = i8;
    fn needs_iconv() -> Option<CodesetItems> { None }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, _: Option<&IConv>) -> i8 {
        *ptr as i8
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::ByteItems::*;

#[derive(Copy, Clone, Debug)]
pub enum ByteArrayItems {
    // CType
    _NL_CTYPE_WIDTH = ffi::_NL_CTYPE_WIDTH as isize,
    // Monetary
    __MON_GROUPING = ffi::__MON_GROUPING as isize,
    // Numeric
    __GROUPING = ffi::__GROUPING as isize,
}

impl<'a> LanginfoItem<'a> for ByteArrayItems {
    type Type = &'a [i8];
    fn needs_iconv() -> Option<CodesetItems> { None }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, _: Option<&IConv>) -> &'a [i8] {
        decode_bytes(ptr)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::ByteArrayItems::*;

#[derive(Copy, Clone, Debug)]
pub enum IntegralItems {
    // Collate
    _NL_COLLATE_NRULES = ffi::_NL_COLLATE_NRULES as isize,
    _NL_COLLATE_SYMB_HASH_SIZEMB = ffi::_NL_COLLATE_SYMB_HASH_SIZEMB as isize,
    // CType
    _NL_CTYPE_MB_CUR_MAX = ffi::_NL_CTYPE_MB_CUR_MAX as isize,
    _NL_CTYPE_CLASS_OFFSET = ffi::_NL_CTYPE_CLASS_OFFSET as isize,
    _NL_CTYPE_MAP_OFFSET = ffi::_NL_CTYPE_MAP_OFFSET as isize,
    _NL_CTYPE_INDIGITS_MB_LEN = ffi::_NL_CTYPE_INDIGITS_MB_LEN as isize,
    _NL_CTYPE_INDIGITS_WC_LEN = ffi::_NL_CTYPE_INDIGITS_WC_LEN as isize,
    _NL_CTYPE_TRANSLIT_TAB_SIZE = ffi::_NL_CTYPE_TRANSLIT_TAB_SIZE as isize,
    _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN = ffi::_NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN as isize,
    _NL_CTYPE_TRANSLIT_IGNORE_LEN = ffi::_NL_CTYPE_TRANSLIT_IGNORE_LEN as isize,
    _NL_CTYPE_MAP_TO_NONASCII = ffi::_NL_CTYPE_MAP_TO_NONASCII as isize,
    _NL_CTYPE_NONASCII_CASE = ffi::_NL_CTYPE_NONASCII_CASE as isize,
    // Monetary
    _NL_MONETARY_UNO_VALID_FROM = ffi::_NL_MONETARY_UNO_VALID_FROM as isize,
    _NL_MONETARY_UNO_VALID_TO = ffi::_NL_MONETARY_UNO_VALID_TO as isize,
    _NL_MONETARY_DUO_VALID_FROM = ffi::_NL_MONETARY_DUO_VALID_FROM as isize,
    _NL_MONETARY_DUO_VALID_TO = ffi::_NL_MONETARY_DUO_VALID_TO as isize,
    // Time
    _NL_TIME_ERA_NUM_ENTRIES = ffi::_NL_TIME_ERA_NUM_ENTRIES as isize,
    _NL_TIME_WEEK_1STDAY = ffi::_NL_TIME_WEEK_1STDAY as isize,
    // Paper
    _NL_PAPER_HEIGHT = ffi::_NL_PAPER_HEIGHT as isize,
    _NL_PAPER_WIDTH = ffi::_NL_PAPER_WIDTH as isize,
    // Address
    _NL_ADDRESS_COUNTRY_NUM = ffi::_NL_ADDRESS_COUNTRY_NUM as isize,
}

impl<'a> LanginfoItem<'a> for IntegralItems {
    type Type = u32;
    fn needs_iconv() -> Option<CodesetItems> { None }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, _: Option<&IConv>) -> u32 {
        transmute_copy(&ptr)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::IntegralItems::*;

#[derive(Copy, Clone, Debug)]
pub enum CharacterItems {
    // CType
    _NL_CTYPE_OUTDIGIT0_WC = ffi::_NL_CTYPE_OUTDIGIT0_WC as isize,
    _NL_CTYPE_OUTDIGIT1_WC = ffi::_NL_CTYPE_OUTDIGIT1_WC as isize,
    _NL_CTYPE_OUTDIGIT2_WC = ffi::_NL_CTYPE_OUTDIGIT2_WC as isize,
    _NL_CTYPE_OUTDIGIT3_WC = ffi::_NL_CTYPE_OUTDIGIT3_WC as isize,
    _NL_CTYPE_OUTDIGIT4_WC = ffi::_NL_CTYPE_OUTDIGIT4_WC as isize,
    _NL_CTYPE_OUTDIGIT5_WC = ffi::_NL_CTYPE_OUTDIGIT5_WC as isize,
    _NL_CTYPE_OUTDIGIT6_WC = ffi::_NL_CTYPE_OUTDIGIT6_WC as isize,
    _NL_CTYPE_OUTDIGIT7_WC = ffi::_NL_CTYPE_OUTDIGIT7_WC as isize,
    _NL_CTYPE_OUTDIGIT8_WC = ffi::_NL_CTYPE_OUTDIGIT8_WC as isize,
    _NL_CTYPE_OUTDIGIT9_WC = ffi::_NL_CTYPE_OUTDIGIT9_WC as isize,
    // Monetary
    _NL_MONETARY_DECIMAL_POINT_WC = ffi::_NL_MONETARY_DECIMAL_POINT_WC as isize,
    _NL_MONETARY_THOUSANDS_SEP_WC = ffi::_NL_MONETARY_THOUSANDS_SEP_WC as isize,
    // Numeric
    _NL_NUMERIC_DECIMAL_POINT_WC = ffi::_NL_NUMERIC_DECIMAL_POINT_WC as isize,
    _NL_NUMERIC_THOUSANDS_SEP_WC = ffi::_NL_NUMERIC_THOUSANDS_SEP_WC as isize,
}

impl<'a> LanginfoItem<'a> for CharacterItems {
    type Type = char;
    fn needs_iconv() -> Option<CodesetItems> { None }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, _: Option<&IConv>) -> char {
        transmute_copy(&ptr)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::CharacterItems::*;

#[derive(Copy, Clone, Debug)]
pub enum IdentificationStringItems {
    _NL_IDENTIFICATION_TITLE = ffi::_NL_IDENTIFICATION_TITLE as isize,
    _NL_IDENTIFICATION_SOURCE = ffi::_NL_IDENTIFICATION_SOURCE as isize,
    _NL_IDENTIFICATION_ADDRESS = ffi::_NL_IDENTIFICATION_ADDRESS as isize,
    _NL_IDENTIFICATION_CONTACT = ffi::_NL_IDENTIFICATION_CONTACT as isize,
    _NL_IDENTIFICATION_EMAIL = ffi::_NL_IDENTIFICATION_EMAIL as isize,
    _NL_IDENTIFICATION_TEL = ffi::_NL_IDENTIFICATION_TEL as isize,
    _NL_IDENTIFICATION_FAX = ffi::_NL_IDENTIFICATION_FAX as isize,
    _NL_IDENTIFICATION_LANGUAGE = ffi::_NL_IDENTIFICATION_LANGUAGE as isize,
    _NL_IDENTIFICATION_TERRITORY = ffi::_NL_IDENTIFICATION_TERRITORY as isize,
    _NL_IDENTIFICATION_AUDIENCE = ffi::_NL_IDENTIFICATION_AUDIENCE as isize,
    _NL_IDENTIFICATION_APPLICATION = ffi::_NL_IDENTIFICATION_APPLICATION as isize,
    _NL_IDENTIFICATION_ABBREVIATION = ffi::_NL_IDENTIFICATION_ABBREVIATION as isize,
    _NL_IDENTIFICATION_REVISION = ffi::_NL_IDENTIFICATION_REVISION as isize,
    _NL_IDENTIFICATION_DATE = ffi::_NL_IDENTIFICATION_DATE as isize,
    _NL_IDENTIFICATION_CATEGORY = ffi::_NL_IDENTIFICATION_CATEGORY as isize,
    _NL_IDENTIFICATION_CATEGORY_1,
    _NL_IDENTIFICATION_CATEGORY_2,
    _NL_IDENTIFICATION_CATEGORY_3,
    _NL_IDENTIFICATION_CATEGORY_4,
    _NL_IDENTIFICATION_CATEGORY_5,
    _NL_IDENTIFICATION_CATEGORY_6,
    _NL_IDENTIFICATION_CATEGORY_7,
    _NL_IDENTIFICATION_CATEGORY_8,
    _NL_IDENTIFICATION_CATEGORY_9,
    _NL_IDENTIFICATION_CATEGORY_10,
    _NL_IDENTIFICATION_CATEGORY_11,
    _NL_IDENTIFICATION_CATEGORY_12,
}

impl<'a> LanginfoItem<'a> for IdentificationStringItems {
    type Type = Cow<'a, str>;
    fn needs_iconv() -> Option<CodesetItems> { Some(CodesetItems::_NL_IDENTIFICATION_CODESET) }
    unsafe fn decode(&self, ptr: *const ::libc::c_char, iconv: Option<&IConv>) -> Cow<'a, str> {
        decode_string(ptr, iconv)
    }
    fn to_ffi(self) -> ffi::nl_item { self as ffi::nl_item }
}

pub use self::IdentificationStringItems::*;
