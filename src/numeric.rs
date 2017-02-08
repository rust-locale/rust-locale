#![allow(dead_code)]
#![allow(unused_variables)]
//! Numeric category.
//!
//! Handling of local variations in how numbers are written and read.
//!
//! # Supported features
//!
//! The default implementation currently only supports formatting numbers in decimal notation.
//!
//! It is possible to provide an enhanced implementation in a separate crate.
//!
//! # Format specifications
//!
//! The general form of format specifier is:
//!
//! ```ebnf
//! format_spec ::= [[fill]align][sign]["0"][width][.["0"]precision][type]
//! fill        ::= <any character>
//! align       ::= "<" | ">" | "=" | "^"
//! sign        ::= "+" | "-" | " " | "|"
//! width       ::= integer
//! precision   ::= integer
//! type        ::= depends on argument type
//! ```
//!
//! The fill and alignment options are:
//!
//!  - `<`: Align to the logical left.
//!  - `>`: Align to the logical right.
//!  - `=`: Insert padding between sign and magnitude.
//!  - `^`: Centre the field with bias to the logical right.
//!
//! The sign options are:
//!
//!  - `+`: Print `+` for positive and `-` for negative numbers.
//!  - `-`: Print nothing for positive and `-` for negative numbers.
//!  - space: Print non-breakable space for positive and `-` for negative numbers.
//!  - `|`: Do not print sign at all (intended mainly as helper for formatting more complex types).
//!
//! _width_ is positive integer defining _minimum_ width of the field. The field will never be
//! truncated. The width is counted roughly in “symbols”, that is each digit, separator, sign and
//! fill character count as 1.
//!
//! If the _width_ starts with `0`, _padding_ will be done by increasing number of integral digits
//! displayed. The added digits will be zeroes from appropriate digit set and will have separators
//! inserted between them.
//!
//! The _precision_ specifies number of digits after decimal point for `f` format and number of
//! significant digits for the other floating-point formats. It has no effect for integers.
//!
//! If the _precision_ starts with `0`, trailing zeroes _after_ decimal point will be _truncated_.
//!
//! ## Integer format types
//!
//!  - `n`: The default format according to current locale. Can be omitted.
//!  - `c`: Use locale digits, but don't insert group separators.
//!  - `C`: Not localized. Use latin digits and don't insert group separators.
//!
//! ## Floating-point number format types
//!
//!  - `e`: Engineering exponential format. _precision_ indicates the number of significant digits.
//!    Short exponent separator will be used which is `E` in most locales.
//!  - `E`: Common exponential format. _precision_ indicates the number of significant digits. Long
//!    exponent separator will be used which is usually `×10` and in locales using the latin digits
//!    the superscript digits will be used to render the exponent.
//!  - `f`: Fixed point representation. _precision_ indicates number of digits after decimal point.
//!  - `g`: General representation. _precision_ indicates number of significant digits. Unlike C,
//!    the format will _not_ switch to exponential representation for too small or too large
//!    numbers.
//!  - `h`: General representation with exponential fallback. This will switch to engineering
//!    exponential format if insignifiant zeroes would be otherwise needed, that is when the number
//!    of digits before decimal point would be larger than number of significant digits or if the
//!    first significant digit is lower order than tenths.
//!  - `H`: General representation with exponential fallback. Like `h`, but will fall back to
//!    common exponential format like `E`.
//!  - `m`: Mantissa. Just the mantissa part of the exponential format. For when something needs to
//!    be inserted between mantissa and exponent.
//!  - `x`: Exponent. Just the exponent part of the exponential format. For when something needs to
//!    be inserted between mantissa and exponent.
//!  - `X`: Exponent, but in locales with latin digits it will be in superscript.
//!
//! Default floating-point format is `f` and default precision is `.03`.
//!
//! ## Examples
// FIXME FIXME FIXME DOCTESTS!

use ::fmtutil::{Fragment,Render,render_pattern};
use nom;
use std::any::Any;
use std::cmp::max;
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Interface for formatting numbers.
///
/// Currently only decimal notation is supported.
///
/// # Default implementation
///
/// All methods have default implementation suitable for the invariant locale.
///
/// # TODO
///
/// - Parsing.
/// - Rule-based formatting (Roman numerals, words and other non-decimal systems).
/// - Abbreviated formats (“x mi.”, “xy bn.” etc.).
pub trait Numeric : Any + Send + Sync {
    /// Format integer.
    ///
    /// Output will be written to a formatter.
    ///
    /// Note that padding is defined in `fmt`. Currently the padding in `out` is ignored, but in
    /// future it might be _added_ to the one in `fmt`, if appropriate.
    ///
    /// # Parameters
    ///  - `n`: The number to format. It's `Display` should produce string of ASCII digits,
    ///     possibly preceded by `'-'` sign.
    ///  - `fmt`: Format specification.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` on invalid patter or if the underlying formatter returns it during
    /// formatting.
    ///
    // FIXME FIXME FIXME Reference module for pattern format
    fn format_int_to(&self, n: &fmt::Display, fmt: &str, out: &mut fmt::Formatter) -> fmt::Result
    {
        let mut syms: Symbols = From::from(self);
        let mut pat = try!(PythonyPattern::new(fmt, 0));
        debug_assert!(pat.prec == 0, "Non-zero precision for integer requested!");
        pat.prec = 0; // no decimals for integer
        match pat.flag {
            b'\0'|b'n' => (),
            b'c' => {
                syms.groups = &syms.groups[0..0];
            }
            b'C' => {
                syms.groups = &syms.groups[0..0];
                syms.digits = "0123456789";
                syms.plus = "+";
                syms.minus = "-";
            }
            _ => {
                return Err(fmt::Error);
            }
        };
        let num = n.to_string(); // no options for integer
        let (neg, not, exp) = split_number_string(&num);
        if exp != "" || not.exp != 0 {
            debug_assert!(false, "Unexpected exponent part for integer: {:?}", exp);
            return Err(fmt::Error);
        }
        return render_pattern(
            &[
                pat.left_pad(),
                pat.sign(neg, &syms),
                pat.inner_pad(),
                pat.mantissa(&not, &syms, false).to_fragment(),
                pat.right_pad(),
            ],
            pat.width, out);
    }

    /// Format floating point number.
    ///
    /// Output will be written to a formatter.
    ///
    /// Note that padding is defined in `fmt`. Currently the padding in `out` is ignored, but in
    /// future it might be _added_ to the one in `fmt`, if appropriate.
    ///
    /// # Parameters
    ///  - `n`: The number to format. It's `Display` and `LowerExp` should behave like those of
    ///     built-in floating types.
    ///  - `fmt`: Format specification. See `DecimalFloatFmt` for format description.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    /// Returns `fmt::Error` on invalid patter or if the underlying formatter returns it during
    /// formatting.
    ///
    fn format_float_to(&self, n: &ExpDisplay, fmt: &str, out: &mut fmt::Formatter) -> fmt::Result
    {
        let mut syms: Symbols = From::from(self);
        let mut pat = try!(PythonyPattern::new(fmt, -3));

        let (use_sig, num) = match pat.flag {
            // number of decimal digits => use Display
            b'f'|b'\0' => (false, format!("{:.*}", pat.prec as usize, n)),
            // number of significant digits => use LowerExp
            _ => (true, format!("{:.*e}", max(pat.prec - 1, 0) as usize, n)),
        };

        if num == "inf" || num == "-inf" || num == "NaN" { // handle infinity and NaN
            pat.zero = false; // can't stretch infinity and NaN symbols, so switch to normal padding
            if pat.flag == b'x' || pat.flag == b'X' {
                // infinity and NaN don't have exponent, but pad it out if requested!
                return render_pattern(
                    &[
                        pat.left_pad(),
                        pat.inner_pad(),
                        pat.right_pad(),
                    ],
                    pat.width, out);
            } else {
                return render_pattern(
                    &[
                        pat.left_pad(),
                        pat.sign(num.starts_with("-"), &syms),
                        pat.inner_pad(),
                        if num == "NaN" {
                            Fragment::Literal(self.not_a_number().width() as i32, self.not_a_number())
                        } else {
                            Fragment::Literal(self.infinity().width() as i32, self.infinity())
                        },
                        pat.right_pad(),
                    ],
                    pat.width, out);
            }
        }

        let (mut neg, mut not, mut exp) = split_number_string(&num);

        // Decide whether exponential format will be used and adjust not if yes:
        let use_exp = match pat.flag {
            b'e'|b'E' => true,
            b'f'|b'g'|b'\0' => false,
            b'h'|b'H' => not.exp >= pat.prec as i32 || not.exp < -1,
            b'm' => { not.exp = 0; false } // mutate not as if exponent, but otherwise don't use it
            b'x'|b'X' => {
                // XXX: I wanted to check no precision is provided here, but since the default is
                // non-0, I can't.
                pat.prec = 0; // exponent is integral, so no decimals
                let (eneg, enot, eexp) = split_number_string(exp);
                if eexp != "" || enot.exp != 0 { return Err(::std::fmt::Error); }
                if pat.flag == b'X' && syms.digits.starts_with('0') {
                    syms = SUPERSCRIPT_SYMBOLS;
                } else {
                    syms.groups = &syms.groups[0..0]; // no grouping in exponent
                }
                neg = eneg; not = enot; exp = eexp;
                false
            },
            _ => return Err(::std::fmt::Error),
        };
        let edata = if use_exp {
            not.exp = 0; // see Notation.format_to doc
            assert!(!exp.is_empty()); // pat.flag is not 'f', so we should have used LowerExp
            let (eneg, enot, eexp) = split_number_string(exp);
            assert_eq!(eexp, ""); // ok, it didn't contain 'e'; split_number_string wouldn't accept that
            let mut epat = PythonyPattern::default();
            epat.sign = b'+';
            let mut esyms;
            if (pat.flag == b'E' || pat.flag == b'H') && syms.digits.starts_with('0') {
                esyms = SUPERSCRIPT_SYMBOLS;
            } else {
                esyms = syms;
                esyms.groups = &syms.groups[0..0]; // no grouping in exponent
            }
            Some((eneg, enot, epat, esyms))
        } else { None };
        let mantissa = pat.mantissa(&not, &syms, use_sig);
        let emaintissa = edata.as_ref().map(|t| t.2.mantissa(&t.1, &t.3, false));
        let frags = [
            pat.left_pad(),
            pat.sign(neg, &syms),
            pat.inner_pad(),
            mantissa.to_fragment(),
            if use_exp {
                if pat.flag == b'E' || pat.flag == b'H' {
                    Fragment::Literal(1, self.common_exponent_separator())
                } else {
                    Fragment::Literal(1, self.engineering_exponent_separator())
                }
            } else {
                Fragment::Blank
            },
            edata.as_ref().map(|t| t.2.sign(t.0, &t.3)).unwrap_or(Fragment::Blank),
            emaintissa.as_ref().map(|m| m.to_fragment()).unwrap_or(Fragment::Blank),
            pat.right_pad(),
        ];
        return render_pattern(&frags, pat.width, out);
    }

    /// Decimal separator
    fn decimal_separator(&self) -> &str { "." }

    /// Thousands separator
    ///
    /// Empty string if not used.
    fn group_separator(&self) -> &str { "" }

    /// Grouping
    ///
    /// Primary group, that is the one including units, is first. The last group is used for all
    /// larger orders. A group with size 0 means no further group separators should be inserted.
    fn grouping(&self) -> &[u8] { &[] }

    /// Mininimum grouping digits
    ///
    /// Avoid creating groups of less than this size. E.g. if 2, format 10,000, but 1000, not
    /// 1,000.
    fn min_grouping_digits(&self) -> i32 { 1 }

    // TODO: Fractional grouping? It does not appear in CLDR, ever.

    /// String of decimal digits
    ///
    /// String of consecutive decimal digits 0 to 9. The digits must come from the same digit set.
    fn decimal_digits(&self) -> &str { "0123456789" }

    /// Plus sign
    ///
    /// In some locales, this includes left-to-right and other marks. However, the calling code
    /// expects it to always only return one glyph.
    fn plus_sign(&self) -> &str { "+" }

    /// Minus sign
    ///
    /// In some locales, this includes left-to-right and other marks. However, the calling code
    /// expects it to always only return one glyph.
    fn minus_sign(&self) -> &str { "-" }

    /// Engineering exponent separator
    ///
    /// The symbol or string used to introduce decimal exponent in engineering notation. Defaults
    /// to ‘e’.
    fn engineering_exponent_separator(&self) -> &str { "e" }

    /// Common exponent separator
    ///
    /// The symbol sequence used to introduce decimal exponent in “common” (e.g. mathematical or
    /// physical) notation. Defaults to ‘×10’. In locales using decimal digits, the exponent after
    /// this uses superscript digits.
    fn common_exponent_separator(&self) -> &str { "\u{d7}10" }

    /// Infinity
    ///
    /// The symbol used for infinity.
    fn infinity(&self) -> &str { "∞" }

    /// Not-a-number
    ///
    /// The symbol used for not-a-number.
    fn not_a_number(&self) -> &str { "NaN" }
}

/// Auxiliary trait for making trait objects from floating point numbers.
///
/// For formatting floating point numbers, we need both `std::fmt::Display` and
/// `std::fmt::LowerExp`. And because the facet is used as trait object, it can't have generic
/// methods and must thus get its arguments as trait objects too. So here we provide auxiliary
/// trait combining the two, because `fmt::Display + fmt::LowerExp` is not a valid type expression.
pub trait ExpDisplay : fmt::Display + fmt::LowerExp { }
impl<T: fmt::Display + fmt::LowerExp + ?Sized> ExpDisplay for T { }

// ------ number formatting implementation (private) ---------------------------------------------

/// Helper struct for processing mantissa of a stringified number.
#[derive(Copy, Clone, Debug)]
struct Notation<'a> {
    pub int: &'a str,
    pub frac: &'a str,
    pub exp: i32,
}

/// Helper struct holding symbols and other parameters for use by Notation and *Pattern.
///
/// The Notation and Pattern need the symbols—digits, decimal and group separator and signs—to
/// compose the output. However passing the `&Numeric` does not work well because:
///
/// 1. in some cases, we pass different symbol sets and
/// 2. provided trait method can't cast self to &Numeric.
///
/// So we extract the symbols instead.
#[derive(Copy, Clone, Debug)]
struct Symbols<'a> {
    dsep: &'a str,
    gsep: &'a str,
    groups: &'a [u8],
    mingrp: i32,
    digits: &'a str,
    plus: &'a str,
    minus: &'a str,
}

static NOGROUPING: [u8; 0] = [];
static SUPERSCRIPT_SYMBOLS: Symbols<'static> = Symbols{
    dsep: "",
    gsep: "",
    groups: &NOGROUPING,
    mingrp: 1,
    digits: "⁰¹²³⁴⁵⁶⁷⁸⁹",
    plus: "⁺",
    minus: "⁻",
};

/// Auxiliary function for walking over the groups definition
///
/// Returns rest of the groups array, but keeps returning the last group if there is only one left.
fn next_groups(g: &[u8]) -> &[u8] {
    if g.len() > 1 {
        &g[1..]
    } else {
        g
    }
}

impl<'a> Notation<'a> {
    /// Returns number of _chars_ the result of `format_to` will have.
    ///
    /// Fortunately numbers don't use anything that would need combining symbols, so number of
    /// characters equals number of glyphs.
    pub fn width(&self, min_int: i32, min_frac: i32, min_sig: i32, syms: &Symbols) -> i32 {
        fn separator_count(digits: i32, groups: &[u8]) -> i32 {
            if groups.is_empty() || groups[0] == 0 || digits <= groups[0] as i32 {
                return 0;
            } else {
                return 1 + separator_count(digits - groups[0] as i32, next_groups(groups));
            }
        }

        debug_assert!(min_int >= 0);
        debug_assert!(min_frac >= 0);
        let integral = max(self.int.len() as i32 + self.exp, min_int);
        let fractional = max(self.frac.len() as i32 - self.exp,
                             max(min_sig - self.int.len() as i32 - self.exp,
                                 min_frac));
        // note: since min_int and min_frac are unsigned, integral and fractional are actually
        // nonnegative here.
        return integral // integral digits will be printed
            + separator_count(integral - max(syms.mingrp - 1, 0), syms.groups) // and separators if requested
            + if fractional > 0 {
                fractional + 1 // if any fractional, don't forget decimal separator
            } else { 0 };
    }

    /// Applies locale-dependent transformation to number, use specified digits.
    ///
    /// Number will be padded with zeroes from either side as required if the precision is not
    /// sufficient, but this function will _not_ truncate it. That must have already been done when
    /// `Display`ing it to get correct rounding.
    ///
    /// The decimal point will be placed according to the exponent recorded in invocant as
    /// appropriate for number is _plain_ format. For printing in exponential format the exponent
    /// needs to be zeroed out.
    ///
    /// The logic for adding and removing zeroes assumes the exponent is either as read from the
    /// string or zeroed and that any number with non-zero exponent will be normalized in the usual
    /// way to one integral digit. The significant digit logic assumes there are no insignificant
    /// zeroes on the left (satisfied since exponential format is used for rounding to given number
    /// of significant digits)
    ///
    /// Arguments are:
    ///
    ///  - `min_int`: Number will be padded from (logical) left with zeroes to at least this number
    ///    of integral digits.
    ///  - `min_frac`: Number will be padded from (logical) right with zeroes to at least this
    ///    number of fractional digits.
    ///  - `min_sig`: Number will be padded from (logical) right with zeroes to at least this
    ///    number of significant digits.
    ///  - `syms`: Symbols to use for the formatting.
    pub fn format_to(&self, mut min_int: i32, min_frac: i32, min_sig: i32, syms: &Symbols,
                     out: &mut Formatter) -> fmt::Result {
        fn is_time_for_separator(digits_left: i32, groups: &[u8]) -> bool {
            if groups.is_empty() || groups[0] == 0 || digits_left < groups[0] as i32 {
                return false;
            } else if digits_left == groups[0] as i32 {
                return true;
            } else {
                return is_time_for_separator(digits_left - groups[0] as i32, next_groups(groups));
            }
        }

        debug_assert!(min_int >= 0);
        debug_assert!(min_frac >= 0);
        let mut integral = self.int.len() as i32 + self.exp;
        let mut fractional = max(self.frac.len() as i32 - self.exp,
                                 max(min_sig - integral,
                                     min_frac));
        let mut dontsep = syms.mingrp;
        let mut iter = self.int.chars().chain(self.frac.chars());

        let dsize = if syms.digits.len() % 10 == 0 { syms.digits.len() / 10 } else { 0 };
        let digit = |oc: Option<char>| {
            let n = oc.map(|c| c.to_digit(10).expect("non-digit in number") as usize).unwrap_or(0);
            if dsize > 0 {
                &syms.digits[n * dsize .. n * dsize + dsize]
            } else {
                syms.digits.split("").nth(n + 1).unwrap_or("\u{fffd}")
            }
        };

        while min_int > integral {
            try!(out.write_str(digit(None)));
            min_int -= 1;
            dontsep -= 1;
            if dontsep <= 0 && is_time_for_separator(min_int, syms.groups) {
                try!(out.write_str(syms.gsep));
            }
        }
        while integral > 0 {
            try!(out.write_str(digit(iter.next())));
            integral -= 1;
            dontsep -= 1;
            if dontsep <= 0 && is_time_for_separator(integral, syms.groups) {
                try!(out.write_str(syms.gsep));
            }
        }
        if fractional > 0 {
            try!(out.write_str(syms.dsep));
            while integral < 0 {
                try!(out.write_str(digit(iter.next())));
                integral += 1; fractional -= 1;
            }
            while fractional > 0 {
                try!(out.write_str(digit(iter.next())));
                fractional -= 1;
            }
        }
        return Ok(());
    }
}

/// Parse number to sign, mantissa and exponent
///
/// Number must be in the basic integral, floating point or (lower) exponential format, i.e. it
/// must match regular expression `-?[0-9]+(\.[0-9]+)?(e-?[0-9]+)?` and must not contain any
/// unnecessary leading zeroes.
///
/// The number must be already rounded to desired precision, since correct rounding is no longer
/// possible on string representation.
///
/// # Returns
///
/// Tripple of
///  - negative
///  - notation as mantissa split to integral and fractional part and value of exponent
///  - exponent string (empty if not present)
///
/// The fractional part has trailing zeroes removed. They will be added back by `Notation.format_to` to
/// fill number of fractional and/or significant digits required.
fn split_number_string<'a>(number: &'a str) -> (bool, Notation<'a>, &'a str) {
    let (negative, mag_s) = if number.starts_with('-') {
            (true, &number[1..])
        } else {
            (false, number)
        };
    let (man_s, exp_s, exponent) = if let Some(i) = mag_s.find('e') {
            (&mag_s[..i], &mag_s[i+1..], i32::from_str(&mag_s[i+1..]).unwrap())
        } else {
            (mag_s, "", 0i32)
        };
    let (int_s, frac_s) = if let Some(i) = man_s.find('.') {
            (&man_s[..i], man_s[i+1..].trim_right_matches('0'))
        } else {
            (man_s, "")
        };
    return (negative,
            Notation{
                int: int_s,
                frac: frac_s,
                exp: exponent,
            },
            exp_s);
}

impl<'a, F: Numeric + ?Sized> From<&'a F> for Symbols<'a> {
    fn from(n: &'a F) -> Self {
        Symbols {
            dsep: n.decimal_separator(),
            gsep: n.group_separator(),
            groups: n.grouping(),
            mingrp: n.min_grouping_digits(),
            digits: n.decimal_digits(),
            plus: n.plus_sign(),
            minus: n.minus_sign(),
        }
    }
}

// ------ pattern implementations (private) ------------------------------------------------------

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
struct PythonyPattern<'a> {
    fill: &'a str,
    align: u8,
    sign: u8,
    zero: bool,
    width: i32,
    trunc: bool,
    prec: i32,
    flag: u8,
}

impl<'a> Default for PythonyPattern<'a> {
    fn default() -> Self {
        PythonyPattern {
            fill: " ",
            align: b'>',
            sign: b'-',
            zero: false,
            width: 0,
            trunc: true,
            prec: 0,
            flag: b'\0',
        }
    }
}

#[derive(Copy,Clone,Debug)]
struct Mantissa<'a> {
    min_int: i32,
    min_frac: i32,
    min_sig: i32,
    zero: bool,
    notation: &'a Notation<'a>,
    syms: &'a Symbols<'a>,
}

impl<'a> PythonyPattern<'a> {
    fn new(fmt: &'a str, prec: i32) -> Result<Self, fmt::Error> {
        let mut def = PythonyPattern::default();
        if prec < 0 {
            def.trunc = true;
            def.prec = -prec;
        } else {
            def.prec = prec;
        }
        pythony_pattern(fmt, def)
    }

    fn pad_to(&self, n: i32, out: &mut Formatter) -> fmt::Result {
        for _ in 0..n { try!(out.write_fmt(format_args!("{}", self.fill))) }
        return Ok(());
    }

    fn sign(&self, negative: bool, syms: &'a Symbols) -> Fragment<'a>
    {
        let sign = match self.sign {
            b'+' => if negative { syms.minus } else { syms.plus },
            b'-' => if negative { syms.minus } else { "" },
            b' ' => if negative { syms.minus } else { self.fill },
            b'|' => "",
            _ => unreachable!(),
        };
        Fragment::Literal(if sign.is_empty() { 0 } else { 1 }, sign)
    }

    fn mantissa(&self, notation: &'a Notation, syms: &'a Symbols, use_significant: bool) -> Mantissa<'a> {
        Mantissa {
            min_int: 1, // may allow more in different kind of pattern one day
            min_frac: if self.trunc { 0 } else if use_significant { 0 } else { self.prec },
            min_sig: if self.trunc { 0 } else if use_significant { self.prec } else { 0 },
            zero: self.zero,
            notation: notation,
            syms: syms,
        }
    }

    fn left_pad(&self) -> Fragment<'a> {
        if (self.align == b'>' || self.align == b'=') && !self.zero {
            Fragment::Padding(self.fill)
        } else {
            Fragment::Blank
        }
    }

    fn inner_pad(&self) -> Fragment<'a> {
        if self.align == b'^' && !self.zero {
            Fragment::Padding(self.fill)
        } else {
            Fragment::Blank
        }
    }

    fn right_pad(&self) -> Fragment<'a> {
        if (self.align == b'<' || self.align == b'=') && !self.zero {
            Fragment::Padding(self.fill)
        } else {
            Fragment::Blank
        }
    }
}

impl<'a> Mantissa<'a> {
    fn width(&self) -> i32 {
        self.xwidth(self.min_int)
    }

    fn xwidth(&self, min_int: i32) -> i32 {
        self.notation.width(min_int, self.min_frac, self.min_sig, self.syms)
    }

    fn to_fragment(&'a self) -> Fragment<'a> {
        if self.zero {
            Fragment::Growing(self.width(), self)
        } else {
            Fragment::Displayed(self.width(), self)
        }
    }
}

impl<'a> Render for Mantissa<'a> {
    fn fmt(&self, width: i32, original: i32, out: &mut Formatter) -> fmt::Result {
        let mut min_int = self.min_int;
        if self.zero && width > original {
            while self.xwidth(min_int) < width {
                min_int += 1;
            }
        }
        self.notation.format_to(min_int, self.min_frac, self.min_sig, self.syms, out)
    }
}

impl<'a> fmt::Display for Mantissa<'a> {
    fn fmt(&self, out: &mut Formatter) -> fmt::Result {
        Render::fmt(self, -1, -1, out)
    }
}

// ------ pattern grammars (private) -------------------------------------------------------------

// Note: nom is relatively simple and obvious and works well on byte arrays, but in its current
// state it is basically unusable for strings. So the function works on [u8] and unsafe-casts
// back to str (it was a str and the parser will never accept an incomplete utf-8 character).
fn grapheme(input: &[u8]) -> nom::IResult<&[u8], &str> {
    let as_str = unsafe { str::from_utf8_unchecked(input) };
    if let Some(s) = as_str.graphemes(true).next() {
        nom::IResult::Done(&input[s.len()..], s)
    } else {
        nom::IResult::Error(nom::ErrorKind::Complete)
    }
}

named!(
    integer<i32>,
    map_res!(
        map_res!(
            nom::digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

fn pythony_pattern<'a>(input: &'a str, mut pat: PythonyPattern<'a>) -> Result<PythonyPattern<'a>, fmt::Error> {
    let res: nom::IResult<&[u8], ()> = do_parse!(
        input.as_bytes(),
        // .?[<>=^] // < left, > right, = internal, ^ centre
        alignment: opt!(
            alt!(
                // FIXME: should be actually anyglyph!
                  complete!(tuple!(call!(grapheme), one_of!("<>=^")))
                | complete!(map!(one_of!("<>=^"), |s| (" ", s)))
            )
        ) >>
        // [+- |] // + always, - negative, ' ' pad, | never
        sign: opt!(complete!(one_of!("+- |"))) >>
        // 0 // fill by integral digits
        zero: opt!(complete!(char!('0'))) >>
        // [1-9][0-9]* // width to fill (sign, digit and each separator always count as 1)
        width: opt!(complete!(integer)) >>
        // \.0?[1-9][0-9]*|\.0 // fractional or significant digits, 0 prefix means truncate trailing zeroes
        prec: opt!(
            preceded!(
                complete!(char!('.')),
                alt!(
                      complete!(map!(tuple!(char!('0'), integer), |(_, i)| (true, i)))
                    | complete!(map!(integer, |i| (false, i)))
                )
            )
        ) >>
        // [ncCeEfghHmxX] //
        flag: opt!(complete!(one_of!("ncCeEfghHmxX"))) >>
        // process
        ({
            if alignment.is_some() {
                pat.fill = alignment.unwrap().0;
                pat.align = alignment.unwrap().1 as u8; // nom currently returns char even though it really has u8
            }
            if sign.is_some() {
                pat.sign = sign.unwrap() as u8; // nom currently returns char even though it really has u8
            }
            if zero.is_some() {
                pat.zero = true;
            }
            if width.is_some() {
                pat.width = width.unwrap();
            }
            if prec.is_some() {
                pat.trunc = prec.unwrap().0;
                pat.prec = prec.unwrap().1;
            }
            if flag.is_some() {
                pat.flag = flag.unwrap() as u8; // nom currently returns char even though it really has u8
            }
        })
    );
    match res {
        nom::IResult::Done(s, ()) if s.is_empty() => Ok(pat),
        nom::IResult::Done(s, ()) => {
            debug_assert!(true, "Unrecognised characters in pattern: {}", unsafe { str::from_utf8_unchecked(s) });
            Err(fmt::Error)
        },
        nom::IResult::Error(e) => {
            debug_assert!(true, "Error parsing pattern: {}", e);
            Err(fmt::Error)
        },
        nom::IResult::Incomplete(_) => panic!("Internal error in pattern parser"),
    }
}

#[cfg(test)]
mod test {
    use std::fmt;
    use std::fmt::{Display,Formatter};
    use super::{Numeric,Symbols};
    use super::split_number_string;
    use super::{pythony_pattern,PythonyPattern};

    static NOGROUPING: [u8; 0] = [];

    struct TestNumeric;
    struct ExpTestNumeric;

    impl Numeric for TestNumeric {
        fn decimal_separator(&self) -> &str { "/" }
        fn group_separator(&self) -> &str { "=" }
        fn grouping(&self) -> &[u8] { static G: [u8; 2] = [3, 2]; &G }
        fn min_grouping_digits(&self) -> i32 { 1 }
        fn decimal_digits(&self) -> &str { "₀₁₂₃₄₅₆₇₈₉" }
        fn plus_sign(&self) -> &str { "p" }
        fn minus_sign(&self) -> &str { "m" }
        fn engineering_exponent_separator(&self) -> &str { "ε" }
        fn common_exponent_separator(&self) -> &str { "c" }
        fn infinity(&self) -> &str { "infty" }
        fn not_a_number(&self) -> &str { "N/N" }
    }

    // superscript digits are only used in exponent when base digits are latin, so another version
    // with those
    impl Numeric for ExpTestNumeric {
        fn decimal_separator(&self) -> &str { "/" }
        fn group_separator(&self) -> &str { "=" }
        fn grouping(&self) -> &[u8] { static G: [u8; 2] = [3, 2]; &G }
        fn min_grouping_digits(&self) -> i32 { 3 }
        fn decimal_digits(&self) -> &str { "0123456789" }
        fn plus_sign(&self) -> &str { "p" }
        fn minus_sign(&self) -> &str { "m" }
        fn engineering_exponent_separator(&self) -> &str { "ε" }
        fn common_exponent_separator(&self) -> &str { "×₁₀" }
        fn infinity(&self) -> &str { "infty" }
        fn not_a_number(&self) -> &str { "N/N" }
    }

    struct Disp<F>(F) where F: Fn(&mut Formatter) -> fmt::Result;

    impl<F> Display for Disp<F> where F: Fn(&mut Formatter) -> fmt::Result {
        fn fmt(&self, out: &mut Formatter) -> fmt::Result { self.0(out) }
    }

    fn disp<F>(f: F) -> String where F: Fn(&mut Formatter) -> fmt::Result {
        Disp(f).to_string()
    }

    #[test]
    fn split_and_format1() {
        let (sign, mag, exp) = split_number_string("2.110e2");
        let n = TestNumeric;
        let mut syms: Symbols = From::from(&n);
        syms.groups = &NOGROUPING;
        assert_eq!(false, sign);
        assert_eq!(3, mag.width(1, 0, 0, &syms));
        assert_eq!("₂₁₁", disp(|out| mag.format_to(1, 0, 0, &syms, out)));
        assert_eq!(5, mag.width(5, 0, 0, &syms));
        assert_eq!("₀₀₂₁₁", disp(|out| mag.format_to(5, 0, 0, &syms, out)));
        assert_eq!(6, mag.width(0, 0, 5, &syms));
        assert_eq!("₂₁₁/₀₀", disp(|out| mag.format_to(0, 0, 5, &syms, out)));
        assert_eq!(2, mag.exp);
        assert_eq!("2", exp);
    }

    #[test]
    fn split_and_format2() {
        let (sign, mag, exp) = split_number_string("-112268431.2");
        let n = TestNumeric;
        let syms = From::from(&n);
        assert_eq!(true, sign);
        assert_eq!(17, mag.width(1, 4, 0, &syms));
        assert_eq!("₁₁=₂₂=₆₈=₄₃₁/₂₀₀₀", disp(|out| mag.format_to(1, 4, 0, &syms, out)));
        assert_eq!(14, mag.width(1, 0, 0, &syms));
        assert_eq!("₁₁=₂₂=₆₈=₄₃₁/₂", disp(|out| mag.format_to(1, 0, 0, &syms, out)));
        assert_eq!(0, mag.exp);
        assert_eq!("", exp);
    }

    #[test]
    fn pythony_patterns() {
        fn parse(s: &str) -> PythonyPattern {
            pythony_pattern(s, PythonyPattern::default()).unwrap()
        }

        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'-', zero: false, width: 0,
                trunc: true, prec: 0, flag: b'\0'},
            parse(""));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'<', sign: b'-', zero: false, width: 0,
                trunc: true, prec: 0, flag: b'c'},
            parse("<c"));
        assert_eq!(
            PythonyPattern{fill: "<", align: b'<', sign: b'-', zero: false, width: 0,
                trunc: true, prec: 0, flag: b'c'},
            parse("<<c"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'|', zero: false, width: 0,
                trunc: true, prec: 0, flag: b'\0'},
            parse("|"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'-', zero: true, width: 0,
                trunc: true, prec: 0, flag: b'\0'},
            parse("0"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'-', zero: false, width: 33,
                trunc: true, prec: 0, flag: b'\0'},
            parse("33"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'-', zero: true, width: 5,
                trunc: true, prec: 0, flag: b'\0'},
            parse("05"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'-', zero: false, width: 0,
                trunc: false, prec: 0, flag: b'\0'},
            parse(".0"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'-', zero: false, width: 0,
                trunc: false, prec: 12, flag: b'\0'},
            parse(".12"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'-', zero: false, width: 0,
                trunc: true, prec: 6, flag: b'\0'},
            parse(".06"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'>', sign: b'-', zero: false, width: 0,
                trunc: true, prec: 0, flag: b'g'},
            parse("g"));
        assert_eq!(
            PythonyPattern{fill: " ", align: b'=', sign: b'+', zero: false, width: 5,
                trunc: false, prec: 2, flag: b'e'},
            parse("=+5.2e"));
        assert_eq!(
            PythonyPattern{fill: "-", align: b'^', sign: b'-', zero: true, width: 8,
                trunc: true, prec: 4, flag: b'f'},
            parse("-^-08.04f"));
        assert_eq!(
            PythonyPattern{fill: "f\u{030c}", align: b'^', sign: b'-', zero: false, width: 1,
                trunc: false, prec: 1, flag: b'\0'},
            parse("f\u{030c}^1.1"));
        assert!(pythony_pattern("fň", PythonyPattern::default()).is_err());
        assert!(pythony_pattern("ň", PythonyPattern::default()).is_err());
        assert!(pythony_pattern("11.12.13", PythonyPattern::default()).is_err());
    }

    #[test]
    fn format_int_n() {
        let n: &Numeric = &TestNumeric;
        let e: &Numeric = &ExpTestNumeric;
        assert_eq!("₂₁₁", disp(|out| n.format_int_to(&211, "", out)));
        assert_eq!("₂₁=₁₀₀", disp(|out| n.format_int_to(&21100, "", out)));
        assert_eq!("   ₂₁₁", disp(|out| n.format_int_to(&211, "6", out)));
        assert_eq!("₂₁₁   ", disp(|out| n.format_int_to(&211, "<6n", out)));
        assert_eq!(" ₂₁₁  ", disp(|out| n.format_int_to(&211, "=6", out)));
        assert_eq!("   ₂₁₁", disp(|out| n.format_int_to(&211, ">6n", out)));
        assert_eq!("p  ₂₁₁", disp(|out| n.format_int_to(&211, "^+6", out)));
        assert_eq!("₀₀=₂₁₁", disp(|out| n.format_int_to(&211, "06", out)));
        assert_eq!("m₀=₂₁₁", disp(|out| n.format_int_to(&-211, "06n", out)));
        assert_eq!("x₀=₂₁₁", disp(|out| n.format_int_to(&211, "x< 06", out)));
        assert_eq!("₀₀₀₂₁₁", disp(|out| n.format_int_to(&211, "06c", out)));
        assert_eq!("000211", disp(|out| n.format_int_to(&211, "06C", out)));
        assert_eq!("   21100", disp(|out| e.format_int_to(&21100, "8", out)));
        assert_eq!("0021=100", disp(|out| e.format_int_to(&21100, "08", out)));
        assert_eq!(" 211=000", disp(|out| e.format_int_to(&211000, "8", out)));
    }

    #[test]
    fn format_float_n() {
        let n: &Numeric = &TestNumeric;
        let e: &Numeric = &ExpTestNumeric;
        assert_eq!("₂/₁₁₂", disp(|out| n.format_float_to(&2.11211, "", out)));
        assert_eq!("₂=₁₁₂/₁₁", disp(|out| n.format_float_to(&2112.11, "", out)));
        // e
        assert_eq!("₂/₁₁εp₂", disp(|out| n.format_float_to(&211.211, "e", out)));
        assert_eq!("₂/₁₁₂εm₂", disp(|out| n.format_float_to(&0.0211211, ".4e", out)));
        // E
        assert_eq!("  ₂/₁₁cp₂", disp(|out| n.format_float_to(&211.211, "9.3E", out)));
        assert_eq!("  2/11×₁₀⁺²", disp(|out| e.format_float_to(&211.211, "9.3E", out)));
        // f
        assert_eq!("  ₂/₁₁  ", disp(|out| n.format_float_to(&2.11211, "=8.2f", out)));
        assert_eq!("₂=₁₁₂/₁₁", disp(|out| n.format_float_to(&2112.11, "=8.04f", out)));
        // g
        assert_eq!("₂=₁₁₀", disp(|out| n.format_float_to(&2112.11, "g", out)));
        assert_eq!("₂/₁₁", disp(|out| n.format_float_to(&2.11211, "g", out)));
        assert_eq!("₂/₁₁₀₀", disp(|out| n.format_float_to(&2.11, ".5g", out)));
        assert_eq!("₂/₁₁", disp(|out| n.format_float_to(&2.11, ".05g", out)));
        assert_eq!("p~~₂=₁₁₂/₁", disp(|out| n.format_float_to(&2112.11, "~^+10.5g", out)));
        assert_eq!("p~~~₂/₁₁₂₁", disp(|out| n.format_float_to(&2.11211, "~^+10.5g", out)));
        // h
        assert_eq!("2110", disp(|out| e.format_float_to(&2110.0, ".4h", out)));
        assert_eq!("2/110εp4", disp(|out| e.format_float_to(&21100.0, ".4h", out)));
        assert_eq!("₀/₂₁₁₀", disp(|out| n.format_float_to(&0.2110, ".4h", out)));
        assert_eq!("₂/₁₁₀εm₂", disp(|out| n.format_float_to(&0.0211, ".4h", out)));
        // H
        assert_eq!("2110", disp(|out| e.format_float_to(&2110.0, ".4H", out)));
        assert_eq!("2/110×₁₀⁺⁴", disp(|out| e.format_float_to(&21100.0, ".4H", out)));
        assert_eq!("₀/₂₁₁₀", disp(|out| n.format_float_to(&0.2110, ".4H", out)));
        assert_eq!("₂/₁₁₀cm₂", disp(|out| n.format_float_to(&0.0211, ".4H", out)));
        // m
        assert_eq!("2/11", disp(|out| e.format_float_to(&0.0211211, "m", out)));
        // x
        assert_eq!("m₂", disp(|out| n.format_float_to(&0.0211211, "x", out)));
        assert_eq!("m2", disp(|out| e.format_float_to(&0.0211211, "x", out)));
        // X
        assert_eq!("m₂", disp(|out| n.format_float_to(&0.0211211, "X", out)));
        assert_eq!("⁻²", disp(|out| e.format_float_to(&0.0211211, "X", out)));
        // infinity
        assert_eq!("infty   ", disp(|out| n.format_float_to(&::std::f64::INFINITY, "<8", out)));
        assert_eq!("m  infty", disp(|out| n.format_float_to(&::std::f64::NEG_INFINITY, "^-08", out)));
        assert_eq!("        ", disp(|out| n.format_float_to(&::std::f64::INFINITY, "08x", out)));
        // NaN
        assert_eq!("N/N", disp(|out| n.format_float_to(&::std::f64::NAN, "g", out)));
        assert_eq!("", disp(|out| n.format_float_to(&::std::f64::NAN, "X", out)));
    }
}
