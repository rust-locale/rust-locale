/* originally generated by rust-bindgen */

//! Bindings for GNU LibC localization functions.
//!
//! This was generated by rust-bindgen from `<locale.h>`, `<langinfo.h>` and `<iconv.h>`. Iconv is
//! included for converting output from nl_langinfo_l to utf-8 in case the user-selected locale is
//! not utf-8. Thankfully in Linux all these functions are part of the libc itself, so they are
//! already available and we don't need to do any additional linking and so there is no need to put
//! this in separate crate either.

#![allow(non_camel_case_types)]

// Note: rust-bindgen does not generate defines, so these had to be cone manually. Fortunately the
// parameters for nl_langinfo are in anonymous enum and were generated.
pub const LC_CTYPE: ::libc::c_int = 0;
pub const LC_NUMERIC: ::libc::c_int = 1;
pub const LC_TIME: ::libc::c_int = 2;
pub const LC_COLLATE: ::libc::c_int = 3;
pub const LC_MONETARY: ::libc::c_int = 4;
pub const LC_MESSAGES: ::libc::c_int = 5;
pub const LC_ALL: ::libc::c_int = 6;
pub const LC_PAPER: ::libc::c_int = 7;
pub const LC_NAME: ::libc::c_int = 8;
pub const LC_ADDRESS: ::libc::c_int = 9;
pub const LC_TELEPHONE: ::libc::c_int = 10;
pub const LC_MEASUREMENT: ::libc::c_int = 11;
pub const LC_IDENTIFICATION: ::libc::c_int = 12;

pub const LC_CTYPE_MASK: ::libc::c_int = 1 << LC_CTYPE;
pub const LC_NUMERIC_MASK: ::libc::c_int = 1 << LC_NUMERIC;
pub const LC_TIME_MASK: ::libc::c_int = 1 << LC_TIME;
pub const LC_COLLATE_MASK: ::libc::c_int = 1 << LC_COLLATE;
pub const LC_MONETARY_MASK: ::libc::c_int = 1 << LC_MONETARY;
pub const LC_MESSAGES_MASK: ::libc::c_int = 1 << LC_MESSAGES;
pub const LC_PAPER_MASK: ::libc::c_int = 1 << LC_PAPER;
pub const LC_NAME_MASK: ::libc::c_int = 1 << LC_NAME;
pub const LC_ADDRESS_MASK: ::libc::c_int = 1 << LC_ADDRESS;
pub const LC_TELEPHONE_MASK: ::libc::c_int = 1 << LC_TELEPHONE;
pub const LC_MEASUREMENT_MASK: ::libc::c_int = 1 << LC_MEASUREMENT;
pub const LC_IDENTIFICATION_MASK: ::libc::c_int = 1 << LC_IDENTIFICATION;
pub const LC_ALL_MASK: ::libc::c_int = LC_CTYPE_MASK
				 | LC_NUMERIC_MASK
				 | LC_TIME_MASK
				 | LC_COLLATE_MASK
				 | LC_MONETARY_MASK
				 | LC_MESSAGES_MASK
				 | LC_PAPER_MASK
				 | LC_NAME_MASK
				 | LC_ADDRESS_MASK
				 | LC_TELEPHONE_MASK
				 | LC_MEASUREMENT_MASK
				 | LC_IDENTIFICATION_MASK;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_lconv {
    pub decimal_point: *mut ::libc::c_char,
    pub thousands_sep: *mut ::libc::c_char,
    pub grouping: *mut ::libc::c_char,
    pub int_curr_symbol: *mut ::libc::c_char,
    pub currency_symbol: *mut ::libc::c_char,
    pub mon_decimal_point: *mut ::libc::c_char,
    pub mon_thousands_sep: *mut ::libc::c_char,
    pub mon_grouping: *mut ::libc::c_char,
    pub positive_sign: *mut ::libc::c_char,
    pub negative_sign: *mut ::libc::c_char,
    pub int_frac_digits: ::libc::c_char,
    pub frac_digits: ::libc::c_char,
    pub p_cs_precedes: ::libc::c_char,
    pub p_sep_by_space: ::libc::c_char,
    pub n_cs_precedes: ::libc::c_char,
    pub n_sep_by_space: ::libc::c_char,
    pub p_sign_posn: ::libc::c_char,
    pub n_sign_posn: ::libc::c_char,
    pub int_p_cs_precedes: ::libc::c_char,
    pub int_p_sep_by_space: ::libc::c_char,
    pub int_n_cs_precedes: ::libc::c_char,
    pub int_n_sep_by_space: ::libc::c_char,
    pub int_p_sign_posn: ::libc::c_char,
    pub int_n_sign_posn: ::libc::c_char,
}
impl ::std::default::Default for Struct_lconv {
    fn default() -> Struct_lconv { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct___locale_data { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct___locale_struct {
    pub __locales: [*mut Struct___locale_data; 13usize],
    pub __ctype_b: *const ::libc::c_ushort,
    pub __ctype_tolower: *const ::libc::c_int,
    pub __ctype_toupper: *const ::libc::c_int,
    pub __names: [*const ::libc::c_char; 13usize],
}
impl ::std::default::Default for Struct___locale_struct {
    fn default() -> Struct___locale_struct { unsafe { ::std::mem::zeroed() } }
}
pub type __locale_t = *mut Struct___locale_struct;
pub type locale_t = __locale_t;
pub type nl_catd = *mut ::libc::c_void;
pub type nl_item = ::libc::c_uint;
pub type Enum_Unnamed1 = ::libc::c_uint;
pub const ABDAY_1: ::libc::c_uint = 131072;
pub const ABDAY_2: ::libc::c_uint = 131073;
pub const ABDAY_3: ::libc::c_uint = 131074;
pub const ABDAY_4: ::libc::c_uint = 131075;
pub const ABDAY_5: ::libc::c_uint = 131076;
pub const ABDAY_6: ::libc::c_uint = 131077;
pub const ABDAY_7: ::libc::c_uint = 131078;
pub const DAY_1: ::libc::c_uint = 131079;
pub const DAY_2: ::libc::c_uint = 131080;
pub const DAY_3: ::libc::c_uint = 131081;
pub const DAY_4: ::libc::c_uint = 131082;
pub const DAY_5: ::libc::c_uint = 131083;
pub const DAY_6: ::libc::c_uint = 131084;
pub const DAY_7: ::libc::c_uint = 131085;
pub const ABMON_1: ::libc::c_uint = 131086;
pub const ABMON_2: ::libc::c_uint = 131087;
pub const ABMON_3: ::libc::c_uint = 131088;
pub const ABMON_4: ::libc::c_uint = 131089;
pub const ABMON_5: ::libc::c_uint = 131090;
pub const ABMON_6: ::libc::c_uint = 131091;
pub const ABMON_7: ::libc::c_uint = 131092;
pub const ABMON_8: ::libc::c_uint = 131093;
pub const ABMON_9: ::libc::c_uint = 131094;
pub const ABMON_10: ::libc::c_uint = 131095;
pub const ABMON_11: ::libc::c_uint = 131096;
pub const ABMON_12: ::libc::c_uint = 131097;
pub const MON_1: ::libc::c_uint = 131098;
pub const MON_2: ::libc::c_uint = 131099;
pub const MON_3: ::libc::c_uint = 131100;
pub const MON_4: ::libc::c_uint = 131101;
pub const MON_5: ::libc::c_uint = 131102;
pub const MON_6: ::libc::c_uint = 131103;
pub const MON_7: ::libc::c_uint = 131104;
pub const MON_8: ::libc::c_uint = 131105;
pub const MON_9: ::libc::c_uint = 131106;
pub const MON_10: ::libc::c_uint = 131107;
pub const MON_11: ::libc::c_uint = 131108;
pub const MON_12: ::libc::c_uint = 131109;
pub const AM_STR: ::libc::c_uint = 131110;
pub const PM_STR: ::libc::c_uint = 131111;
pub const D_T_FMT: ::libc::c_uint = 131112;
pub const D_FMT: ::libc::c_uint = 131113;
pub const T_FMT: ::libc::c_uint = 131114;
pub const T_FMT_AMPM: ::libc::c_uint = 131115;
pub const ERA: ::libc::c_uint = 131116;
pub const __ERA_YEAR: ::libc::c_uint = 131117;
pub const ERA_D_FMT: ::libc::c_uint = 131118;
pub const ALT_DIGITS: ::libc::c_uint = 131119;
pub const ERA_D_T_FMT: ::libc::c_uint = 131120;
pub const ERA_T_FMT: ::libc::c_uint = 131121;
pub const _NL_TIME_ERA_NUM_ENTRIES: ::libc::c_uint = 131122;
pub const _NL_TIME_ERA_ENTRIES: ::libc::c_uint = 131123;
pub const _NL_WABDAY_1: ::libc::c_uint = 131124;
pub const _NL_WABDAY_2: ::libc::c_uint = 131125;
pub const _NL_WABDAY_3: ::libc::c_uint = 131126;
pub const _NL_WABDAY_4: ::libc::c_uint = 131127;
pub const _NL_WABDAY_5: ::libc::c_uint = 131128;
pub const _NL_WABDAY_6: ::libc::c_uint = 131129;
pub const _NL_WABDAY_7: ::libc::c_uint = 131130;
pub const _NL_WDAY_1: ::libc::c_uint = 131131;
pub const _NL_WDAY_2: ::libc::c_uint = 131132;
pub const _NL_WDAY_3: ::libc::c_uint = 131133;
pub const _NL_WDAY_4: ::libc::c_uint = 131134;
pub const _NL_WDAY_5: ::libc::c_uint = 131135;
pub const _NL_WDAY_6: ::libc::c_uint = 131136;
pub const _NL_WDAY_7: ::libc::c_uint = 131137;
pub const _NL_WABMON_1: ::libc::c_uint = 131138;
pub const _NL_WABMON_2: ::libc::c_uint = 131139;
pub const _NL_WABMON_3: ::libc::c_uint = 131140;
pub const _NL_WABMON_4: ::libc::c_uint = 131141;
pub const _NL_WABMON_5: ::libc::c_uint = 131142;
pub const _NL_WABMON_6: ::libc::c_uint = 131143;
pub const _NL_WABMON_7: ::libc::c_uint = 131144;
pub const _NL_WABMON_8: ::libc::c_uint = 131145;
pub const _NL_WABMON_9: ::libc::c_uint = 131146;
pub const _NL_WABMON_10: ::libc::c_uint = 131147;
pub const _NL_WABMON_11: ::libc::c_uint = 131148;
pub const _NL_WABMON_12: ::libc::c_uint = 131149;
pub const _NL_WMON_1: ::libc::c_uint = 131150;
pub const _NL_WMON_2: ::libc::c_uint = 131151;
pub const _NL_WMON_3: ::libc::c_uint = 131152;
pub const _NL_WMON_4: ::libc::c_uint = 131153;
pub const _NL_WMON_5: ::libc::c_uint = 131154;
pub const _NL_WMON_6: ::libc::c_uint = 131155;
pub const _NL_WMON_7: ::libc::c_uint = 131156;
pub const _NL_WMON_8: ::libc::c_uint = 131157;
pub const _NL_WMON_9: ::libc::c_uint = 131158;
pub const _NL_WMON_10: ::libc::c_uint = 131159;
pub const _NL_WMON_11: ::libc::c_uint = 131160;
pub const _NL_WMON_12: ::libc::c_uint = 131161;
pub const _NL_WAM_STR: ::libc::c_uint = 131162;
pub const _NL_WPM_STR: ::libc::c_uint = 131163;
pub const _NL_WD_T_FMT: ::libc::c_uint = 131164;
pub const _NL_WD_FMT: ::libc::c_uint = 131165;
pub const _NL_WT_FMT: ::libc::c_uint = 131166;
pub const _NL_WT_FMT_AMPM: ::libc::c_uint = 131167;
pub const _NL_WERA_YEAR: ::libc::c_uint = 131168;
pub const _NL_WERA_D_FMT: ::libc::c_uint = 131169;
pub const _NL_WALT_DIGITS: ::libc::c_uint = 131170;
pub const _NL_WERA_D_T_FMT: ::libc::c_uint = 131171;
pub const _NL_WERA_T_FMT: ::libc::c_uint = 131172;
pub const _NL_TIME_WEEK_NDAYS: ::libc::c_uint = 131173;
pub const _NL_TIME_WEEK_1STDAY: ::libc::c_uint = 131174;
pub const _NL_TIME_WEEK_1STWEEK: ::libc::c_uint = 131175;
pub const _NL_TIME_FIRST_WEEKDAY: ::libc::c_uint = 131176;
pub const _NL_TIME_FIRST_WORKDAY: ::libc::c_uint = 131177;
pub const _NL_TIME_CAL_DIRECTION: ::libc::c_uint = 131178;
pub const _NL_TIME_TIMEZONE: ::libc::c_uint = 131179;
pub const _DATE_FMT: ::libc::c_uint = 131180;
pub const _NL_W_DATE_FMT: ::libc::c_uint = 131181;
pub const _NL_TIME_CODESET: ::libc::c_uint = 131182;
pub const _NL_NUM_LC_TIME: ::libc::c_uint = 131183;
pub const _NL_COLLATE_NRULES: ::libc::c_uint = 196608;
pub const _NL_COLLATE_RULESETS: ::libc::c_uint = 196609;
pub const _NL_COLLATE_TABLEMB: ::libc::c_uint = 196610;
pub const _NL_COLLATE_WEIGHTMB: ::libc::c_uint = 196611;
pub const _NL_COLLATE_EXTRAMB: ::libc::c_uint = 196612;
pub const _NL_COLLATE_INDIRECTMB: ::libc::c_uint = 196613;
pub const _NL_COLLATE_GAP1: ::libc::c_uint = 196614;
pub const _NL_COLLATE_GAP2: ::libc::c_uint = 196615;
pub const _NL_COLLATE_GAP3: ::libc::c_uint = 196616;
pub const _NL_COLLATE_TABLEWC: ::libc::c_uint = 196617;
pub const _NL_COLLATE_WEIGHTWC: ::libc::c_uint = 196618;
pub const _NL_COLLATE_EXTRAWC: ::libc::c_uint = 196619;
pub const _NL_COLLATE_INDIRECTWC: ::libc::c_uint = 196620;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: ::libc::c_uint = 196621;
pub const _NL_COLLATE_SYMB_TABLEMB: ::libc::c_uint = 196622;
pub const _NL_COLLATE_SYMB_EXTRAMB: ::libc::c_uint = 196623;
pub const _NL_COLLATE_COLLSEQMB: ::libc::c_uint = 196624;
pub const _NL_COLLATE_COLLSEQWC: ::libc::c_uint = 196625;
pub const _NL_COLLATE_CODESET: ::libc::c_uint = 196626;
pub const _NL_NUM_LC_COLLATE: ::libc::c_uint = 196627;
pub const _NL_CTYPE_CLASS: ::libc::c_uint = 0;
pub const _NL_CTYPE_TOUPPER: ::libc::c_uint = 1;
pub const _NL_CTYPE_GAP1: ::libc::c_uint = 2;
pub const _NL_CTYPE_TOLOWER: ::libc::c_uint = 3;
pub const _NL_CTYPE_GAP2: ::libc::c_uint = 4;
pub const _NL_CTYPE_CLASS32: ::libc::c_uint = 5;
pub const _NL_CTYPE_GAP3: ::libc::c_uint = 6;
pub const _NL_CTYPE_GAP4: ::libc::c_uint = 7;
pub const _NL_CTYPE_GAP5: ::libc::c_uint = 8;
pub const _NL_CTYPE_GAP6: ::libc::c_uint = 9;
pub const _NL_CTYPE_CLASS_NAMES: ::libc::c_uint = 10;
pub const _NL_CTYPE_MAP_NAMES: ::libc::c_uint = 11;
pub const _NL_CTYPE_WIDTH: ::libc::c_uint = 12;
pub const _NL_CTYPE_MB_CUR_MAX: ::libc::c_uint = 13;
pub const _NL_CTYPE_CODESET_NAME: ::libc::c_uint = 14;
pub const CODESET: ::libc::c_uint = 14;
pub const _NL_CTYPE_TOUPPER32: ::libc::c_uint = 15;
pub const _NL_CTYPE_TOLOWER32: ::libc::c_uint = 16;
pub const _NL_CTYPE_CLASS_OFFSET: ::libc::c_uint = 17;
pub const _NL_CTYPE_MAP_OFFSET: ::libc::c_uint = 18;
pub const _NL_CTYPE_INDIGITS_MB_LEN: ::libc::c_uint = 19;
pub const _NL_CTYPE_INDIGITS0_MB: ::libc::c_uint = 20;
pub const _NL_CTYPE_INDIGITS1_MB: ::libc::c_uint = 21;
pub const _NL_CTYPE_INDIGITS2_MB: ::libc::c_uint = 22;
pub const _NL_CTYPE_INDIGITS3_MB: ::libc::c_uint = 23;
pub const _NL_CTYPE_INDIGITS4_MB: ::libc::c_uint = 24;
pub const _NL_CTYPE_INDIGITS5_MB: ::libc::c_uint = 25;
pub const _NL_CTYPE_INDIGITS6_MB: ::libc::c_uint = 26;
pub const _NL_CTYPE_INDIGITS7_MB: ::libc::c_uint = 27;
pub const _NL_CTYPE_INDIGITS8_MB: ::libc::c_uint = 28;
pub const _NL_CTYPE_INDIGITS9_MB: ::libc::c_uint = 29;
pub const _NL_CTYPE_INDIGITS_WC_LEN: ::libc::c_uint = 30;
pub const _NL_CTYPE_INDIGITS0_WC: ::libc::c_uint = 31;
pub const _NL_CTYPE_INDIGITS1_WC: ::libc::c_uint = 32;
pub const _NL_CTYPE_INDIGITS2_WC: ::libc::c_uint = 33;
pub const _NL_CTYPE_INDIGITS3_WC: ::libc::c_uint = 34;
pub const _NL_CTYPE_INDIGITS4_WC: ::libc::c_uint = 35;
pub const _NL_CTYPE_INDIGITS5_WC: ::libc::c_uint = 36;
pub const _NL_CTYPE_INDIGITS6_WC: ::libc::c_uint = 37;
pub const _NL_CTYPE_INDIGITS7_WC: ::libc::c_uint = 38;
pub const _NL_CTYPE_INDIGITS8_WC: ::libc::c_uint = 39;
pub const _NL_CTYPE_INDIGITS9_WC: ::libc::c_uint = 40;
pub const _NL_CTYPE_OUTDIGIT0_MB: ::libc::c_uint = 41;
pub const _NL_CTYPE_OUTDIGIT1_MB: ::libc::c_uint = 42;
pub const _NL_CTYPE_OUTDIGIT2_MB: ::libc::c_uint = 43;
pub const _NL_CTYPE_OUTDIGIT3_MB: ::libc::c_uint = 44;
pub const _NL_CTYPE_OUTDIGIT4_MB: ::libc::c_uint = 45;
pub const _NL_CTYPE_OUTDIGIT5_MB: ::libc::c_uint = 46;
pub const _NL_CTYPE_OUTDIGIT6_MB: ::libc::c_uint = 47;
pub const _NL_CTYPE_OUTDIGIT7_MB: ::libc::c_uint = 48;
pub const _NL_CTYPE_OUTDIGIT8_MB: ::libc::c_uint = 49;
pub const _NL_CTYPE_OUTDIGIT9_MB: ::libc::c_uint = 50;
pub const _NL_CTYPE_OUTDIGIT0_WC: ::libc::c_uint = 51;
pub const _NL_CTYPE_OUTDIGIT1_WC: ::libc::c_uint = 52;
pub const _NL_CTYPE_OUTDIGIT2_WC: ::libc::c_uint = 53;
pub const _NL_CTYPE_OUTDIGIT3_WC: ::libc::c_uint = 54;
pub const _NL_CTYPE_OUTDIGIT4_WC: ::libc::c_uint = 55;
pub const _NL_CTYPE_OUTDIGIT5_WC: ::libc::c_uint = 56;
pub const _NL_CTYPE_OUTDIGIT6_WC: ::libc::c_uint = 57;
pub const _NL_CTYPE_OUTDIGIT7_WC: ::libc::c_uint = 58;
pub const _NL_CTYPE_OUTDIGIT8_WC: ::libc::c_uint = 59;
pub const _NL_CTYPE_OUTDIGIT9_WC: ::libc::c_uint = 60;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: ::libc::c_uint = 61;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: ::libc::c_uint = 62;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: ::libc::c_uint = 63;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: ::libc::c_uint = 64;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: ::libc::c_uint = 65;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: ::libc::c_uint = 66;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: ::libc::c_uint = 67;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: ::libc::c_uint = 68;
pub const _NL_CTYPE_TRANSLIT_IGNORE: ::libc::c_uint = 69;
pub const _NL_CTYPE_MAP_TO_NONASCII: ::libc::c_uint = 70;
pub const _NL_CTYPE_NONASCII_CASE: ::libc::c_uint = 71;
pub const _NL_CTYPE_EXTRA_MAP_1: ::libc::c_uint = 72;
pub const _NL_CTYPE_EXTRA_MAP_2: ::libc::c_uint = 73;
pub const _NL_CTYPE_EXTRA_MAP_3: ::libc::c_uint = 74;
pub const _NL_CTYPE_EXTRA_MAP_4: ::libc::c_uint = 75;
pub const _NL_CTYPE_EXTRA_MAP_5: ::libc::c_uint = 76;
pub const _NL_CTYPE_EXTRA_MAP_6: ::libc::c_uint = 77;
pub const _NL_CTYPE_EXTRA_MAP_7: ::libc::c_uint = 78;
pub const _NL_CTYPE_EXTRA_MAP_8: ::libc::c_uint = 79;
pub const _NL_CTYPE_EXTRA_MAP_9: ::libc::c_uint = 80;
pub const _NL_CTYPE_EXTRA_MAP_10: ::libc::c_uint = 81;
pub const _NL_CTYPE_EXTRA_MAP_11: ::libc::c_uint = 82;
pub const _NL_CTYPE_EXTRA_MAP_12: ::libc::c_uint = 83;
pub const _NL_CTYPE_EXTRA_MAP_13: ::libc::c_uint = 84;
pub const _NL_CTYPE_EXTRA_MAP_14: ::libc::c_uint = 85;
pub const _NL_NUM_LC_CTYPE: ::libc::c_uint = 86;
pub const __INT_CURR_SYMBOL: ::libc::c_uint = 262144;
pub const __CURRENCY_SYMBOL: ::libc::c_uint = 262145;
pub const __MON_DECIMAL_POINT: ::libc::c_uint = 262146;
pub const __MON_THOUSANDS_SEP: ::libc::c_uint = 262147;
pub const __MON_GROUPING: ::libc::c_uint = 262148;
pub const __POSITIVE_SIGN: ::libc::c_uint = 262149;
pub const __NEGATIVE_SIGN: ::libc::c_uint = 262150;
pub const __INT_FRAC_DIGITS: ::libc::c_uint = 262151;
pub const __FRAC_DIGITS: ::libc::c_uint = 262152;
pub const __P_CS_PRECEDES: ::libc::c_uint = 262153;
pub const __P_SEP_BY_SPACE: ::libc::c_uint = 262154;
pub const __N_CS_PRECEDES: ::libc::c_uint = 262155;
pub const __N_SEP_BY_SPACE: ::libc::c_uint = 262156;
pub const __P_SIGN_POSN: ::libc::c_uint = 262157;
pub const __N_SIGN_POSN: ::libc::c_uint = 262158;
pub const _NL_MONETARY_CRNCYSTR: ::libc::c_uint = 262159;
pub const __INT_P_CS_PRECEDES: ::libc::c_uint = 262160;
pub const __INT_P_SEP_BY_SPACE: ::libc::c_uint = 262161;
pub const __INT_N_CS_PRECEDES: ::libc::c_uint = 262162;
pub const __INT_N_SEP_BY_SPACE: ::libc::c_uint = 262163;
pub const __INT_P_SIGN_POSN: ::libc::c_uint = 262164;
pub const __INT_N_SIGN_POSN: ::libc::c_uint = 262165;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: ::libc::c_uint = 262166;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: ::libc::c_uint = 262167;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: ::libc::c_uint = 262168;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: ::libc::c_uint = 262169;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: ::libc::c_uint = 262170;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: ::libc::c_uint = 262171;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: ::libc::c_uint = 262172;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: ::libc::c_uint = 262173;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: ::libc::c_uint = 262174;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: ::libc::c_uint = 262175;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: ::libc::c_uint = 262176;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: ::libc::c_uint = 262177;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: ::libc::c_uint = 262178;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: ::libc::c_uint = 262179;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: ::libc::c_uint = 262180;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: ::libc::c_uint = 262181;
pub const _NL_MONETARY_UNO_VALID_FROM: ::libc::c_uint = 262182;
pub const _NL_MONETARY_UNO_VALID_TO: ::libc::c_uint = 262183;
pub const _NL_MONETARY_DUO_VALID_FROM: ::libc::c_uint = 262184;
pub const _NL_MONETARY_DUO_VALID_TO: ::libc::c_uint = 262185;
pub const _NL_MONETARY_CONVERSION_RATE: ::libc::c_uint = 262186;
pub const _NL_MONETARY_DECIMAL_POINT_WC: ::libc::c_uint = 262187;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: ::libc::c_uint = 262188;
pub const _NL_MONETARY_CODESET: ::libc::c_uint = 262189;
pub const _NL_NUM_LC_MONETARY: ::libc::c_uint = 262190;
pub const __DECIMAL_POINT: ::libc::c_uint = 65536;
pub const RADIXCHAR: ::libc::c_uint = 65536;
pub const __THOUSANDS_SEP: ::libc::c_uint = 65537;
pub const THOUSEP: ::libc::c_uint = 65537;
pub const __GROUPING: ::libc::c_uint = 65538;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: ::libc::c_uint = 65539;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: ::libc::c_uint = 65540;
pub const _NL_NUMERIC_CODESET: ::libc::c_uint = 65541;
pub const _NL_NUM_LC_NUMERIC: ::libc::c_uint = 65542;
pub const __YESEXPR: ::libc::c_uint = 327680;
pub const __NOEXPR: ::libc::c_uint = 327681;
pub const __YESSTR: ::libc::c_uint = 327682;
pub const __NOSTR: ::libc::c_uint = 327683;
pub const _NL_MESSAGES_CODESET: ::libc::c_uint = 327684;
pub const _NL_NUM_LC_MESSAGES: ::libc::c_uint = 327685;
pub const _NL_PAPER_HEIGHT: ::libc::c_uint = 458752;
pub const _NL_PAPER_WIDTH: ::libc::c_uint = 458753;
pub const _NL_PAPER_CODESET: ::libc::c_uint = 458754;
pub const _NL_NUM_LC_PAPER: ::libc::c_uint = 458755;
pub const _NL_NAME_NAME_FMT: ::libc::c_uint = 524288;
pub const _NL_NAME_NAME_GEN: ::libc::c_uint = 524289;
pub const _NL_NAME_NAME_MR: ::libc::c_uint = 524290;
pub const _NL_NAME_NAME_MRS: ::libc::c_uint = 524291;
pub const _NL_NAME_NAME_MISS: ::libc::c_uint = 524292;
pub const _NL_NAME_NAME_MS: ::libc::c_uint = 524293;
pub const _NL_NAME_CODESET: ::libc::c_uint = 524294;
pub const _NL_NUM_LC_NAME: ::libc::c_uint = 524295;
pub const _NL_ADDRESS_POSTAL_FMT: ::libc::c_uint = 589824;
pub const _NL_ADDRESS_COUNTRY_NAME: ::libc::c_uint = 589825;
pub const _NL_ADDRESS_COUNTRY_POST: ::libc::c_uint = 589826;
pub const _NL_ADDRESS_COUNTRY_AB2: ::libc::c_uint = 589827;
pub const _NL_ADDRESS_COUNTRY_AB3: ::libc::c_uint = 589828;
pub const _NL_ADDRESS_COUNTRY_CAR: ::libc::c_uint = 589829;
pub const _NL_ADDRESS_COUNTRY_NUM: ::libc::c_uint = 589830;
pub const _NL_ADDRESS_COUNTRY_ISBN: ::libc::c_uint = 589831;
pub const _NL_ADDRESS_LANG_NAME: ::libc::c_uint = 589832;
pub const _NL_ADDRESS_LANG_AB: ::libc::c_uint = 589833;
pub const _NL_ADDRESS_LANG_TERM: ::libc::c_uint = 589834;
pub const _NL_ADDRESS_LANG_LIB: ::libc::c_uint = 589835;
pub const _NL_ADDRESS_CODESET: ::libc::c_uint = 589836;
pub const _NL_NUM_LC_ADDRESS: ::libc::c_uint = 589837;
pub const _NL_TELEPHONE_TEL_INT_FMT: ::libc::c_uint = 655360;
pub const _NL_TELEPHONE_TEL_DOM_FMT: ::libc::c_uint = 655361;
pub const _NL_TELEPHONE_INT_SELECT: ::libc::c_uint = 655362;
pub const _NL_TELEPHONE_INT_PREFIX: ::libc::c_uint = 655363;
pub const _NL_TELEPHONE_CODESET: ::libc::c_uint = 655364;
pub const _NL_NUM_LC_TELEPHONE: ::libc::c_uint = 655365;
pub const _NL_MEASUREMENT_MEASUREMENT: ::libc::c_uint = 720896;
pub const _NL_MEASUREMENT_CODESET: ::libc::c_uint = 720897;
pub const _NL_NUM_LC_MEASUREMENT: ::libc::c_uint = 720898;
pub const _NL_IDENTIFICATION_TITLE: ::libc::c_uint = 786432;
pub const _NL_IDENTIFICATION_SOURCE: ::libc::c_uint = 786433;
pub const _NL_IDENTIFICATION_ADDRESS: ::libc::c_uint = 786434;
pub const _NL_IDENTIFICATION_CONTACT: ::libc::c_uint = 786435;
pub const _NL_IDENTIFICATION_EMAIL: ::libc::c_uint = 786436;
pub const _NL_IDENTIFICATION_TEL: ::libc::c_uint = 786437;
pub const _NL_IDENTIFICATION_FAX: ::libc::c_uint = 786438;
pub const _NL_IDENTIFICATION_LANGUAGE: ::libc::c_uint = 786439;
pub const _NL_IDENTIFICATION_TERRITORY: ::libc::c_uint = 786440;
pub const _NL_IDENTIFICATION_AUDIENCE: ::libc::c_uint = 786441;
pub const _NL_IDENTIFICATION_APPLICATION: ::libc::c_uint = 786442;
pub const _NL_IDENTIFICATION_ABBREVIATION: ::libc::c_uint = 786443;
pub const _NL_IDENTIFICATION_REVISION: ::libc::c_uint = 786444;
pub const _NL_IDENTIFICATION_DATE: ::libc::c_uint = 786445;
pub const _NL_IDENTIFICATION_CATEGORY: ::libc::c_uint = 786446;
pub const _NL_IDENTIFICATION_CODESET: ::libc::c_uint = 786447;
pub const _NL_NUM_LC_IDENTIFICATION: ::libc::c_uint = 786448;
pub const _NL_NUM: ::libc::c_uint = 786449;
pub type size_t = ::libc::c_ulong;
pub type iconv_t = *mut ::libc::c_void;
extern "C" {
    pub fn setlocale(__category: ::libc::c_int,
                     __locale: *const ::libc::c_char) -> *mut ::libc::c_char;
    pub fn localeconv() -> *mut Struct_lconv;
    pub fn newlocale(__category_mask: ::libc::c_int,
                     __locale: *const ::libc::c_char, __base: __locale_t)
     -> __locale_t;
    pub fn duplocale(__dataset: __locale_t) -> __locale_t;
    pub fn freelocale(__dataset: __locale_t) -> ();
    pub fn uselocale(__dataset: __locale_t) -> __locale_t;
    pub fn catopen(__cat_name: *const ::libc::c_char, __flag: ::libc::c_int)
     -> nl_catd;
    pub fn catgets(__catalog: nl_catd, __set: ::libc::c_int,
                   __number: ::libc::c_int, __string: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn catclose(__catalog: nl_catd) -> ::libc::c_int;
    pub fn nl_langinfo(__item: nl_item) -> *const ::libc::c_char;
    pub fn nl_langinfo_l(__item: nl_item, __l: __locale_t)
     -> *const ::libc::c_char;
    pub fn iconv_open(__tocode: *const ::libc::c_char,
                      __fromcode: *const ::libc::c_char) -> iconv_t;
    // NOTE: The C decl has __inbuf as *mut *mut, but it actually isn't
    pub fn iconv(__cd: iconv_t, __inbuf: *mut *const ::libc::c_char,
                 __inbytesleft: *mut size_t,
                 __outbuf: *mut *mut ::libc::c_char,
                 __outbytesleft: *mut size_t) -> size_t;
    pub fn iconv_close(__cd: iconv_t) -> ::libc::c_int;
}
