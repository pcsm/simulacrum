use handlebox::HandleBox;

use std::sync::Mutex;

use super::{ExpectationId, MethodName};
use super::expectation::{Constraint, Expectation, ExpectationT, ExpectationEra, ExpectationResult};
use super::user::{MethodSig, MethodTypes};

// A thread-safe store for `Box<ExpectationT>`s, including the order that they should be
// evaluated in (Eras).
pub(crate) struct ExpectationStore(Mutex<Inner>);

struct Inner {
    eras: Vec<ExpectationEra>,
    expectations: HandleBox<Box<ExpectationT>>
}

impl ExpectationStore {
    pub fn new() -> Self {
        let eras = vec![ExpectationEra::new()];
        ExpectationStore(Mutex::new(Inner {
            eras,
            expectations: HandleBox::new()
        }))
    }

    pub fn get_mut<I, O>(&self, id: ExpectationId) -> ExpectationEditor<I, O> {
        ExpectationEditor {
            id,
            store: &self,
            types: MethodTypes::new()
        }
    }

    pub fn matcher_for<I, O>(&self, name: MethodName) -> ExpectationMatcher<I, O> where
        I: 'static,
        O: 'static
    {
        let sig = MethodSig {
            name,
            types: MethodTypes::new()
        };
        let mut matcher = ExpectationMatcher {
            ids: Vec::new(),
            sig,
            store: &self
        };

        // Gather up ids for expectations that match this one
        // self.get_mut(self.top_group).find_matches()
        // for (id, exp) in self.expectations.lock().unwrap().internal_map().iter() {

        // }
        matcher
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
        inner.eras.last_mut().unwrap().add(id);

        id
    }

    /// Verify all expectations in this store.
    pub fn verify(&self) -> ExpectationResult {
        unimplemented!()
    }

    /// (Testing) Get the number of total Expectations in the store.
    fn exp_count(&self) -> usize {
        self.0.lock().unwrap().expectations.internal_map().len()
    }

    /// (Testing) Get the number of total Eras in the store.
    fn era_count(&self) -> usize {
        self.0.lock().unwrap().eras.len()
    }
}

// Used internally to mutably access an `ExpectationStore`.
pub struct ExpectationEditor<'a, I, O> {
    id: ExpectationId,
    store: &'a ExpectationStore,
    types: MethodTypes<I, O>
}

impl<'a, I, O> ExpectationEditor<'a, I, O> where
    I: 'static,
    O: 'static
{
    pub(crate) fn constrain(&self, constraint: Constraint) {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().constrain(constraint);
    }

    pub(crate) fn set_return<F>(&mut self, return_behavior: F) where
        F: 'static + FnMut(I) -> O
    {
        self.store.0.lock().unwrap().expectations.get_mut(&self.id).unwrap().as_any().downcast_mut::<Expectation<I, O>>().unwrap().set_return(return_behavior);
    }

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

impl<'a, I, O> ExpectationMatcher<'a, I, O> {
    /// Validate params with param verifier closure the Mock user provided with `TrackedMethod.with()`.
    pub fn with(self, params: I) -> Self {
        // TODO: Validate params with param verifier fn
        unimplemented!()
    }

    /// Return the result of the closure the Mock user provided with `TrackedMethod.returning()`.
    pub fn returning(self) -> O {
        // TODO: Call returning behavior and return the result
        unimplemented!()
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = ExpectationStore::new();

        assert_eq!(s.era_count(), 1, "Number of Eras");
        assert_eq!(s.exp_count(), 0, "Number of Expectations");
    }

    #[test]
    fn test_add() {
        let s = ExpectationStore::new();
        let e: Expectation<(), ()> = Expectation::new("foo");

        s.add(e);

        assert_eq!(s.era_count(), 1, "Number of Eras");
        assert_eq!(s.exp_count(), 1, "Number of Expectations");
    }
}