use std::cmp::Ord;
use std::collections::VecDeque;
use super::{Data,Item};
use ::LanguageRange;

#[derive(Debug)]
pub struct CldrData {
    parents: &'static [&'static CldrData],
    index: &'static [(Item, u16)],
    data: &'static str,
}

#[derive(Debug)]
pub struct CldrTree {
    data: &'static CldrData,
    children: &'static [(&'static str, &'static CldrTree)],
}

impl Data for &'static CldrData {
    fn get(&self, item: Item) -> &str {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        while !queue.is_empty() {
            let data = queue.pop_front().unwrap();
            if let Ok(i) = data.index.binary_search_by(|&(k, _)| k.cmp(&item)) {
                let begin = if i > 0 { data.index[i - 1].1 } else { 0 } as usize;
                let end = data.index[i].1 as usize;
                return &data.data[begin..end];
            }
            queue.extend(data.parents);
        }
        return "";
    }
}

mod data;

pub fn new_for<'a>(t: &LanguageRange<'a>) -> Option<&'static CldrData> {
    let mut tags = t.as_ref().split('-');
    let mut tree = &data::TAG_ROOT;
    let mut tag = tags.next();
    while !tag.is_none() {
        if let Ok(i) = tree.children.binary_search_by(|&(k, _)| k.cmp(tag.unwrap())) {
            tree = tree.children[i].1;
        } else {
            break;
        }
        tag = tags.next();
    }
    // Don't return invariant from here, because there may be fallback at facet factory level.
    return if tree as *const _ != &data::TAG_ROOT as *const _ { Some(tree.data) } else { None };
}

pub fn new_invariant() -> &'static CldrData { &data::LOC_ROOT }

#[cfg(test)]
mod test {
    use ::data::{Data,Item};
    use ::Locale;

    fn check_item(lang: &str, item: Item, exp: &str) {
        let loc = Locale::new(lang).unwrap();
        let fac = ::facet::get::<Data>(&loc);
        let act = fac.get(item);
        assert_eq!(exp, act);
    }

    #[test]
    fn invariant_values() {
        check_item("", Item::DecimalDigits, "0123456789");
        check_item("", Item::DecimalSeparator, ".");
        // FIXME: Invariant should *not* use , as GS. Because it is *invariant*. It should strive
        // to be as obvious to non-Americans and non-British as possible and , is pretty common
        // *decimal* separator too. Problem is that when I override the value in invariant, I have
        // to force it into the children.
        check_item("", Item::GroupSeparator, ",");
        check_item("", Item::Grouping, "3");
        check_item("", Item::FractionalGrouping, "");
        check_item("", Item::MinGroupingDigits, "1");
        check_item("", Item::MinIntegralDigits, "1");
    }

    #[test]
    fn various_values() {
        check_item("cs", Item::DecimalSeparator, ",");
        check_item("cs", Item::GroupSeparator, "\u{a0}");
        check_item("de", Item::DecimalSeparator, ",");
        check_item("de", Item::GroupSeparator, ".");
        check_item("de-CH", Item::DecimalSeparator, ".");
        check_item("de-CH", Item::GroupSeparator, "’");
        check_item("my", Item::DecimalDigits, "၀၁၂၃၄၅၆၇၈၉");
        check_item("en-IN", Item::Grouping, "2;3");
        check_item("pl", Item::MinGroupingDigits, "2");
    }
}
