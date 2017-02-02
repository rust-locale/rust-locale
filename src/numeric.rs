//! Numeric category.
//!
//! Handling of local variations in how numbers are written and read.

use std::any::Any;
use std::cmp::max;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

/// Interface for formatting numbers.
///
/// Currently only decimal notation is supported.
///
/// TODO: Parsing.
///
/// # Default implementation
///
/// All methods have default implementation suitable for the invariant locale.
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
    /// # Default implementation
    ///
    /// Uses `DecimalIntFmt`.
    fn format_int_to(&self, n: &fmt::Display, fmt: &str, out: &mut fmt::Formatter) -> fmt::Result
    {
        // FIXME FIXME FIXME FILLME IN
        Err(fmt::Error)
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
    /// # Default implementation
    ///
    /// Uses `DecimalFmt`
    fn format_float_to(&self, n: &ExpDisplay, fmt: &str, out: &mut fmt::Formatter) -> fmt::Result
    {
        // FIXME FIXME FIXME FILLME IN
        Err(fmt::Error)
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

    // TODO: fractional grouping?

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
    digits: &'a str,
    plus: &'a str,
    minus: &'a str,
}

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
            + separator_count(integral, syms.groups) // and separators if requested
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
        let mut iter = self.int.chars().chain(self.frac.chars());

        debug_assert!(syms.digits.len() % 10 == 0);
        let dsize = syms.digits.len() / 10;
        let digit = |oc: Option<char>| {
            let n = oc.map(|c| c.to_digit(10).expect("non-digit in number") as usize).unwrap_or(0);
            &syms.digits[n * dsize .. n * dsize + dsize]
        };

        while min_int > integral {
            try!(out.write_str(&syms.digits[0..dsize]));
            min_int -= 1;
            if is_time_for_separator(min_int, syms.groups) {
                try!(out.write_str(syms.gsep));
            }
        }
        while integral > 0 {
            try!(out.write_str(digit(iter.next())));
            integral -= 1;
            if is_time_for_separator(integral, syms.groups) {
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
            digits: n.decimal_digits(),
            plus: n.plus_sign(),
            minus: n.minus_sign(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt;
    use std::fmt::{Display,Formatter};
    use super::{Numeric,Symbols};
    use super::split_number_string;

    static NOGROUPING: [u8; 0] = [];

    struct TestNumeric;

    impl Numeric for TestNumeric {
        fn decimal_separator(&self) -> &str { "/" }
        fn group_separator(&self) -> &str { "=" }
        fn grouping(&self) -> &[u8] { static G: [u8; 2] = [3, 2]; &G }
        fn decimal_digits(&self) -> &str { "₀₁₂₃₄₅₆₇₈₉" }
        fn plus_sign(&self) -> &str { "p" }
        fn minus_sign(&self) -> &str { "m" }
        fn engineering_exponent_separator(&self) -> &str { "ε" }
        fn common_exponent_separator(&self) -> &str { "c" }
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
}
