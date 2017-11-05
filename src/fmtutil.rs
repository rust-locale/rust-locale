//! Formatting utilities.
//!
//! This defines helpers that are reused for formatting various kinds of patterns.
//!
//! Most of this module is visible crate-wide.

use ::std::fmt;
use ::std::fmt::{Display,Formatter};
use ::std::iter::IntoIterator;

/// A fragment of output format.
///
/// To format something, we need to collect all elements, count their size, distribute any
/// left-over space to padding and only then we can actually write it to the Formatter. To unify
/// that logic a bit, we have an array of `Fragment`s that specify the parts and common padding
/// calculation is done over them.
#[allow(dead_code)] // TODO: Remove when all formatting styles are complete
pub enum Fragment<'a> {
    /// An empty element. Many patterns have fixed order of element, but some of them are optional.
    /// So this can be used in place of those that are not needed in that particular case so that
    /// we can still use a fixed array for these patterns.
    Blank,
    /// A single codepoint as char. Length is somewhat obviously 1.
    Codepoint(char),
    /// A literal string. The first parameter is length, in graphemes, the second is the actual
    /// string that will be printed exactly once.
    Literal(i32, &'a str),
    /// A padding grapheme. It is assumed the length is 1 grapheme and it will be printed as many
    /// times as needed to pad the output. Padding will be distributed evenly among the Padding
    /// fragments. Rounding tends to be down on the left and up on the right.
    Padding(&'a str),
    /// A complex fragment represented by object. The first argument is the length in graphemes,
    /// the second is an object that can be rendered via std::fmt::Display.
    Displayed(i32, &'a Display),
    /// Like Displayed, but has internal padding.
    Growing(i32, &'a Render),
    /// Like Displayed, but can truncate itself.
    Shrinking(i32, &'a Render),
}

#[allow(dead_code)] // TODO: I thought I might need it, but I don't currently see where
impl<'a> Fragment<'a> {
    pub fn len(&self) -> i32 {
        match self {
            &Fragment::Blank => 0,
            &Fragment::Codepoint(_) => 1,
            &Fragment::Literal(n, _) => n,
            &Fragment::Padding(_) => 0,
            &Fragment::Displayed(n, _) => n,
            &Fragment::Growing(n, _) => n,
            &Fragment::Shrinking(n, _) => n,
        }
    }
}

/// Display to specified width subject to ability to pad or shrink itself.
///
/// To avoid creating a nested formatter when rendering growable (e.g. numbers can be padded with
/// 0s from the right) or shrinkable (e.g. strings can be truncated) elements, we have a trait that
/// works similar to Display, but has extra parameters.
pub trait Render {
    /// Format with length adjustment.
    ///
    /// # Parameters
    ///
    ///  - `width`: Desired width.
    ///  - `original`: Original width from Fragment::Growing or Fragment::Shrinking. If given as
    ///    -1, it is not known and should be calculated internally.
    ///  - `out`: The output formatter.
    fn fmt(&self, width: i32, original: i32, out: &mut Formatter) -> fmt::Result;
}

// FIXME: I would like to
// impl<'a, Collection> Render for Collection where Collection: IntoIterator<Item=&Fragment<'a>>,
// but it keeps asking for lifetime annotations until it does not work anyway while this works.
pub fn render_pattern<'a, Collection>(collection: &'a Collection, width: i32, out: &mut Formatter) -> fmt::Result
    where &'a Collection: IntoIterator<Item=&'a Fragment<'a>>
{
    fn classify(f: &Fragment) -> (i32, i32, i32) {
        match f {
            &Fragment::Blank => (0, 0, 0),
            &Fragment::Codepoint(_) => (1, 0, 0),
            &Fragment::Literal(l, _) => (l, 0, 0),
            &Fragment::Padding(_) => (0, 1, 0),
            &Fragment::Displayed(l, _) => (l, 0, 0),
            &Fragment::Growing(l, _) => (l, 1, 0),
            &Fragment::Shrinking(l, _) => (l, 0, 1),
        }
    }

    let (mut adjust, mut pads, mut shrinks) = collection.into_iter().map(classify).fold(
        (width, 0, 0), |(w, p0, s0), (n, p, s)| (w - n, p0 + p, s0 + s));

    for f in collection.into_iter() {
        match f {
            &Fragment::Blank => {}
            &Fragment::Codepoint(c) => {
                try!(out.write_fmt(format_args!("{}", c)));
            },
            &Fragment::Literal(_, s) => {
                try!(out.write_str(s));
            }
            &Fragment::Padding(s) => {
                if adjust > 0 {
                    let n = adjust / pads;
                    for _ in 0..n {
                        try!(out.write_str(s));
                    }
                    pads -= 1;
                    adjust -= n;
                }
            },
            &Fragment::Displayed(_, d) => {
                try!(d.fmt(out));
            },
            &Fragment::Growing(l, r) => {
                let m = if adjust > 0 {
                    let n = adjust / pads;
                    pads -= 1;
                    adjust -= n;
                    n
                } else { 0 };
                try!(r.fmt(l + m, l, out));
            },
            &Fragment::Shrinking(l, r) => {
                let m = if adjust < 0 {
                    let n = adjust / shrinks;
                    shrinks -= 1;
                    adjust -= n;
                    n
                } else { 0 };
                try!(r.fmt(l + m, l, out));
            },
        }
    };
    return Ok(());
}

#[cfg(test)]
mod test {
    use std::fmt;
    use std::fmt::{Display,Formatter};
    use super::{Fragment,Render,render_pattern};

    struct Disp<F>(F) where F: Fn(&mut Formatter) -> fmt::Result;

    impl<F> Display for Disp<F> where F: Fn(&mut Formatter) -> fmt::Result {
        fn fmt(&self, out: &mut Formatter) -> fmt::Result { self.0(out) }
    }

    fn disp<F>(f: F) -> String where F: Fn(&mut Formatter) -> fmt::Result {
        Disp(f).to_string()
    }

    struct Filler;

    impl Render for Filler {
        fn fmt(&self, width: i32, original: i32, out: &mut Formatter) -> fmt::Result {
            assert_eq!(2, original);
            for _ in 0..width {
                try!(out.write_str("·"));
            }
            Ok(())
        }
    }

    #[test]
    fn render_blank() {
        assert_eq!("", disp(|o| render_pattern(&[Fragment::Blank], 2, o)));
    }

    #[test]
    fn render_codepoint() {
        assert_eq!("ω", disp(|o| render_pattern(&[Fragment::Codepoint('ω')], 2, o)));
    }

    #[test]
    fn render_literal() {
        assert_eq!("f̌", disp(|o| render_pattern(&[Fragment::Literal(1, "f̌")], 2, o)));
    }

    #[test]
    fn render_padding() {
        assert_eq!("~~~", disp(|o| render_pattern(&[Fragment::Padding("~")], 3, o)));
    }

    #[test]
    fn render_displayed() {
        assert_eq!("x̌", disp(|o| render_pattern(&[Fragment::Displayed(1, &"x̌")], 2, o)));
    }

    #[test]
    fn render_growing() {
        assert_eq!("·····", disp(|o| render_pattern(&[Fragment::Growing(2, &Filler)], 5, o)));
        assert_eq!("··", disp(|o| render_pattern(&[Fragment::Growing(2, &Filler)], 1, o)));
    }

    #[test]
    fn render_shrinking() {
        assert_eq!("··", disp(|o| render_pattern(&[Fragment::Shrinking(2, &Filler)], 5, o)));
        assert_eq!("·", disp(|o| render_pattern(&[Fragment::Shrinking(2, &Filler)], 1, o)));
    }

    #[test]
    fn render_multiple() {
        assert_eq!("~foo~bar~~", disp(|o| render_pattern(
                    &[Fragment::Padding("~"), Fragment::Literal(3, "foo"), Fragment::Padding("~"),
                      Fragment::Literal(3, "bar"), Fragment::Padding("~")], 10, o)));
    }
}
