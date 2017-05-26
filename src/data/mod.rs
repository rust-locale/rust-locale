//! Access to data that are used in locale-aware formatting.

use std::any::Any;
use std::sync::Arc;
use super::facet::{Builder,Factory};
use super::LanguageRange;

mod cldr;

use ::time::Calendar;
use ::time::DayPeriod as DayPeriodType;
use ::time::FormatWidth as Width;

// TODO: Move to Time facet/module; or some general formats module
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash,PartialOrd,Ord)]
pub enum Length {
    Short,
    Medium,
    Long,
    Full,
}

/// Items provided by the data facet.
///
/// Only items used by the standard implementation are supported. Supporting additional items does
/// not make much sense because the standard formatters wouldn't know how to use them and extension
/// formatters would not find the data here so they have to provide them themselves anyway.
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash,PartialOrd,Ord)]
pub enum Item {
    // Numeric
    DecimalDigits,
    DecimalSeparator,
    GroupSeparator,
    PlusSign,
    MinusSign,
    PercentSign,
    PerMilleSign,
    EngineeringExponent,
    CommonExponent,
    InfinitySymbol,
    NotANumberSymbol,
    Grouping,
    FractionalGrouping,
    MinGroupingDigits,
    MinIntegralDigits,
    // Date&Time
    Month(Width, Calendar, u8),
    Day(Width, Calendar, u8),
    Quarter(Width, Calendar, u8),
    DayPeriod(Width, Calendar, DayPeriodType),
    EraAbbr(Calendar, u8),
    EraWide(Calendar, u8),
    EraNarrow(Calendar, u8),
    DateFormat(Length, Calendar),
    TimeFormat(Length, Calendar),
    DateTimeFormat(Length, Calendar),
    // FIXME CONTINUE...
}

/// Facet for accessing locale data.
pub trait Data : Any + Send + Sync {
    fn get(&self, item: Item) -> &str;
}

impl Factory<Data> for Builder {
    fn category() -> &'static str { "data" } // but we generally ask by specific tag anyway

    fn new_for<'a>(t: &LanguageRange<'a>) -> Option<Arc<Data>> {
        cldr::new_for(t).map(|d| Arc::new(d) as Arc<Data>)
    }

    fn new_invariant() -> Arc<Data> {
        Arc::new(cldr::new_invariant())
    }
}
