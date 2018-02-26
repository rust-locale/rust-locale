use time::TimeZone;
use std::time::SystemTime;
use std::sync::Arc;
use facet::{Builder, Factory};
use numeric::Numeric;
use time::Time;
use std::fmt;

// TODO: Using the legacy implementation. Replace with by-hand numeric (lying around on another branch).
impl Numeric for ::Numeric {
    fn format_int_to(&self, n: &fmt::Display, _: &str, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_fmt(format_args!("{}", self.format_int(n)))
    }
    fn format_float_to(
        &self,
        n: &::numeric::DispLowerExp,
        _: &str,
        out: &mut fmt::Formatter,
    ) -> fmt::Result {
        out.write_fmt(format_args!("{}", self.format_float(n, 3)))
    }
}

impl Factory<Numeric> for Builder {
    fn category() -> &'static str {
        "numeric"
    }
    fn new_for<'a>(_: &::LanguageRange<'a>) -> Option<Arc<Numeric>> {
        None
    }
    fn new_invariant() -> Arc<Numeric> {
        Arc::<::Numeric>::new(::Numeric::english())
    }
}

// TODO: Very dumb dummy neutral implementation. Replace with, well, something.
impl Time for ::Time {
    fn format_datetime_to(
        &self,
        time: SystemTime,
        _zone: TimeZone,
        _fmt: &str,
        out: &mut fmt::Formatter,
    ) -> fmt::Result {
        out.write_fmt(format_args!("{:?}", time))
    }
}
