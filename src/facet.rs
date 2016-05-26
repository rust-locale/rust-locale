//! Infrastructure for defining facets.
//!
//! Facet is an object that defines data and logic required for some aspect (category) of
//! localization. For example `Numeric` facet deals with formatting and parsing numbers, `Time` facet
//! deals with formatting and parsing dates and times, `Messages` deals with translated messages
//! and so on.
//!
//! To use facets, all you need is the convenience interface on `locale::Locale`. If that's what
//! you need, you can stop reading now.
//!
//! But if you want to define your own facets, this module provides common infrastructure for
//! caching facet instances, defining a default implementations and default factory functions to
//! construct them for recognized locale and making sure a fall-back "invariant" implementation is
//! available if nothing better can be obtained.

use std::any::{Any,TypeId};
use std::collections::HashMap;
use std::sync::{Arc,RwLock};
use super::{LanguageRange,Locale};

/// Get instance of facet for given Locale.
///
/// The instance is constructed if needed, and cached for reuse. The function always succeeds. If
/// the factory in use does not understand any language tag of the provided locale, the invariant
/// implementation is returned.
///
/// For convenience, this function is also wrapped as Locale.get().
pub fn get<'a, F: Any + Send + Sync + ?Sized>(l: &Locale) -> Arc<F>
    where Builder: Factory<F>
{
    if let Some(f) = CACHE.get_by_locale(l) {
        return f;
    }

    for t in l.tags_for(Builder::category()) {
        let f = {
            if let Some(f) = CACHE.get_by_tag(&t) {
                f
            } else if let Some(f) = Builder::new_for(&t) {
                CACHE.set_by_tag(&t, f)
            } else {
                continue
            }
        };
        return CACHE.set_by_locale(l, f);
    }
    // cache even this, so we don't need to iterate the tags next time for this locale
    return CACHE.set_by_locale(l, Builder::new_invariant());
}

/// Trait defining how a facet is to be instantiated by default.
///
/// To define a facet, this trait, specialized to the facet trait, must be implemented for the
/// Builder tag type.
pub trait Factory<F: Any + Send + Sync + ?Sized> {
    /// Name of category this trait implements.
    ///
    /// This should be an associated constant, but those are not stable yet.
    fn category() -> &'static str;

    /// Construct facet for given `LanguageRange`
    ///
    /// If the tag is not understood or no data are available for it, this functions should return
    /// None. The `get` function will then take care of trying the fallback tags and, if that fails
    /// too, uses `new_invariant()` to get invariant implementation.
    fn new_for<'a>(&LanguageRange<'a>) -> Option<Arc<F>>;

    /// Construct facet for invariant locale.
    ///
    /// Every facet *must* have an invariant implementation. That will be used as last resort if
    /// there is no other implementation for any of the users locales (or if user requests the
    /// invariant locale).
    fn new_invariant() -> Arc<F>;
}

/// Tag type on which the Factory is implemented.
///
/// We need to specialize factory functions for each facet. Since Rust does not have
/// explicit specializations, we emulate them by `impl`ementing for this tag type.
pub struct Builder;

type FacetBox = Box<Any + Send + Sync>;
type FacetMap = HashMap<TypeId, FacetBox>;

struct Cache {
    by_locale: RwLock<HashMap<Locale, FacetMap>>,
    by_tag: RwLock<HashMap<LanguageRange<'static>, FacetMap>>,
}

impl Cache {
    fn get_by_locale<F: Any + Send + Sync + ?Sized>(&self, l: &Locale) -> Option<Arc<F>> {
        let map = self.by_locale.read().unwrap();
        if let Some(row) = map.get(l) {
            if let Some(box_) = row.get(&TypeId::of::<F>()) {
                // it is under that TypeId, so it should be correct type
                // note: we are hitting some limit of type inference without the annotation
                let a: &Arc<F> = (box_.as_ref() as &Any).downcast_ref().unwrap();
                return Some(a.clone())
            }
        }
        return None;
    }

    fn get_by_tag<'a, F: Any + Send + Sync + ?Sized>(&self, t: &LanguageRange<'a>) -> Option<Arc<F>> {
        let map = self.by_tag.read().unwrap();
        if let Some(row) = map.get(t) {
            if let Some(box_) = row.get(&TypeId::of::<F>()) {
                // it is under that TypeId, so it should be correct type
                // note: we are hitting some limit of type inference without the annotation
                let a: &Arc<F> = (box_.as_ref() as &Any).downcast_ref().unwrap();
                return Some(a.clone())
            }
        }
        return None;
    }

    fn set_by_locale<F: Any + Send + Sync + ?Sized>(&self, l: &Locale, a: Arc<F>) -> Arc<F> {
        let mut map = self.by_locale.write().unwrap();
        if !map.contains_key(l) {
            map.insert(l.clone(), FacetMap::new());
        }
        // we've inserted it, so it must be there
        let row = map.get_mut(l).unwrap();

        if !row.contains_key(&TypeId::of::<F>()) {
            row.insert(TypeId::of::<F>(), Box::new(a) as FacetBox);
        }
        // again, we've inserted it, so it must be there and correct type
        let a: &Arc<F> = (row.get(&TypeId::of::<F>()).unwrap().as_ref() as &Any).downcast_ref().unwrap();
        return a.clone();
    }

    fn set_by_tag<'a, F: Any + Send + Sync + ?Sized>(&self, t: &LanguageRange<'a>, a: Arc<F>) -> Arc<F> {
        // XXX: If I try to pass t to the get_mut, the compiler fails to deduce lifetimes.
        let t2 = t.clone().into_static();
        let mut map = self.by_tag.write().unwrap();
        if !map.contains_key(t) {
            map.insert(t2.clone(), FacetMap::new());
        }
        // we've inserted it, so it must be there
        let row = map.get_mut(&t2).unwrap();

        if !row.contains_key(&TypeId::of::<F>()) {
            row.insert(TypeId::of::<F>(), Box::new(a) as FacetBox);
        }
        // again, we've inserted it, so it must be there and correct type
        let a: &Arc<F> = (row.get(&TypeId::of::<F>()).unwrap().as_ref() as &Any).downcast_ref().unwrap();
        return a.clone();
    }
}

lazy_static! {
    static ref CACHE: Cache = Cache {
        by_locale: RwLock::new(HashMap::new()),
        by_tag: RwLock::new(HashMap::new()),
    };
}

// ---- tests ----

#[cfg(test)]
mod test {
    use std::any::Any;
    use std::sync::Arc;
    use super::{get,Builder,Factory};
    use super::super::{LanguageRange,Locale};

    trait Test : Any + Send + Sync {
        fn tag(&self) -> &LanguageRange<'static>;
        fn ptr(&self) -> usize;
    }

    struct TestImpl {
        t: LanguageRange<'static>,
    }

    impl Test for TestImpl {
        fn tag(&self) -> &LanguageRange<'static> { &self.t }
        fn ptr(&self) -> usize {
            self as *const TestImpl as usize
        }
    }

    impl Factory<Test> for Builder {
        fn category() -> &'static str { "test" }

        fn new_for<'a>(t: &LanguageRange<'a>) -> Option<Arc<Test>> {
            if t.as_ref() > "b" {
                Some(Arc::new(TestImpl { t: t.clone().into_static() }))
            } else {
                None
            }
        }

        fn new_invariant() -> Arc<Test> {
            Arc::new(TestImpl { t: LanguageRange::invariant() })
        }
    }

    #[test]
    fn basic() {
        let f = get::<Test>(&Locale::new("en").unwrap());
        assert_eq!(&LanguageRange::new("en").unwrap(), f.tag());
    }

    #[test]
    fn multiple() {
        let f1 = get::<Test>(&Locale::new("en").unwrap());
        let f2 = get::<Test>(&Locale::new("cs-CZ").unwrap());
        assert_eq!(&LanguageRange::new("cs-CZ").unwrap(), f2.tag());
        assert!(f1.ptr() != f2.ptr());
    }

    #[test]
    fn cache() {
        let f1 = get::<Test>(&Locale::new("en").unwrap());
        let f2 = get::<Test>(&Locale::new("en").unwrap());
        assert_eq!(f1.ptr(), f2.ptr());
    }

    #[test]
    fn default() {
        let f = get::<Test>(&Locale::new("ar").unwrap());
        assert_eq!(&LanguageRange::invariant(), f.tag());
    }

    #[test]
    fn fallback() {
        let f = get::<Test>(&Locale::new("ar-SA,de-AT,en-US").unwrap());
        assert_eq!(&LanguageRange::new("de-AT").unwrap(), f.tag());
    }

    #[test]
    fn category() {
        let f = get::<Test>(&Locale::new("en-US,test=cs-CZ").unwrap());
        assert_eq!(&LanguageRange::new("cs-CZ").unwrap(), f.tag());
    }
}
