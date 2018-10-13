use handlebox::HandleBox;

use std::cell::RefCell;
use std::sync::Mutex;

use super::{ExpectationId, MethodName};
use super::expectation::{Constraint, Expectation, ExpectationT, ExpectationResult};
use super::method::{MethodSig, MethodTypes};

// A thread-safe store for `Box<ExpectationT>`s, including the order that they should be
// evaluated in (Eras).
pub(crate) struct ExpectationStore(Mutex<Inner>);

struct Inner {
    current_unverified_era: usize,
    eras: Vec<Era>,
    expectations: HandleBox<Box<ExpectationT>>
}

type Era = Vec<ExpectationId>;

impl ExpectationStore {
    pub fn new() -> Self {
        let eras = vec![Era::new()];
        ExpectationStore(Mutex::new(Inner {
            current_unverified_era: 0,
            eras,
            expectations: HandleBox::new()
        }))
    }

    pub fn get_mut<I, O>(&self, id: ExpectationId) -> ExpectationEditor<I, O> {
        ExpectationEditor {
            id,
            store: &self,
            _types: MethodTypes::new()
        }
    }

    pub(crate) fn has_expectation_for_method_in_current_era(&self, name: &MethodName) -> bool {
        // If the current era is complete, move on to the next incomplete one.
        self.advance_era();

        // Lock our inner mutex
        let inner = self.0.lock().unwrap();

        // Get the current era and see if there's an expectation with this name in it 
        for id in inner.eras.last().unwrap() {
            if inner.expectations.get(&id).unwrap().name() == name {
                return true;
            }
        }

        false
    }

    pub fn matcher_for<I, O>(&self, name: &str) -> ExpectationMatcher<I, O> where
        I: 'static,
        O: 'static
    {
        let sig = MethodSig {
            name: name.to_string(),
            _types: MethodTypes::new()
        };

        // If the current era is complete, move on to the next incomplete one.
        self.advance_era();

        // Lock our inner mutex
        let inner = self.0.lock().unwrap();

        // Only return ids if we have unverified Eras remaining
        if inner.current_unverified_era < inner.eras.len() {
            // Gather up ids for expectations that match this one in the current Era
            let mut ids = inner.eras.get(inner.current_unverified_era).unwrap().clone();
            ids.retain(|&id| {
                inner.expectations.get(&id).unwrap().name() == name
            });

            ExpectationMatcher {
                ids,
                sig,
                store: &self
            }
        } else {
            ExpectationMatcher {
                ids: Vec::new(),
                sig,
                store: &self
            }
        }
    }

    // If the current era is complete, move on to the next incomplete one
    #[allow(unused_must_use)]
    pub fn advance_era(&self) {
        // We don't care about the result, we're just doing it to advance the era if necessary
        self.verify();
    }

    // Add a new Expectation under the current Era and return its id.
    pub fn add<E>(&self, expectation: E) -> ExpectationId where
        E: ExpectationT + 'static
    {
        // Lock our inner mutex
        let mut inner = self.0.lock().unwrap();
        
        // Add a new expectation
        let id = inner.expectations.add(Box::new(expectation));

        // Add that new expectation to the current Era
        inner.eras.last_mut().unwrap().push(id);

        id
    }

    // Begin a new Era and make it the current one.
    pub fn new_era(&self) {
        // Lock our inner mutex
        let mut inner = self.0.lock().unwrap();

        inner.eras.push(Vec::new());
    }

    /// Verify all expectations in this store.
    pub fn verify(&self) -> ExpectationResult {
        let mut status = Ok(());

        // Lock our inner mutex
        let mut inner = self.0.lock().unwrap();

        let original_unverified_era = inner.current_unverified_era;

        'eras: for era_index in original_unverified_era .. inner.eras.len() {
            // Update our current unverified era to be the current Era index
            inner.current_unverified_era = era_index;

            // If we have any not-yet-verified Expectations in this Era, do not
            // mark it as complete
            let era = &inner.eras[era_index];
            for id in era.iter() {
                let expectation = inner.expectations.get(id).unwrap();
                let r = expectation.verify();
                if r.is_err() {
                    // Note the error in this Era
                    status = r;
                    // Stop processing Eras since this one is still incomplete
                    break 'eras;
                }
            }
        }

        status
    }

    /// (For testing) Get the number of total Expectations in the store.
    #[allow(dead_code)]
    fn exp_count(&self) -> usize {
        self.0.lock().unwrap().expectations.map.len()
    }

    /// (For testing) Get the number of total Eras in the store.
    #[allow(dead_code)]
    fn era_count(&self) -> usize {
        self.0.lock().unwrap().eras.len()
    }
}

impl Default for ExpectationStore {
    fn default() -> Self {
        Self::new()
    }
}

// Used internally to mutably access an `ExpectationStore`.
pub struct ExpectationEditor<'a, I, O> {
    id: ExpectationId,
    store: &'a ExpectationStore,
    _types: MethodTypes<I, O>
}

impl<'a, I, O> ExpectationEditor<'a, I, O> where
    I: 'static,
    O: 'static
{
    pub(crate) fn constrain<C>(&self, constraint: C) where
        C: Constraint<I> + 'static
    {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().constrain(constraint);
    }

    pub(crate) fn set_modification<F>(&mut self, modification_behavior: F) where
        F: 'static + FnMut(&mut I)
    {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().set_modification(modification_behavior);
    }

    pub(crate) fn set_return<F>(&mut self, return_behavior: F) where
        F: 'static + FnMut(I) -> O
    {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().set_return(return_behavior);
    }

    #[allow(dead_code)]
    fn verify(&self) -> ExpectationResult {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().verify()
    }
}

// Used internally to mutably access an `ExpectationStore` when trying to apply
// a method to a set of matched expectations.
//
// I is a tuple of args for this method excluding self.
// O is the return value or () if there is no return value.
pub(crate) struct ExpectationMatcher<'a, I, O> {
    ids: Vec<ExpectationId>,
    sig: MethodSig<I, O>,
    store: &'a ExpectationStore
}

impl<'a, I, O> ExpectationMatcher<'a, I, O> where
    I: 'static,
    O: 'static
{
    /// Tell each matched Expectation that this method was called.
    #[allow(unused_must_use)]
    pub fn was_called(self, params: I) -> Self {
        let cell = RefCell::new(params);
        for id in self.ids.iter() {
            self.store.0.lock().unwrap().expectations.get_mut(&id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().handle_call(&cell);
        }
        self
    }

    /// Same as `was_called()`, but returns a value as well.
    ///
    /// Returns the result of the closure the user provided with `TrackedMethod.returning()`.
    ///
    /// If multiple Expectations are matched, the last one matched is used.
    ///
    /// If no closure was specified or no expectations matched, this method panics.
    #[allow(unused_must_use)]
    pub fn was_called_returning(mut self, params: I) -> O {
        let cell = RefCell::new(params);
        if let Some(id) = self.ids.pop() {
            self.store.0.lock().unwrap().expectations.get_mut(&id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().handle_call(&cell);
            let result = self.store.0.lock().unwrap().expectations.get_mut(&id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().return_value_for(cell);
            result
        } else {
            panic!("Can't return a value for method `{}` with no matching expectations.", self.sig.name);
        }
    }

    // For Testing
    #[allow(dead_code)]
    fn id_count(&self) -> usize {
        self.ids.len()
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;
    use constraint::ConstraintError;
    use constraint::stock::always::{AlwaysFail, AlwaysPass};

    #[test]
    fn test_new() {
        let s = ExpectationStore::new();

        assert!(s.verify().is_ok(), "Store should be Ok after creation");
        assert_eq!(s.era_count(), 1, "Store should have one Era after creation");
        assert_eq!(s.exp_count(), 0, "Store should have no Expectations after creation");
    }

    #[test]
    fn test_add() {
        let s = ExpectationStore::new();
        let e: Expectation<(), ()> = Expectation::new("foo");

        s.add(e);

        assert_eq!(s.era_count(), 1, "Number of Eras");
        assert_eq!(s.exp_count(), 1, "Number of Expectations");
    }

    #[test]
    fn test_new_era() {
        let s = ExpectationStore::new();
        
        s.new_era();

        assert_eq!(s.era_count(), 2, "Number of Eras after creating a new one");
    }

    #[test]
    fn test_verify_no_expectations() {
        let s = ExpectationStore::new();

        let r = s.verify();

        assert!(r.is_ok());
    }

    #[test]
    fn test_verify_simple_pass() {
        let s = ExpectationStore::new();
        let mut e: Expectation<(), ()> = Expectation::new("squig");
        e.constrain(AlwaysPass);
        s.add(e);

        let r = s.verify();

        assert!(r.is_ok(), "Store should pass");
    }

    #[test]
    fn test_verify_simple_fail() {
        let s = ExpectationStore::new();
        let mut e: Expectation<(), ()> = Expectation::new("zooks");
        e.constrain(AlwaysFail);
        s.add(e);

        let r = s.verify();

        assert!(r.is_err(), "Store should fail");
        let r = r.unwrap_err();
        assert_eq!(r.method_name, "zooks", "Store error should have the correct method name");
        assert_eq!(r.constraint_err, ConstraintError::AlwaysFail, "Store error should contain the correct Constraint error");
    }

    #[test]
    fn test_match() {
        let s = ExpectationStore::new();
        let mut e: Expectation<(), ()> = Expectation::new("frolf");
        e.constrain(AlwaysPass);
        s.add(e);
        let mut e: Expectation<(), ()> = Expectation::new("star");
        e.constrain(AlwaysPass);
        s.add(e);

        let m = s.matcher_for::<(), ()>("star");

        assert_eq!(m.id_count(), 1, "Ids matched should be 1");
    }

    #[test]
    fn test_match_current_era() {
        let s = ExpectationStore::new();
        let mut e: Expectation<(), ()> = Expectation::new("frob");
        e.constrain(AlwaysFail);
        s.add(e);

        s.new_era();

        // Add the same method twice! It doesn't make sense in practice, but we want
        // to only return one Id, signifying that the first Era was matched against.
        let mut e: Expectation<(), ()> = Expectation::new("frob");
        e.constrain(AlwaysFail);
        s.add(e);
        let mut e: Expectation<(), ()> = Expectation::new("frob");
        e.constrain(AlwaysFail);
        s.add(e);

        let m = s.matcher_for::<(), ()>("frob");

        assert_eq!(m.id_count(), 1, "Ids matched should be 1");
    }

    #[test]
    fn test_match_current_era_passed() {
        let s = ExpectationStore::new();
        let mut e: Expectation<(), ()> = Expectation::new("buzz");
        e.constrain(AlwaysPass);
        s.add(e);

        // Cheat and bump up the current unverified era to the end
        s.0.lock().unwrap().current_unverified_era = 1; 

        let m = s.matcher_for::<(), ()>("buzz");

        assert_eq!(m.id_count(), 0, "Ids matched should be 0");
    }
}
