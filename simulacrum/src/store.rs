use handlebox::HandleBox;

use std::sync::Mutex;

use super::{ExpectationId, MethodName};
use super::expectation::{Constraint, Expectation, ExpectationT, ExpectationResult};
use super::user::{MethodSig, MethodTypes};

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

    pub fn matcher_for<I, O>(&self, name: MethodName) -> ExpectationMatcher<I, O> where
        I: 'static,
        O: 'static
    {
        let sig = MethodSig {
            name,
            _types: MethodTypes::new()
        };

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
                _sig: sig,
                store: &self
            }
        } else {
            ExpectationMatcher {
                ids: Vec::new(),
                _sig: sig,
                store: &self
            }
        }
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
        // Lock our inner mutex
        let mut inner = self.0.lock().unwrap();

        // If all of our Eras are verfied, we're good to go!
        if inner.current_unverified_era >= inner.eras.len() {
            return Ok(());
        }

        let mut current_unverified_era = inner.current_unverified_era;
        let mut status = Ok(());

        'eras: for era_index in inner.current_unverified_era .. inner.eras.len() {
            let era = &inner.eras[era_index];
            for id in era.iter() {
                let expectation = inner.expectations.get(id).unwrap();
                let r = expectation.verify();
                if r.is_err() {
                    // Note the current Era index
                    current_unverified_era = era_index;
                    // Note the error in this Era
                    status = r;
                    // Stop processing Eras since this one is still incomplete
                    break 'eras;
                }
            }
        }

        inner.current_unverified_era = current_unverified_era;

        status
    }

    /// (For testing) Get the number of total Expectations in the store.
    #[allow(dead_code)]
    fn exp_count(&self) -> usize {
        self.0.lock().unwrap().expectations.internal_map().len()
    }

    /// (For testing) Get the number of total Eras in the store.
    #[allow(dead_code)]
    fn era_count(&self) -> usize {
        self.0.lock().unwrap().eras.len()
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

    pub(crate) fn set_return<F>(&mut self, return_behavior: F) where
        F: 'static + FnMut(&I) -> O
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
    _sig: MethodSig<I, O>,
    store: &'a ExpectationStore
}

impl<'a, I, O> ExpectationMatcher<'a, I, O> where
    I: 'static,
    O: 'static
{
    /// Tell each matched Expectation that this method was called.
    pub fn was_called(self, params: I) -> Self {
        for id in self.ids.iter() {
            self.store.0.lock().unwrap().expectations.get_mut(&id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().handle_call(&params);
        }
        self
    }

    /// Return the result of the closure the user provided with `TrackedMethod.returning()`.
    pub fn returning(self) -> O {
        // TODO: Call returning behavior and return the result
        unimplemented!()
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
        e.constrain(AlwaysPass);
        s.add(e);

        s.new_era();

        // Add the same method twice! It doesn't make sense in practice, but we want
        // to only return one Id, signifying that the first Era was matched against.
        let mut e: Expectation<(), ()> = Expectation::new("frob");
        e.constrain(AlwaysPass);
        s.add(e);
        let mut e: Expectation<(), ()> = Expectation::new("frob");
        e.constrain(AlwaysPass);
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