#![allow(dead_code)]

use regex::Regex;
use std::fmt;

const NOT_WELL_FORMED: &'static str = "Language tag is not well-formed.";
const COULD_NOT_CANONICALIZE: &'static str = "Could not canonicalize language tag.";

static LANGUAGE_TAG_REGEX: Regex = regex!(r"(?ix)
^
(?P<language>[:alpha:]{2,3}(-(?P<extlang>[:alpha:]{3}(-[:alpha:]{3}){0,2}))?)
(?P<script>-[:alpha:]{4})?
(?P<region>-([:alpha:]{2}|[:digit:]{3}))?
(?P<variant>-([:alnum:]{5,8}|[:digit:][:alnum:]{3}))*
(?P<extension>-(?P<singleton>[0-9A-WY-Za-wy-z])(-[:alnum:]{2,8})+)*
(?P<privateuse>-x(-[:alnum:]{1,8})+)?
$
");

#[derive(Debug)]
pub struct LanguageTag<'a> {
    language: &'a str
}

impl<'a> LanguageTag<'a> {
    pub fn new(lt: &'a str) -> Result<LanguageTag, &'static str> {
        if Self::is_well_formed(lt) {
            Ok(Self::canonicalize(lt))
        } else {
            Err(NOT_WELL_FORMED)
        }
    }

    fn is_well_formed(lt: &'a str) -> bool {
        return LANGUAGE_TAG_REGEX.is_match(lt)
    }

    fn canonicalize(lt: &'a str) -> LanguageTag {
        LanguageTag {
            language: lt
        }
    }
}

impl<'a> fmt::Display for LanguageTag<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.language)
    }
}

#[cfg(test)]
mod test {
    use super::LanguageTag;

    #[test]
    fn simple_valid_lang_tags() {
        assert_eq!(LanguageTag::new("en-US").unwrap().to_string(), String::from("en-US"));
        assert_eq!(LanguageTag::new("EN-US").unwrap().to_string(), String::from("EN-US"));
        assert_eq!(LanguageTag::new("en").unwrap().to_string(), String::from("en"));
        assert_eq!(LanguageTag::new("eng-Latn-840").unwrap().to_string(), String::from("eng-Latn-840"));
    }

    #[test]
    fn complex_valid_lang_tags() {
        assert_eq!(LanguageTag::new("de-DE-u-email-co-phonebk-x-linux").unwrap().to_string(),
                   String::from("de-DE-u-email-co-phonebk-x-linux"));
        assert_eq!(LanguageTag::new("vi-vn-u-fw-mon-hc-h24-ms-metric").unwrap().to_string(),
                   String::from("vi-vn-u-fw-mon-hc-h24-ms-metric"));
        assert_eq!(LanguageTag::new("sl-Cyrl-YU-rozaj-solba-1994-b-1234-a-Foobar-x-b-1234-a-Foobar").unwrap().to_string(),
                   String::from("sl-Cyrl-YU-rozaj-solba-1994-b-1234-a-Foobar-x-b-1234-a-Foobar"));
    }

    #[test]
    #[should_panic]
    fn invalid_lang_tag() {
        LanguageTag::new("english").unwrap();
    }

    #[test]
    #[should_panic]
    fn subtly_invalid_lang_tag() {
        LanguageTag::new("de-DE-u-email-co-phonebook-x-linux").unwrap();
    }

    #[test]
    #[should_panic]
    fn incomplete_extension_lang_tag() {
        LanguageTag::new("de-DE-u-email-co-phonebk-x").unwrap();
    }
}